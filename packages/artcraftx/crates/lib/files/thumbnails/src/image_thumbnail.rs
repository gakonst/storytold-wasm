use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use images::image::codecs::jpeg::JpegEncoder;
use images::image::DynamicImage;
use images::resize_preserving_aspect::resize_preserving_aspect;

use crate::error::ThumbnailError;

const JPEG_QUALITY: u8 = 80;

/// Decode an image file, downscale it (aspect-preserving, longest edge =
/// `max_dimension`, never upscaled), and write it as a JPEG.
///
/// Blocking CPU + I/O — call from `spawn_blocking` in async contexts.
pub fn generate_image_thumbnail<P: AsRef<Path>, Q: AsRef<Path>>(
  source: P,
  destination_jpg: Q,
  max_dimension: u32,
) -> Result<(), ThumbnailError> {
  let decoded = images::image::open(source.as_ref())
      .map_err(|err| ThumbnailError::Unsupported(format!("{}: {}", source.as_ref().display(), err)))?;
  let thumbnail = downscale(&decoded, max_dimension);
  write_jpeg(&thumbnail, destination_jpg.as_ref())
}

/// In-memory counterpart of `generate_image_thumbnail`, including its no-upscale
/// behavior and JPEG quality. The caller owns storage of the returned bytes.
pub fn generate_image_thumbnail_bytes(
  source: &[u8],
  max_dimension: u32,
) -> Result<Vec<u8>, ThumbnailError> {
  if max_dimension == 0 {
    return Err(ThumbnailError::Unsupported("thumbnail dimension must be positive".into()));
  }
  let reader = images::image::ImageReader::new(std::io::Cursor::new(source))
      .with_guessed_format()?;
  let decoded = images::decode_reader(reader)
      .map_err(|err| ThumbnailError::Unsupported(err.to_string()))?;
  let thumbnail = downscale(&decoded, max_dimension);
  let mut bytes = Vec::new();
  JpegEncoder::new_with_quality(&mut bytes, JPEG_QUALITY)
      .encode_image(&thumbnail.to_rgb8())
      .map_err(|err| ThumbnailError::Encode(err.to_string()))?;
  Ok(bytes)
}

/// Downscale only; small sources pass through at native size.
pub(crate) fn downscale(decoded: &DynamicImage, max_dimension: u32) -> DynamicImage {
  if decoded.width() <= max_dimension && decoded.height() <= max_dimension {
    decoded.clone()
  } else {
    resize_preserving_aspect(decoded, max_dimension, max_dimension, false)
  }
}

/// Write via a temp file + rename so a crash never leaves a half-written
/// cache entry behind.
pub(crate) fn write_jpeg(thumbnail: &DynamicImage, destination: &Path) -> Result<(), ThumbnailError> {
  let temp_path = temp_sibling(destination);
  {
    let file = File::create(&temp_path)?;
    let mut encoder = JpegEncoder::new_with_quality(BufWriter::new(file), JPEG_QUALITY);
    encoder.encode_image(&thumbnail.to_rgb8())
        .map_err(|err| ThumbnailError::Encode(err.to_string()))?;
  }
  std::fs::rename(&temp_path, destination)?;
  Ok(())
}

pub(crate) fn write_bytes_atomically(bytes: &[u8], destination: &Path) -> Result<(), ThumbnailError> {
  let temp_path = temp_sibling(destination);
  std::fs::write(&temp_path, bytes)?;
  std::fs::rename(&temp_path, destination)?;
  Ok(())
}

fn temp_sibling(destination: &Path) -> std::path::PathBuf {
  let mut name = destination.file_name().unwrap_or_default().to_os_string();
  name.push(".tmp");
  destination.with_file_name(name)
}

#[cfg(test)]
mod tests {
  use images::image::RgbImage;

  use super::*;

  #[test]
  fn thumbnails_are_downscaled_and_written_atomically() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("source.png");
    let wide = DynamicImage::ImageRgb8(RgbImage::from_fn(1024, 512, |x, _| {
      images::image::Rgb([(x % 256) as u8, 0, 0])
    }));
    wide.save(&source).unwrap();

    let destination = dir.path().join("thumb.jpg");
    generate_image_thumbnail(&source, &destination, 512).unwrap();

    let thumbnail = images::image::open(&destination).unwrap();
    assert_eq!(thumbnail.width(), 512);
    assert_eq!(thumbnail.height(), 256);
    assert!(!dir.path().join("thumb.jpg.tmp").exists());
  }

  #[test]
  fn small_images_are_not_upscaled() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("small.png");
    DynamicImage::ImageRgb8(RgbImage::new(100, 60)).save(&source).unwrap();

    let destination = dir.path().join("thumb.jpg");
    generate_image_thumbnail(&source, &destination, 512).unwrap();
    let thumbnail = images::image::open(&destination).unwrap();
    assert_eq!((thumbnail.width(), thumbnail.height()), (100, 60));
  }

  #[test]
  fn non_images_are_unsupported() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("not_an_image.bin");
    std::fs::write(&source, b"definitely not pixels").unwrap();
    let err = generate_image_thumbnail(&source, dir.path().join("t.jpg"), 512).unwrap_err();
    assert!(err.is_unsupported());
  }
}
