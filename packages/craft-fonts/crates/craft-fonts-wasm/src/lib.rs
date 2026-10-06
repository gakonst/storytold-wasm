//! Font metadata and manifest parsing, usable in Workers or any JS WASM host.
#![forbid(unsafe_code)]
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;
fn value(f: &craft_fonts::Font) -> Value {
    json!({"family":f.family,"style":f.style,"file":f.file,"scripts":f.scripts,"licence":f.licence,"licence_file":f.licence_file,"sha256":f.sha256,"source":f.source})
}
#[wasm_bindgen]
pub fn catalog(script: &str) -> String {
    let fonts: Vec<Value> = if script.is_empty() {
        craft_fonts::FONTS.iter().map(value).collect()
    } else {
        craft_fonts::for_script(script).map(value).collect()
    };
    json!(fonts).to_string()
}
#[wasm_bindgen]
pub fn parse_manifest(text: &str) -> Result<String, String> {
    let entries = craft_fonts::parse_manifest(text).map_err(|e| e.to_string())?;
    Ok(json!(entries.into_iter().map(|f|json!({"family":f.family,"style":f.style,"file":f.file,"scripts":f.scripts,"licence":f.licence,"licence_file":f.licence_file,"sha256":f.sha256,"source":f.source})).collect::<Vec<_>>()).to_string())
}
/// Optional self-contained library build; Worker deployment serves fonts as static assets.
#[cfg(feature = "embed")]
#[wasm_bindgen]
pub fn font_bytes(index: usize) -> Result<Vec<u8>, String> {
    let font = craft_fonts::FONTS
        .get(index)
        .ok_or("font index out of range")?;
    craft_fonts::embedded(font)
        .map(<[u8]>::to_vec)
        .ok_or_else(|| "font is not embedded".into())
}
