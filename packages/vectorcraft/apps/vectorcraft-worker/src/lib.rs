//! Full headless engine bindings. No adapter command allowlist or resource policy.
//! Filesystem operations retain upstream WASM errors; import and export use bytes.
#![forbid(unsafe_code)]

use serde_json::{Value, json};
use vectorcraft_engine::cmd::fileio;
use vectorcraft_mcp::{Backend, Headless, Server};
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
mod http;

fn parse(raw: &str) -> Result<Value, String> {
    serde_json::from_str(raw).map_err(|e| format!("invalid JSON: {e}"))
}

fn object(raw: &str) -> Result<Value, String> {
    let value = parse(raw)?;
    if !value.is_object() {
        return Err("parameters must be a JSON object".into());
    }
    Ok(value)
}

/// A caller-owned session with documents, history and tool state.
/// Plug-ins retain the upstream isolate-wide registry for direct binding callers.
/// Call `free()` when finished. HTTP requests each own a separate session.
#[wasm_bindgen]
pub struct VectorCraft {
    headless: Headless,
    warnings: Vec<String>,
}

impl Default for VectorCraft {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl VectorCraft {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { headless: Headless::new(), warnings: Vec::new() }
    }

    /// Every upstream engine and headless host command, including disabled commands.
    pub fn commands(&mut self) -> Result<String, String> {
        self.call("engine.commands", "{}")
    }

    /// Full upstream headless control protocol, including document-space tool gestures.
    pub fn call(&mut self, method: &str, params: &str) -> Result<String, String> {
        self.headless.call(method, object(params)?).map(|v| v.to_string())
    }

    pub fn execute(&mut self, command: &str, params: &str) -> Result<String, String> {
        self.headless.call("engine.execute", json!({"command": command, "params": object(params)?})).map(|v| v.to_string())
    }

    /// Opens every upstream readable format. `name` supplies an extension, never a disk path.
    pub fn open(&mut self, name: &str, bytes: &[u8], options: &str) -> Result<String, String> {
        fileio::open_bytes_with(&mut self.headless.session, name, bytes, None, &object(options)?).map(|v| v.to_string()).map_err(|e| e.to_string())
    }

    pub fn inspect(&mut self) -> Result<String, String> {
        self.call("document.inspect", "{}")
    }

    /// Single-file binary export. Multi-file exports use `document.export` or
    /// `document.exportForScreens` through `execute`, returning all files and warnings.
    pub fn export(&mut self, format: &str, options: &str) -> Result<Vec<u8>, String> {
        self.warnings.clear();
        let params = object(options)?;
        let st = self.headless.session.doc().map_err(|e| e.to_string())?;
        let doc = fileio::export_source(st, &params).map_err(|e| e.to_string())?;
        let (bytes, warnings) = fileio::encode_with_warnings(&doc, format, &params).map_err(|e| e.to_string())?;
        self.warnings = warnings;
        Ok(bytes)
    }

    pub fn warnings(&self) -> String {
        json!(self.warnings).to_string()
    }
}

/// Stateful MCP JSON-RPC binding using the real upstream server and tool definitions.
/// This is the protocol engine, not an implementation of MCP Streamable HTTP sessions.
#[wasm_bindgen]
pub struct McpSession {
    server: Server,
}

impl Default for McpSession {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl McpSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { server: Server::new(Box::new(Headless::new())) }
    }

    pub fn handle(&mut self, message: &str) -> Option<String> {
        self.server.handle_line(message)
    }
}
