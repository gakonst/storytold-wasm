//! Headless bindings. No DOM, filesystem, GPU, sockets, or HTTP policy caps.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

use designcraft_engine::{Session, script};
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;

fn js_error(e: impl std::fmt::Display) -> JsValue {
    js_sys::Error::new(&e.to_string()).into()
}

/// A caller-owned session: all upstream commands, history, multiple documents and clipboard.
#[wasm_bindgen]
#[derive(Default)]
pub struct HeadlessSession {
    session: Session,
}

#[wasm_bindgen]
impl HeadlessSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn commands(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.session.commands()).map_err(js_error)
    }

    pub fn execute(&mut self, command: &str, params_json: &str) -> Result<String, JsValue> {
        let params: Value = serde_json::from_str(params_json).map_err(js_error)?;
        if !params.is_object() {
            return Err(js_error("params must be an object"));
        }
        self.run_command(command, &params).map(|v| v.to_string()).map_err(js_error)
    }

    /// JSON arrays, JSONL and compact scripts; upstream $N.path result references.
    pub fn script(&mut self, text: &str) -> Result<String, JsValue> {
        let steps = script::parse(text).map_err(js_error)?;
        Ok(self.run_steps(&steps).to_json().to_string())
    }

    pub fn open(&mut self, bytes: &[u8]) -> Result<usize, JsValue> {
        let doc = if designcraft_idml::is_idml(bytes) {
            designcraft_engine::cmd::interchange::import(bytes, None).map_err(js_error)?
        } else {
            designcraft_format::load(bytes).map_err(js_error)?
        };
        Ok(self.session.add_document(designcraft_engine::DocState::new(doc, None)))
    }

    pub fn save(&self) -> Result<Vec<u8>, JsValue> {
        let doc = self.session.doc().map_err(js_error)?;
        designcraft_format::save(&doc.doc).map_err(js_error)
    }

    /// Absolute zero-based page; CPU rendering at pixels per point.
    pub fn render(&self, page: usize, scale: f64, bleed: bool, jpeg: bool) -> Result<Vec<u8>, JsValue> {
        if !scale.is_finite() || scale <= 0.0 {
            return Err(js_error("scale must be finite and positive"));
        }
        let doc = self.session.doc().map_err(js_error)?;
        let mut renderer = designcraft_render::Renderer::new();
        let opts = designcraft_render::RenderOptions { printing_only: true, ..Default::default() };
        let image = renderer
            .render_page(&doc.doc, &self.session.cache, page, scale, bleed, &opts)
            .ok_or_else(|| js_error("page missing or dimensions exceed the upstream CPU renderer's representable output"))?;
        Ok(if jpeg { image.to_jpeg(90) } else { image.to_png() })
    }
}

fn response(bytes: &[u8], status: u16, mime: &str) -> Result<web_sys::Response, JsValue> {
    let init = web_sys::ResponseInit::new();
    init.set_status(status);
    let mut body = bytes.to_vec();
    let response = web_sys::Response::new_with_opt_u8_array_and_init(Some(&mut body), &init)?;
    response.headers().set("content-type", mime)?;
    response.headers().set("cache-control", "no-store")?;
    Ok(response)
}

fn json_response(value: Value, status: u16) -> Result<web_sys::Response, JsValue> {
    response(value.to_string().as_bytes(), status, "application/json")
}

fn message(error: JsValue) -> String {
    error
        .dyn_ref::<js_sys::Error>()
        .map(|e| String::from(e.message()))
        .or_else(|| error.as_string())
        .unwrap_or_else(|| "WASM operation failed".into())
}

/// HTTP routing and all engine orchestration live in Rust. Each request owns a fresh session.
#[wasm_bindgen]
pub async fn fetch(request: web_sys::Request) -> Result<web_sys::Response, JsValue> {
    match handle(request).await {
        Ok(response) => Ok(response),
        Err(error) => json_response(json!({"error": message(error)}), 400),
    }
}

