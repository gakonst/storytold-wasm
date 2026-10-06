use crate::{js_error, json};
use images::image::{DynamicImage, ImageFormat, ImageReader, imageops::FilterType};
use serde::Deserialize;
use serde_json::json;
use std::io::Cursor;
use wasm_bindgen::JsValue;

fn decode(bytes: &[u8]) -> Result<DynamicImage, JsValue> {
    let mut reader = ImageReader::new(Cursor::new(bytes)).with_guessed_format().map_err(js_error)?;
    // Resource limits belong to the host, not an arbitrary adapter policy.
    reader.no_limits();
    reader.decode().map_err(js_error)
}

pub fn info(bytes: &[u8]) -> Result<String, JsValue> {
    let image = decode(bytes)?;
    let mime = mimetypes::mimetype_for_bytes::get_mimetype_for_bytes_or_default(bytes);
    json(json!({"width": image.width(), "height": image.height(), "color": format!("{:?}", image.color()),
        "mime": mime, "extension": mimetypes::mimetype_to_extension::mimetype_to_extension(mime)}))
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Filter { Nearest, Triangle, CatmullRom, Gaussian, Lanczos3 }
impl From<Filter> for FilterType {
    fn from(f: Filter) -> Self { match f { Filter::Nearest => Self::Nearest, Filter::Triangle => Self::Triangle,
        Filter::CatmullRom => Self::CatmullRom, Filter::Gaussian => Self::Gaussian, Filter::Lanczos3 => Self::Lanczos3 } }
}
fn default_filter() -> Filter { Filter::Lanczos3 }
fn default_format() -> String { "png".into() }

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Options {
    #[serde(default)] operations: Vec<Operation>,
    #[serde(default = "default_format")] format: String,
    #[serde(default)] jpeg_quality: Option<u8>,
}

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
enum Operation {
    Resize { width: u32, height: u32, #[serde(default)] exact_fit: bool },
    ResizeExact { width: u32, height: u32, #[serde(default = "default_filter")] filter: Filter },
    ResizeFilter { width: u32, height: u32, #[serde(default = "default_filter")] filter: Filter },
    ResizeToFill { width: u32, height: u32, #[serde(default = "default_filter")] filter: Filter },
    Thumbnail { width: u32, height: u32 },
    ThumbnailExact { width: u32, height: u32 },
    Crop { x: u32, y: u32, width: u32, height: u32 },
    Rotate90, Rotate180, Rotate270, FlipHorizontal, FlipVertical, Grayscale, Invert,
    Blur { sigma: f32 }, FastBlur { sigma: f32 },
    Unsharpen { sigma: f32, threshold: i32 },
    Filter3x3 { kernel: [f32; 9] },
    Contrast { value: f32 }, Brighten { value: i32 }, HueRotate { degrees: i32 },
    Color { color: String },
}
fn dimensions(width: u32, height: u32) -> Result<(), JsValue> {
    if width == 0 || height == 0 { return Err(js_error("image dimensions must be positive")); }
    Ok(())
}
fn sigma(value: f32) -> Result<f32, JsValue> {
    if !value.is_finite() || value < 0.0 { return Err(js_error("sigma must be finite and nonnegative")); }
    Ok(value)
}

pub fn transform(bytes: &[u8], options: &str) -> Result<Vec<u8>, JsValue> {
    let options: Options = serde_json::from_str(options).map_err(js_error)?;
    let mut img = decode(bytes)?;
    for operation in options.operations {
        img = match operation {
            Operation::Resize { width, height, exact_fit } => { dimensions(width,height)?; images::resize_preserving_aspect::resize_preserving_aspect(&img,width,height,exact_fit) },
            Operation::ResizeExact { width, height, filter } => { dimensions(width,height)?; img.resize_exact(width,height,filter.into()) },
            Operation::ResizeFilter { width, height, filter } => { dimensions(width,height)?; img.resize(width,height,filter.into()) },
            Operation::ResizeToFill { width, height, filter } => { dimensions(width,height)?; img.resize_to_fill(width,height,filter.into()) },
            Operation::Thumbnail { width, height } => { dimensions(width,height)?; img.thumbnail(width,height) },
            Operation::ThumbnailExact { width, height } => { dimensions(width,height)?; img.thumbnail_exact(width,height) },
            Operation::Crop { x, y, width, height } => {
                dimensions(width,height)?;
                if x.checked_add(width).is_none_or(|v| v > img.width()) || y.checked_add(height).is_none_or(|v| v > img.height()) {
                    return Err(js_error("crop must lie within the image"));
                }
                img.crop_imm(x,y,width,height)
            },
            Operation::Rotate90 => img.rotate90(), Operation::Rotate180 => img.rotate180(), Operation::Rotate270 => img.rotate270(),
            Operation::FlipHorizontal => img.fliph(), Operation::FlipVertical => img.flipv(), Operation::Grayscale => img.grayscale(),
            Operation::Invert => { img.invert(); img },
            Operation::Blur { sigma: s } => img.blur(sigma(s)?),
            Operation::FastBlur { sigma: s } => img.fast_blur(sigma(s)?),
            Operation::Unsharpen { sigma: s, threshold } => img.unsharpen(sigma(s)?, threshold),
            Operation::Filter3x3 { kernel } => {
                if kernel.iter().any(|v| !v.is_finite()) { return Err(js_error("kernel must be finite")); }
                img.filter3x3(&kernel)
            },
            Operation::Contrast { value } => {
                if !value.is_finite() { return Err(js_error("contrast must be finite")); }
                img.adjust_contrast(value)
            },
            Operation::Brighten { value } => img.brighten(value), Operation::HueRotate { degrees } => img.huerotate(degrees),
            Operation::Color { color } => match color.as_str() {
                "l8" => DynamicImage::ImageLuma8(img.to_luma8()), "la8" => DynamicImage::ImageLumaA8(img.to_luma_alpha8()),
                "rgb8" => DynamicImage::ImageRgb8(img.to_rgb8()), "rgba8" => DynamicImage::ImageRgba8(img.to_rgba8()),
                "l16" => DynamicImage::ImageLuma16(img.to_luma16()), "la16" => DynamicImage::ImageLumaA16(img.to_luma_alpha16()),
                "rgb16" => DynamicImage::ImageRgb16(img.to_rgb16()), "rgba16" => DynamicImage::ImageRgba16(img.to_rgba16()),
                "rgb32f" => DynamicImage::ImageRgb32F(img.to_rgb32f()), "rgba32f" => DynamicImage::ImageRgba32F(img.to_rgba32f()),
                _ => return Err(js_error("unknown color type")),
            },
        };
    }
    // Resolve using image's format registry, not an adapter codec whitelist.
    let format = ImageFormat::from_extension(&options.format).ok_or_else(|| js_error("unknown output format"))?;
    let mut out = Cursor::new(Vec::new());
    if format == ImageFormat::Jpeg {
        let quality = options.jpeg_quality.unwrap_or(80);
        if !(1..=100).contains(&quality) { return Err(js_error("JPEG quality must be 1..=100")); }
        images::image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality).encode_image(&img.to_rgb8()).map_err(js_error)?;
    } else {
        if options.jpeg_quality.is_some() { return Err(js_error("jpeg_quality requires JPEG output")); }
        img.write_to(&mut out, format).map_err(js_error)?;
    }
    Ok(out.into_inner())
}
