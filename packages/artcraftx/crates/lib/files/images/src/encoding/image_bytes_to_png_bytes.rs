use crate::error::ImagesError;
use image::{ImageFormat, ImageReader};
use std::io::Cursor;

pub fn image_bytes_to_png_bytes(
  arbitrary_image_bytes: &[u8],
) -> Result<Vec<u8>, ImagesError> {
  let reader = ImageReader::new(Cursor::new(arbitrary_image_bytes));

  let image = crate::decode_reader(reader.with_guessed_format()?)?;
  
  let mut output_bytes: Vec<u8> = Vec::new();
  image.write_to(&mut Cursor::new(&mut output_bytes), ImageFormat::Png)?;

  Ok(output_bytes)
}