async fn handle(request: web_sys::Request) -> Result<web_sys::Response, JsValue> {
    let path = web_sys::Url::new(&request.url())?.pathname();
    match (request.method().as_str(), path.as_str()) {
        ("GET", "/health") => json_response(json!({"service": "designcraft", "runtime": "wasm32", "state": "request-scoped"}), 200),
        ("GET", "/commands") => {
            let session = HeadlessSession::new();
            response(session.commands()?.as_bytes(), 200, "application/json")
        }
        ("POST", "/run") => {
            let body = wasm_bindgen_futures::JsFuture::from(request.text()?).await?.as_string().ok_or_else(|| js_error("expected JSON body"))?;
            let value: Value = serde_json::from_str(&body).map_err(js_error)?;
            if !value.is_object() {
                return Err(js_error("expected JSON object"));
            }
            let mut session = HeadlessSession::new();
            if let Some(input) = value.get("input") {
                let input = input.as_str().ok_or_else(|| js_error("input must be a base64 document"))?;
                session.open(&designcraft_engine::cmd::base64_decode(input))?;
            }
            let steps = match value.get("steps") {
                Some(Value::String(text)) => script::parse(text),
                Some(Value::Array(steps)) => script::parse(&Value::Array(steps.clone()).to_string()),
                None => Ok(Vec::new()),
                _ => return Err(js_error("steps must be a script string or array")),
            }
            .map_err(js_error)?;
            let report = session.run_steps(&steps);
            if report.failed.is_some() {
                return json_response(report.to_json(), 422);
            }
            let output = value.get("output").cloned().unwrap_or(json!({"format": "json"}));
            let format = output.get("format").and_then(Value::as_str).ok_or_else(|| js_error("output.format must be a string"))?;
            match format {
                "json" => json_response(report.to_json(), 200),
                "designcraft" => response(&session.save()?, 200, designcraft_format::MIME),
                "png" | "jpeg" => {
                    let page = match output.get("page") {
                        None => 0,
                        Some(v) => v.as_u64().and_then(|n| usize::try_from(n).ok()).ok_or_else(|| js_error("page must be a zero-based integer"))?,
                    };
                    let scale = match output.get("scale") {
                        None => 1.0,
                        Some(v) => v.as_f64().ok_or_else(|| js_error("scale must be a number"))?,
                    };
                    let bleed = output.get("bleed").and_then(Value::as_bool).unwrap_or(false);
                    response(&session.render(page, scale, bleed, format == "jpeg")?, 200, if format == "png" { "image/png" } else { "image/jpeg" })
                }
                "pdf" | "idml" | "epub" | "html" => {
                    let (command, mime) = match format {
                        "pdf" => ("file.exportPdf", "application/pdf"),
                        "idml" => ("file.exportIdml", "application/vnd.adobe.indesign-idml-package"),
                        "epub" => ("file.exportEpub", "application/epub+zip"),
                        _ => ("file.exportHtml", "text/html; charset=utf-8"),
                    };
                    let params = output.get("params").cloned().unwrap_or(json!({}));
                    let result: Value = serde_json::from_str(&session.execute(command, &params.to_string())?).map_err(js_error)?;
                    let bytes = if let Some(text) = result.get("text").and_then(Value::as_str) {
                        text.as_bytes().to_vec()
                    } else {
                        designcraft_engine::cmd::base64_decode(
                            result.get("base64").and_then(Value::as_str).ok_or_else(|| js_error("export did not return bytes; omit path"))?,
                        )
                    };
                    response(&bytes, 200, mime)
                }
                _ => Err(js_error("unknown output format; use any upstream export command in steps for other formats")),
            }
        }
        (_, "/run" | "/commands" | "/health") => json_response(json!({"error": "method not allowed"}), 405),
        _ => json_response(json!({"error": "not found"}), 404),
    }
}

impl HeadlessSession {
    fn run_command(&mut self, command: &str, params: &Value) -> Result<Value, String> {
        // Engine commands can request a native/browser picker; headless callers must supply bytes.
        let result = self.session.execute(command, params).map_err(|e| e.to_string())?;
        if !self.session.ui_requests.is_empty() {
            self.session.ui_requests.clear();
            return Err("interactive UI unavailable in a headless session; supply bytes and explicit parameters".into());
        }
        Ok(result)
    }

    fn run_steps(&mut self, steps: &[script::Step]) -> script::Report {
        script::run(steps, |command, params| self.run_command(command, &params))
    }
}
