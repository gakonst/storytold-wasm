//! Headless PhotoCraft: upstream command engine and byte APIs, without a DOM.
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use photocraft_engine::{Session, command_specs, inspect};
use serde_json::json;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub struct PhotoCraft {
    session: Session,
    warnings: Vec<String>,
}
#[wasm_bindgen]
impl PhotoCraft {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { session: Session::new(), warnings: Vec::new() }
    }
    pub fn commands(&self) -> String {
        json!(command_specs().iter().map(|c| json!({"id":c.id,"label":c.label,"params":c.params,"enabled":self.session.is_enabled(c.id)})).collect::<Vec<_>>())
            .to_string()
    }
    pub fn execute(&mut self, id: &str, params: &str) -> Result<String, String> {
        let p = serde_json::from_str(params).map_err(|e| e.to_string())?;
        self.session.execute(id, p).map(|v| v.to_string()).map_err(|e| e.to_string())
    }
    pub fn inspect(&self) -> String {
        inspect::session(&self.session).to_string()
    }
    pub fn open(&mut self, name: &str, bytes: &[u8]) -> Result<String, String> {
        let imported = photocraft_io::import(name, bytes).map_err(|e| e.to_string())?;
        let index = self.session.add_document(imported.document, None);
        self.warnings = imported.warnings;
        Ok(json!({"index":index,"warnings":self.warnings}).to_string())
    }
    pub fn export(&mut self, format: &str) -> Result<Vec<u8>, String> {
        let state = self.session.active().ok_or("no active document")?;
        let output = photocraft_io::export(&state.doc, format, &Default::default()).map_err(|e| e.to_string())?;
        self.warnings = output.warnings;
        Ok(output.bytes)
    }
    pub fn warnings(&self) -> String {
        json!(self.warnings).to_string()
    }
}
impl Default for PhotoCraft {
    fn default() -> Self {
        Self::new()
    }
}
/// Uses upstream codec validation and limits, without adapter-specific restrictions.
#[wasm_bindgen]
pub fn convert(bytes: &[u8], format: &str) -> Result<Vec<u8>, String> {
    let decoded = photocraft_codecs::decode(bytes).map_err(|e| e.to_string())?;
    let target = photocraft_codecs::from_extension(format).ok_or("unknown output format")?;
    photocraft_codecs::encode(&decoded, target, &Default::default()).map_err(|e| e.to_string())
}
