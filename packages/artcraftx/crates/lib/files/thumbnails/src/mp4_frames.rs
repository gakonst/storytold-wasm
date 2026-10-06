//! Decode the leading frames of an H.264 MP4 into RGB images.
//!
//! `mp4` demuxes the container; `openh264` (vendored Cisco decoder) decodes
//! the access units. Sample payloads are length-prefixed NAL units, so each
//! is rewritten to Annex-B (start codes) with the track's SPS/PPS fed first.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use images::image::RgbImage;
use mp4::{MediaType, Mp4Reader, TrackType};
use openh264::decoder::{Decoder, DecoderConfig, Flush};
use openh264::OpenH264API;

use crate::error::ThumbnailError;

const ANNEX_B_START_CODE: [u8; 4] = [0, 0, 0, 1];

/// One decoded frame with its presentation time.
pub struct DecodedFrame {
  pub pts_ms: u32,
  pub image: RgbImage,
}

/// Decode frames from the start of the file until `max_duration_ms` of
/// presentation time or `max_frames` frames, whichever comes first. Frames
/// are returned sorted by presentation time.
///
/// Blocking CPU work — call from `spawn_blocking` in async contexts.
pub fn decode_mp4_leading_frames<P: AsRef<Path>>(
  path: P,
  max_duration_ms: u32,
  max_frames: usize,
) -> Result<Vec<DecodedFrame>, ThumbnailError> {
  let file = File::open(path.as_ref())?;
  let size = file.metadata()?.len();
  let mut mp4 = Mp4Reader::read_header(BufReader::new(file), size)
      .map_err(|err| ThumbnailError::Unsupported(format!("not a readable MP4: {}", err)))?;

  let (track_id, timescale, nal_length_size, sps, pps) = {
    let track = mp4.tracks().values()
        .find(|track| {
          track.track_type().is_ok_and(|kind| kind == TrackType::Video)
              && track.media_type().is_ok_and(|media| media == MediaType::H264)
        })
        .ok_or_else(|| ThumbnailError::Unsupported("no H.264 video track".to_string()))?;

    let sps = track.sequence_parameter_set()
        .map_err(|err| ThumbnailError::Unsupported(format!("missing SPS: {}", err)))?
        .to_vec();
    let pps = track.picture_parameter_set()
        .map_err(|err| ThumbnailError::Unsupported(format!("missing PPS: {}", err)))?
        .to_vec();
    (track.track_id(), track.timescale(), nal_length_size_for(track), sps, pps)
  };

  // NB: openh264-rs flushes the decoder after every call by default, which
  // wrecks B-frame reordering: streams with B-frames (every provider's
  // High-profile output) start erroring a handful of samples in and never
  // recover. Let the decoder keep its reorder buffer and drain it at the end.
  let config = DecoderConfig::new().flush_after_decode(Flush::NoFlush);
  let mut decoder = Decoder::with_api_config(OpenH264API::from_source(), config)
      .map_err(|err| ThumbnailError::Decode(format!("could not create H.264 decoder: {}", err)))?;

  // Prime the decoder with the parameter sets.
  let mut parameter_sets = Vec::with_capacity(sps.len() + pps.len() + 8);
  append_annex_b(&mut parameter_sets, &sps);
  append_annex_b(&mut parameter_sets, &pps);
  let _ = decoder.decode(&parameter_sets);

  let mut frames: Vec<DecodedFrame> = Vec::new();
  // Presentation times of every sample fed so far. The decoder emits frames
  // in presentation order (that's what its reorder buffer is for), so the
  // k-th output frame gets the k-th smallest pts fed — exact regardless of
  // B-frame depth.
  let mut fed_pts_ms: Vec<u32> = Vec::new();
  let sample_count = mp4.sample_count(track_id)
      .map_err(|err| ThumbnailError::Decode(format!("could not count samples: {}", err)))?;

  for sample_id in 1..=sample_count {
    if frames.len() >= max_frames {
      break;
    }

    let sample = match mp4.read_sample(track_id, sample_id) {
      Ok(Some(sample)) => sample,
      Ok(None) => break,
      Err(err) => return Err(ThumbnailError::Decode(format!("could not read sample {}: {}", sample_id, err))),
    };

    // Presentation time: decode time + composition offset.
    let pts_units = sample.start_time as i64 + sample.rendering_offset as i64;
    let pts_ms = ((pts_units.max(0) as u128 * 1000) / timescale.max(1) as u128) as u32;

    let annex_b = length_prefixed_to_annex_b(&sample.bytes, nal_length_size)?;
    let insert_at = fed_pts_ms.partition_point(|&fed| fed <= pts_ms);
    fed_pts_ms.insert(insert_at, pts_ms);

    match decoder.decode(&annex_b) {
      Ok(Some(yuv)) => {
        let output_pts_ms = fed_pts_ms[frames.len()];
        frames.push(DecodedFrame { pts_ms: output_pts_ms, image: yuv_to_rgb(&yuv)? });
      }
      Ok(None) => {} // Decoder delay; frame not ready yet.
      Err(err) => {
        // A corrupt tail sample shouldn't discard the frames we have.
        log::warn!("H.264 decode error at sample {}: {}", sample_id, err);
        break;
      }
    }

    // Samples arrive in decode order, so a B-frame with an earlier pts can
    // follow the sample that crossed the limit; stop once the frames we
    // actually have reach it.
    if frames.last().is_some_and(|frame| frame.pts_ms >= max_duration_ms) {
      break;
    }
  }

  // Frames still parked in the reorder buffer.
  if frames.len() < max_frames {
    match decoder.flush_remaining() {
      Ok(remaining) => {
        for yuv in remaining {
          if frames.len() >= max_frames || frames.len() >= fed_pts_ms.len() {
            break;
          }
          let output_pts_ms = fed_pts_ms[frames.len()];
          if output_pts_ms > max_duration_ms {
            break;
          }
          frames.push(DecodedFrame { pts_ms: output_pts_ms, image: yuv_to_rgb(&yuv)? });
        }
      }
      Err(err) => log::warn!("H.264 flush error: {}", err),
    }
  }

  if frames.is_empty() {
    return Err(ThumbnailError::Decode("no frames decoded".to_string()));
  }

  frames.sort_by_key(|frame| frame.pts_ms);
  Ok(frames)
}

