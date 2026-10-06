//! Video thumbnails: one decode pass over the leading ~3s produces both the
//! static first-frame JPEG and the animated WebP preview.

use std::path::Path;

use images::image::DynamicImage;
use webp_animation::{Encoder, EncoderOptions, EncodingConfig, EncodingType, LossyEncodingConfig};

use crate::error::ThumbnailError;
use crate::image_thumbnail::{downscale, write_bytes_atomically, write_jpeg};
use crate::mp4_frames::{decode_mp4_leading_frames, DecodedFrame};
use crate::{
  ANIMATED_PREVIEW_MAX_DECODED_FRAMES, ANIMATED_PREVIEW_MAX_DIMENSION, ANIMATED_PREVIEW_MAX_DURATION_MS,
  ANIMATED_PREVIEW_QUALITY, ANIMATED_PREVIEW_TARGET_FPS, STATIC_THUMBNAIL_MAX_DIMENSION,
};

#[derive(Debug)]
pub struct VideoThumbnailOutcome {
  pub wrote_static_thumbnail: bool,
  pub wrote_animated_preview: bool,
}

/// Decode the leading frames of `source` once; write the first frame as a
/// 512px JPEG and the whole ~3s run, subsampled to ~12fps, as a lossy 320px
/// animated WebP.
///
/// The animated preview is best-effort: if its encode fails, the static
/// thumbnail still lands and the outcome reports the difference.
///
/// Blocking CPU work — call from `spawn_blocking` in async contexts.
pub fn generate_video_thumbnails<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(
  source: P,
  destination_jpg: Q,
  destination_webp: R,
) -> Result<VideoThumbnailOutcome, ThumbnailError> {
  let frames = decode_mp4_leading_frames(
    source.as_ref(),
    ANIMATED_PREVIEW_MAX_DURATION_MS,
    ANIMATED_PREVIEW_MAX_DECODED_FRAMES,
  )?;

  let first_frame = DynamicImage::ImageRgb8(frames[0].image.clone());
  write_jpeg(&downscale(&first_frame, STATIC_THUMBNAIL_MAX_DIMENSION), destination_jpg.as_ref())?;

  let preview_frames = subsample_to_fps(&frames, ANIMATED_PREVIEW_TARGET_FPS);
  let wrote_animated_preview = match encode_animated_webp(&preview_frames) {
    Ok(webp_bytes) => {
      write_bytes_atomically(&webp_bytes, destination_webp.as_ref())?;
      true
    }
    Err(err) => {
      log::warn!("Animated preview encode failed for {}: {}", source.as_ref().display(), err);
      false
    }
  };

  Ok(VideoThumbnailOutcome { wrote_static_thumbnail: true, wrote_animated_preview })
}

/// Keep roughly one frame per `1000 / target_fps` ms: the first frame at or
/// after each slot boundary. Frames must be sorted by pts. A source already
/// at or below the target rate passes through untouched.
fn subsample_to_fps(frames: &[DecodedFrame], target_fps: u32) -> Vec<&DecodedFrame> {
  let slot_ms = 1000 / target_fps.max(1);
  let mut picked = Vec::with_capacity(frames.len());
  let mut next_slot_ms = 0u32;
  for frame in frames {
    if frame.pts_ms >= next_slot_ms {
      picked.push(frame);
      // Advance to the slot after this frame, so a frame landing late in its
      // slot doesn't also eat the next one.
      next_slot_ms = (frame.pts_ms / slot_ms + 1) * slot_ms;
    }
  }
  picked
}

fn encode_animated_webp(frames: &[&DecodedFrame]) -> Result<Vec<u8>, ThumbnailError> {
  let scaled: Vec<(u32, DynamicImage)> = frames.iter()
      .map(|frame| {
        let image = DynamicImage::ImageRgb8(frame.image.clone());
        (frame.pts_ms, downscale(&image, ANIMATED_PREVIEW_MAX_DIMENSION))
      })
      .collect();

  let (width, height) = (scaled[0].1.width(), scaled[0].1.height());
  let options = EncoderOptions {
    encoding_config: Some(EncodingConfig {
      encoding_type: EncodingType::Lossy(LossyEncodingConfig::default()),
      quality: ANIMATED_PREVIEW_QUALITY,
      method: 4,
    }),
    ..Default::default()
  };
  let mut encoder = Encoder::new_with_options((width, height), options)
      .map_err(|err| ThumbnailError::Encode(format!("webp encoder: {}", err)))?;

  // Normalize timestamps so the preview starts at 0 even if the stream's
  // first pts doesn't.
  let first_pts = scaled[0].0;
  let mut last_timestamp_ms: i32 = 0;
  for (pts_ms, image) in &scaled {
    last_timestamp_ms = pts_ms.saturating_sub(first_pts) as i32;
    let rgba = image.to_rgba8();
    encoder.add_frame(rgba.as_raw(), last_timestamp_ms)
        .map_err(|err| ThumbnailError::Encode(format!("webp frame: {}", err)))?;
  }

  // Close the last frame with a plausible display duration.
  let average_frame_ms = if scaled.len() > 1 {
    (last_timestamp_ms / (scaled.len() as i32 - 1)).max(1)
  } else {
    500
  };
  let webp_data = encoder.finalize(last_timestamp_ms + average_frame_ms)
      .map_err(|err| ThumbnailError::Encode(format!("webp finalize: {}", err)))?;
  Ok(webp_data.to_vec())
}

