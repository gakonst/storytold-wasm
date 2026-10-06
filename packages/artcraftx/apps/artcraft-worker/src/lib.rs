use wasm_bindgen::prelude::*;

mod catalog;
mod imaging;
mod utilities;

pub(crate) fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}

pub(crate) fn json(value: impl serde::Serialize) -> Result<String, JsValue> {
    serde_json::to_string(&value).map_err(js_error)
}

#[wasm_bindgen]
pub fn catalog() -> Result<String, JsValue> { catalog::catalog() }

#[wasm_bindgen]
pub fn model_config(modality: &str, model: &str, provider: Option<String>) -> Result<String, JsValue> {
    catalog::model_config(modality, model, provider)
}

#[wasm_bindgen]
pub fn inspect_bytes(bytes: &[u8]) -> Result<String, JsValue> { utilities::inspect_bytes(bytes) }

#[wasm_bindgen]
pub fn utility(request: &str) -> Result<String, JsValue> { utilities::utility(request) }

#[wasm_bindgen]
pub fn decode_base64(input: &str) -> Result<Vec<u8>, JsValue> {
    web_base64::web_base64_decode::web_base64_decode(input).map_err(js_error)
}

#[wasm_bindgen]
pub fn image_info(bytes: &[u8]) -> Result<String, JsValue> { imaging::info(bytes) }

#[wasm_bindgen]
pub fn transform_image(bytes: &[u8], options: &str) -> Result<Vec<u8>, JsValue> {
    imaging::transform(bytes, options)
}

#[wasm_bindgen]
pub fn normalize_flux_mask(bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    images::mask_images::normalize_image_bytes_to_flux_mask::normalize_image_bytes_to_flux_mask(bytes)
        .map(|png| png.0).map_err(js_error)
}

#[wasm_bindgen]
pub fn image_to_png(bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    images::encoding::image_bytes_to_png_bytes::image_bytes_to_png_bytes(bytes).map_err(js_error)
}

#[wasm_bindgen]
pub fn webp_to_png(bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    images::encoding::webp_bytes_to_png_bytes::webp_bytes_to_png_bytes(bytes).map_err(js_error)
}

#[wasm_bindgen]
pub fn image_thumbnail(bytes: &[u8], max_dimension: u32) -> Result<Vec<u8>, JsValue> {
    thumbnails::image_thumbnail::generate_image_thumbnail_bytes(bytes, max_dimension).map_err(js_error)
}