/// The avcC NAL length-prefix size (1, 2, or 4 bytes; 4 is ubiquitous).
fn nal_length_size_for(track: &mp4::Mp4Track) -> usize {
  track.trak.mdia.minf.stbl.stsd.avc1.as_ref()
      .map(|avc1| (avc1.avcc.length_size_minus_one as usize & 0b11) + 1)
      .unwrap_or(4)
}

fn length_prefixed_to_annex_b(sample_bytes: &[u8], length_size: usize) -> Result<Vec<u8>, ThumbnailError> {
  let mut annex_b = Vec::with_capacity(sample_bytes.len() + 16);
  let mut offset = 0usize;
  while offset + length_size <= sample_bytes.len() {
    let mut nal_length = 0usize;
    for byte in &sample_bytes[offset..offset + length_size] {
      nal_length = (nal_length << 8) | *byte as usize;
    }
    offset += length_size;
    let end = offset.checked_add(nal_length)
        .filter(|end| *end <= sample_bytes.len())
        .ok_or_else(|| ThumbnailError::Decode("NAL length overruns sample".to_string()))?;
    append_annex_b(&mut annex_b, &sample_bytes[offset..end]);
    offset = end;
  }
  Ok(annex_b)
}

fn append_annex_b(buffer: &mut Vec<u8>, nal: &[u8]) {
  buffer.extend_from_slice(&ANNEX_B_START_CODE);
  buffer.extend_from_slice(nal);
}

fn yuv_to_rgb(yuv: &openh264::decoder::DecodedYUV<'_>) -> Result<RgbImage, ThumbnailError> {
  use openh264::formats::YUVSource;
  let (width, height) = yuv.dimensions();
  let mut rgb = vec![0u8; width * height * 3];
  yuv.write_rgb8(&mut rgb);
  RgbImage::from_raw(width as u32, height as u32, rgb)
      .ok_or_else(|| ThumbnailError::Decode("YUV → RGB buffer size mismatch".to_string()))
}
