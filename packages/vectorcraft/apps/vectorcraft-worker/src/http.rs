//! HTTP routes implemented in Rust; only the generated Worker entrypoint is JavaScript.
use js_sys::Uint8Array;
use serde::Deserialize;
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response, ResponseInit, Url};

use crate::{McpSession, VectorCraft, object, parse};

// Upstream plug-ins are process-global. HTTP work executes synchronously after body reads;
// clear its registry on every exit, including errors, before yielding to another request.
// Caller-owned wasm-bindgen sessions retain the upstream isolate-wide registry semantics.
struct PluginScope;
impl Drop for PluginScope {
    fn drop(&mut self) {
        for plugin in vectorcraft_plugins::registry::list() {
            vectorcraft_plugins::registry::remove(plugin.id());
        }
    }
}

fn empty_object() -> Value {
    json!({})
}

#[derive(Deserialize)]
struct Input {
    name: String,
    #[serde(rename = "dataBase64")]
    data: String,
    #[serde(default = "empty_object")]
    options: Value,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Step {
    Command {
        command: String,
        #[serde(default = "empty_object")]
        params: Value,
    },
    Control {
        method: String,
        #[serde(default = "empty_object")]
        params: Value,
    },
}

#[derive(Deserialize)]
struct Job {
    input: Option<Input>,
    #[serde(default)]
    steps: Vec<Step>,
    /// Export parameters, including `format`. The upstream command returns every output file.
    output: Option<Value>,
}

fn js_error(value: JsValue) -> String {
    value.as_string().unwrap_or_else(|| format!("Worker API error: {value:?}"))
}

fn response(bytes: &[u8], status: u16, mime: &str) -> Result<Response, String> {
    let init = ResponseInit::new();
    init.set_status(status);
    let mut body = bytes.to_vec();
    let response = Response::new_with_opt_u8_array_and_init(Some(&mut body), &init).map_err(js_error)?;
    response.headers().set("content-type", mime).map_err(js_error)?;
    Ok(response)
}

fn json_response(value: &Value, status: u16) -> Result<Response, String> {
    response(value.to_string().as_bytes(), status, "application/json")
}

async fn body_text(request: &Request) -> Result<String, String> {
    JsFuture::from(request.text().map_err(js_error)?).await.map_err(js_error)?.as_string().ok_or_else(|| "request body is not text".into())
}

fn mime(format: &str) -> &'static str {
    match format {
        "svg" => "image/svg+xml",
        "svgz" => "application/gzip",
        "pdf" | "ai" => "application/pdf",
        "png" | "png8" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "tif" | "tiff" => "image/tiff",
        "bmp" => "image/bmp",
        "eps" | "ps" => "application/postscript",
        "txt" => "text/plain",
        // Native files may be compressed or carry embedded previews.
        "vectorcraft" | "drawcraft" => "application/octet-stream",
        _ => "application/octet-stream",
    }
}

async fn route(request: Request) -> Result<Response, String> {
    let url = Url::new(&request.url()).map_err(js_error)?;
    let path = url.pathname();
    let method = request.method();
    let allowed = match path.as_str() {
        "/health" | "/commands" | "/formats" => "GET",
        "/run" | "/convert" | "/mcp" => "POST",
        _ => return json_response(&json!({"error": "not found"}), 404),
    };
    if method != allowed {
        let r = json_response(&json!({"error": "method not allowed"}), 405)?;
        r.headers().set("allow", allowed).map_err(js_error)?;
        return Ok(r);
    }
    match path.as_str() {
        "/health" => json_response(
            &json!({"engine": "vectorcraft", "runtime": "wasm32", "state": "request-scoped", "filesystem": false, "gpu": false,
                "mcp": "request-scoped JSON-RPC; use McpSession binding for persistent protocol state"}),
            200,
        ),
        "/commands" => response(VectorCraft::new().commands()?.as_bytes(), 200, "application/json"),
        "/formats" => response(VectorCraft::new().execute("document.formats", "{}")?.as_bytes(), 200, "application/json"),
        "/mcp" => {
            let body = body_text(&request).await?;
            let _plugins = PluginScope;
            let mut session = McpSession::new();
            // The upstream handler supports JSON-RPC arrays for compatibility. Ordered messages
            // in one request share a session; separate HTTP requests deliberately do not.
            match session.handle(&body) {
                Some(reply) => response(reply.as_bytes(), 200, "application/json"),
                None => response(&[], 204, "application/json"),
            }
        }
        "/convert" => {
            let query = url.search_params();
            let name = query.get("name").unwrap_or_else(|| "input".into());
            let format = query.get("format").unwrap_or_else(|| "png".into());
            let options = query.get("options").unwrap_or_else(|| "{}".into());
            let input_options = query.get("inputOptions").unwrap_or_else(|| "{}".into());
            let buffer = JsFuture::from(request.array_buffer().map_err(js_error)?).await.map_err(js_error)?;
            let bytes = Uint8Array::new(&buffer).to_vec();
            let _plugins = PluginScope;
            let mut engine = VectorCraft::new();
            let imported = engine.open(&name, &bytes, &input_options)?;
            let out = engine.export(&format, &options)?;
            let r = response(&out, 200, mime(&format))?;
            // Percent encoding preserves non-ASCII warnings in HTTP headers.
            r.headers().set("x-vectorcraft-import", &js_sys::encode_uri_component(&imported).as_string().unwrap_or_default()).map_err(js_error)?;
            r.headers()
                .set("x-vectorcraft-warnings", &js_sys::encode_uri_component(&engine.warnings()).as_string().unwrap_or_default())
                .map_err(js_error)?;
            Ok(r)
        }
        "/run" => {
            let body = body_text(&request).await?;
            let job: Job = serde_json::from_value(object(&body)?).map_err(|e| format!("invalid job: {e}"))?;
            let _plugins = PluginScope;
            let mut engine = VectorCraft::new();
            let imported = match job.input {
                Some(input) => {
                    let bytes = vectorcraft_format::base64_decode(&input.data).ok_or("invalid input.dataBase64")?;
                    Some(parse(&engine.open(&input.name, &bytes, &input.options.to_string())?)?)
                }
                None => None,
            };
            let mut results = Vec::new();
            for (index, step) in job.steps.into_iter().enumerate() {
                let result = match step {
                    Step::Command { command, params } => engine.execute(&command, &params.to_string()),
                    Step::Control { method, params } => engine.call(&method, &params.to_string()),
                };
                results.push(parse(&result.map_err(|e| format!("step {index}: {e}"))?)?);
            }
            let output = match job.output {
                Some(options) => Some(parse(&engine.execute("document.export", &options.to_string())?)?),
                None => None,
            };
            let state = if engine.headless.session.active().is_some() { parse(&engine.inspect()?)? } else { Value::Null };
            json_response(&json!({"results": results, "state": state, "imported": imported, "output": output}), 200)
        }
        _ => json_response(&json!({"error": "not found"}), 404),
    }
}

/// Cloudflare module Worker fetch entrypoint. Errors remain structured HTTP responses.
#[wasm_bindgen]
pub async fn worker_fetch(request: Request) -> Result<Response, JsValue> {
    match route(request).await {
        Ok(response) => Ok(response),
        Err(error) => json_response(&json!({"error": error}), 400).map_err(|e| JsValue::from_str(&e)),
    }
}
