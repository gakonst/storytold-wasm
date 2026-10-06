//! Full upstream automation in a request-local, byte-backed session.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use base64::{Engine, engine::general_purpose::STANDARD};
use printcraft_automation::{Automation, Content};
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;

fn error(e: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&e.to_string())
}

fn now() -> i64 {
    (js_sys::Date::now() / 1000.0) as i64
}

#[wasm_bindgen]
pub struct PrintCraft {
    engine: Automation,
}

impl Default for PrintCraft {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl PrintCraft {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { engine: Automation::in_memory().with_clock(now) }
    }

    pub fn put_file(&self, path: &str, bytes: Vec<u8>) -> Result<(), JsValue> {
        self.engine.put_file(path, bytes).map_err(error)
    }

    pub fn file(&self, path: &str) -> Result<Vec<u8>, JsValue> {
        self.engine.file_bytes(path).map_err(error)
    }

    pub fn files(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.engine.file_names().map_err(error)?).map_err(error)
    }

    /// Every upstream tool is dispatched through the shared automation table.
    pub fn call(&mut self, name: &str, arguments: &str) -> Result<String, JsValue> {
        let args: Value = serde_json::from_str(arguments).map_err(error)?;
        // These need actual OS facilities or model discovery, which this host does not have.
        match name {
            "printers" | "sign_keychain_ids" => return Err(error("unavailable in Workers: no OS printers or Keychain; use doc_print with path or a P-256 identity file")),
            "ocr_recognize" | "ocr_recognize_files" => return Err(error("OCR unavailable in Workers: upstream discovers RTEN models on the native filesystem; no model byte loader is wired to this session")),
            "doc_print" if args.get("printer").is_some_and(|v| !v.is_null()) => return Err(error("printer hardware is unavailable in Workers; use doc_print with path to export a print-ready PDF")),
            _ => {}
        }
        let result: Vec<Value> = self.engine.call(name, &args).map_err(error)?.into_iter().map(|c| match c {
            Content::Json(value) => json!({"type": "json", "value": value}),
            Content::Png { data, width, height } => json!({"type": "image", "mimeType": "image/png", "base64": STANDARD.encode(data), "width": width, "height": height}),
        }).collect();
        serde_json::to_string(&result).map_err(error)
    }
}

/// Live upstream schemas, with runtime differences attached instead of hiding tools.
#[wasm_bindgen]
pub fn tools() -> String {
    let tools: Vec<Value> = printcraft_automation::tools().into_iter().map(|t| {
        let availability = match t.name {
            "printers" | "sign_keychain_ids" => "unavailable: host hardware/OS service",
            "ocr_recognize" | "ocr_recognize_files" => "unavailable: native model discovery",
            "ocr_status" => "reports native model discovery (no models in Worker)",
            "sign_id_create" | "sign_document" => "portable P-256/P-384; RSA private-key operations unavailable upstream on wasm32",
            "doc_print" => "portable print-ready PDF via path; printer hardware unavailable",
            _ => "upstream automation with request-local memory files",
        };
        json!({"name": t.name, "title": t.title, "description": t.description, "input_schema": t.input_schema,
            "read_only": t.read_only, "destructive": t.destructive, "command": t.command, "availability": availability})
    }).collect();
    Value::Array(tools).to_string()
}