#[cfg(test)]
mod tests {
  use super::*;
  use images::image::RgbImage;

  // The 4-second, 12fps, B-frame (High profile) H.264 clip the
  // higgsfield_client live tests upload.
  const H264_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../api_clients/higgsfield_client/test_assets/clip_4s.mp4",
  );

  #[test]
  fn h264_mp4_produces_static_and_animated_thumbnails() {
    let dir = tempfile::tempdir().unwrap();
    let jpg = dir.path().join("thumb.jpg");
    let webp = dir.path().join("preview.webp");

    let outcome = generate_video_thumbnails(H264_FIXTURE, &jpg, &webp).unwrap();
    assert!(outcome.wrote_static_thumbnail);
    assert!(outcome.wrote_animated_preview);

    let thumbnail = images::image::open(&jpg).unwrap();
    assert!(thumbnail.width() > 0 && thumbnail.height() > 0);
    assert!(thumbnail.width() <= 512 && thumbnail.height() <= 512);

    let webp_bytes = std::fs::read(&webp).unwrap();
    assert!(webp_bytes.len() > 12);
    assert_eq!(&webp_bytes[0..4], b"RIFF");
    assert_eq!(&webp_bytes[8..12], b"WEBP");
    // Lossy at 320px: a 3s preview stays well under half a megabyte.
    assert!(webp_bytes.len() < 400 * 1024, "preview is {} bytes", webp_bytes.len());
  }

  #[test]
  fn b_frame_stream_decodes_the_whole_preview_window() {
    let frames = decode_mp4_leading_frames(H264_FIXTURE, ANIMATED_PREVIEW_MAX_DURATION_MS, ANIMATED_PREVIEW_MAX_DECODED_FRAMES).unwrap();
    // 12fps over ~3s; the old flush-per-decode setup stopped after a few
    // frames on any B-frame stream.
    assert!(frames.len() >= 35, "only {} frames decoded", frames.len());
    // Raw composition times (no edit-list shift), so the first pts is a
    // couple of frames in, not exactly 0.
    assert!(frames[0].pts_ms < 300, "first pts {}", frames[0].pts_ms);
    assert!(frames.last().unwrap().pts_ms >= 2_900);
    assert!(frames.windows(2).all(|pair| pair[0].pts_ms < pair[1].pts_ms), "pts must be strictly increasing");

    // Already at the target rate: nothing dropped.
    let preview = subsample_to_fps(&frames, ANIMATED_PREVIEW_TARGET_FPS);
    assert_eq!(preview.len(), frames.len());
  }

  #[test]
  fn subsampling_keeps_one_frame_per_slot() {
    let frames: Vec<DecodedFrame> = (0..24).map(|i| DecodedFrame {
      pts_ms: i * 1000 / 24,
      image: RgbImage::new(1, 1),
    }).collect();
    let picked: Vec<u32> = subsample_to_fps(&frames, 12).iter().map(|f| f.pts_ms).collect();
    assert_eq!(picked, vec![0, 83, 166, 250, 333, 416, 500, 583, 666, 750, 833, 916]);

    // Already sparse input passes through.
    let sparse: Vec<DecodedFrame> = [0u32, 500, 1000].iter().map(|&pts_ms| DecodedFrame { pts_ms, image: RgbImage::new(1, 1) }).collect();
    assert_eq!(subsample_to_fps(&sparse, 12).len(), 3);
  }

  #[test]
  fn non_mp4_is_unsupported() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("not_a_video.mp4");
    std::fs::write(&source, b"this is not an mp4 file at all").unwrap();
    let err = generate_video_thumbnails(&source, dir.path().join("t.jpg"), dir.path().join("p.webp"))
        .unwrap_err();
    assert!(err.is_unsupported());
  }
}

