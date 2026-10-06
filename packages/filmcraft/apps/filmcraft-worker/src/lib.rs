//! Headless bindings. Files are supplied by the host; no DOM, GPU or native filesystem.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::collections::BTreeMap;
use std::io::{self, Cursor};
use std::sync::{Arc, Mutex};

use filmcraft_engine::{Services, Session};
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;

#[derive(Clone, Default)]
struct MemoryFiles(Arc<Mutex<BTreeMap<String, Vec<u8>>>>);

impl Services for MemoryFiles {
    fn read_file(&self, path: &str) -> io::Result<Vec<u8>> {
        self.0.lock().unwrap_or_else(|e| e.into_inner()).get(path).cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("{path}: not in request file store")))
    }
    fn write_file(&self, path: &str, data: &[u8]) -> io::Result<()> {
        self.0.lock().unwrap_or_else(|e| e.into_inner()).insert(path.into(), data.into());
        Ok(())
    }
    fn read_range(&self, path: &str, offset: u64, len: usize) -> io::Result<Vec<u8>> {
        let files = self.0.lock().unwrap_or_else(|e| e.into_inner());
        let data = files.get(path).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, path.to_string()))?;
        let start = usize::try_from(offset).unwrap_or(usize::MAX).min(data.len());
        Ok(data.get(start..start.saturating_add(len).min(data.len())).unwrap_or_default().to_vec())
    }
    fn file_size(&self, path: &str) -> io::Result<u64> {
        self.0.lock().unwrap_or_else(|e| e.into_inner()).get(path).map(|v| v.len() as u64)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, path.to_string()))
    }
    fn list_dir(&self, dir: &str) -> Option<io::Result<Vec<String>>> {
        let prefix = if dir.is_empty() { String::new() } else { format!("{}/", dir.trim_end_matches('/')) };
        Some(Ok(self.0.lock().unwrap_or_else(|e| e.into_inner()).keys()
            .filter_map(|path| path.strip_prefix(&prefix).filter(|s| !s.contains('/')).map(str::to_string)).collect()))
    }
    fn file_loader(&self) -> Option<filmcraft_media::sequence::FrameLoader> {
        let files = self.clone();
        Some(Arc::new(move |path| files.read_file(path)))
    }
    fn export_in_memory(&self) -> bool { true }
}

fn js_error(e: impl std::fmt::Display) -> JsValue { JsValue::from_str(&e.to_string()) }

/// One request-local editing session. The full engine registry remains accessible.
#[wasm_bindgen]
pub struct HeadlessSession {
    session: Session,
    files: MemoryFiles,
}

#[wasm_bindgen]
impl HeadlessSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let files = MemoryFiles::default();
        Self { session: Session::new(Arc::new(files.clone())), files }
    }

    pub fn execute(&mut self, command: &str, params: &str) -> Result<String, JsValue> {
        let params: Value = serde_json::from_str(params).map_err(js_error)?;
        if !params.is_object() { return Err(js_error("command params must be an object")); }
        self.session.execute(command, params).map(|v| v.to_string()).map_err(js_error)
    }

    pub fn put_file(&self, path: &str, bytes: &[u8]) -> Result<(), JsValue> {
        if path.is_empty() { return Err(js_error("file path must not be empty")); }
        self.files.write_file(path, bytes).map_err(js_error)
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, JsValue> {
        self.files.read_file(path).map_err(js_error)
    }

    pub fn files(&self) -> String {
        json!(self.files.0.lock().unwrap_or_else(|e| e.into_inner()).iter()
            .map(|(path, bytes)| json!({"path":path,"bytes":bytes.len()})).collect::<Vec<_>>()).to_string()
    }

    /// Versioned .fcproj bytes, independent of filesystem persistence.
    pub fn project(&self) -> Vec<u8> { filmcraft_format::encode(&self.session.project, false) }

    /// Advance cooperative export jobs by one step (no Worker clock deadline required).
    pub fn pump(&mut self) -> bool { self.session.pump_jobs(std::time::Duration::ZERO) }

    pub fn render_png(&self, scale: f32) -> Result<Vec<u8>, JsValue> {
        if !scale.is_finite() || scale <= 0.0 { return Err(js_error("scale must be finite and positive")); }
        let seq = self.session.active_sequence().ok_or_else(|| js_error("no active sequence"))?;
        // Representation checks, not a policy limit: the CPU working image stores RGBA f32.
        let w = (f64::from(seq.settings.width) * f64::from(scale)).round().max(1.0);
        let h = (f64::from(seq.settings.height) * f64::from(scale)).round().max(1.0);
        if w > u32::MAX as f64 || h > u32::MAX as f64 || w * h * 16.0 > isize::MAX as f64 {
            return Err(js_error("render dimensions exceed addressable image representation"));
        }
        let frame = self.session.render_program(scale).ok_or_else(|| js_error("no active sequence"))?;
        let image = image::RgbaImage::from_raw(frame.w as u32, frame.h as u32, frame.to_rgba8())
            .ok_or_else(|| js_error("invalid rendered image layout"))?;
        let mut png = Cursor::new(Vec::new());
        image.write_to(&mut png, image::ImageFormat::Png).map_err(js_error)?;
        Ok(png.into_inner())
    }
}

impl Default for HeadlessSession {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn file_ranges_do_not_overflow() {
        let files = MemoryFiles::default();
        files.write_file("/a", b"abc").unwrap();
        assert_eq!(files.read_range("/a", 1, usize::MAX).unwrap(), b"bc");
        assert!(files.read_range("/a", u64::MAX, usize::MAX).unwrap().is_empty());
        assert!(files.read_file("/missing").is_err());
    }
}
