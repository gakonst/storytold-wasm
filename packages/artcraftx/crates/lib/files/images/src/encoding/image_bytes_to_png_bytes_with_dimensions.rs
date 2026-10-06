use std::io::Cursor;
use image::{ImageFormat, ImageReader};
use crate::error::ImagesError;

pub struct PngBytesWithDimensions {
  pub png_bytes: Vec<u8>,
  pub width: u32,
  pub height: u32,
}

pub fn image_bytes_to_png_bytes_with_dimensions(
  arbitrary_image_bytes: &[u8],
) -> Result<PngBytesWithDimensions, ImagesError> {

  let reader = ImageReader::new(Cursor::new(arbitrary_image_bytes));

  let image = crate::decode_reader(reader.with_guessed_format()?)?;

  let mut output_bytes: Vec<u8> = Vec::new();
  image.write_to(&mut Cursor::new(&mut output_bytes), ImageFormat::Png)?;

  Ok(PngBytesWithDimensions {
    png_bytes: output_bytes,
    width: image.width(),
    height: image.height(),
  })
}
