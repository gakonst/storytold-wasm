//! A headless session with byte-backed media and output; no browser or native UI.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use effectcraft_engine::{Importer, Services, Session, project::Footage};
use serde::Deserialize;
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;

type Files = BTreeMap<String, Arc<[u8]>>;

#[derive(Default)]
struct MemoryFiles(Mutex<Files>);

impl MemoryFiles {
    fn get(&self, path: &str) -> Option<Arc<[u8]>> {
        self.0.lock().unwrap_or_else(|e| e.into_inner()).get(path).cloned()
    }
    fn put(&self, path: &str, bytes: Arc<[u8]>) {
        self.0.lock().unwrap_or_else(|e| e.into_inner()).insert(path.into(), bytes);
    }
}

impl Services for MemoryFiles {
    fn read_file(&self, path: &str) -> std::io::Result<Vec<u8>> {
        self.get(path).map(|b| b.to_vec()).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("{path}: upload bytes in files; native filesystem unavailable")))
    }
    fn write_file(&self, path: &str, bytes: &[u8]) -> std::io::Result<()> {
        self.put(path, bytes.into());
        Ok(())
    }
    fn store_file(&self, path: &str, bytes: &[u8]) -> std::io::Result<()> {
        self.write_file(path, bytes)
    }
    fn exists(&self, path: &str) -> bool {
        self.get(path).is_some()
    }
}

struct MemoryImporter {
    files: Arc<MemoryFiles>,
    pool: Arc<effectcraft_media::MediaPool>,
}
impl Importer for MemoryImporter {
    fn probe(&self, path: &str) -> Result<Footage, String> {
        let bytes = self.files.get(path).ok_or_else(|| format!("{path}: no uploaded file; native filesystem unavailable"))?;
        self.pool.add_bytes(path, bytes.clone());
        effectcraft_media::probe_bytes(path, bytes).map_err(|e| e.to_string())
    }
    fn register(&self, path: &str, bytes: &[u8]) {
        self.pool.add_bytes(path, bytes.into());
    }
}

fn js_error(e: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&e.to_string())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Frame {
    comp: Option<Value>,
    time: Option<f64>,
    #[serde(default)]
    max_side: u32,
    #[serde(default)]
    transparent: bool,
}

/// Each HTTP request owns a fresh WASM instance and one session. Call `free()` when done.
#[wasm_bindgen]
pub struct Engine {
    session: Session,
    files: Arc<MemoryFiles>,
    pool: Arc<effectcraft_media::MediaPool>,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Engine {
        let files = Arc::new(MemoryFiles::default());
        let pool = Arc::new(effectcraft_media::MediaPool::new());
        let output = files.clone();
        let session = Session {
            services: files.clone(),
            footage: pool.clone(),
            importer: Some(Arc::new(MemoryImporter { files: files.clone(), pool: pool.clone() })),
            exporter: Some(Arc::new(effectcraft_host::FileExporter {
                sink: Some(Arc::new(move |path, data| output.put(path, data.into()))),
            })),
            expr: Some(Arc::new(effectcraft_expr::Expressions)),
            expr_check: Some(effectcraft_expr::check_syntax),
            script: Some(effectcraft_script::runner),
            plugin_loader: Some(effectcraft_plugin::loader),
            script_ui: effectcraft_engine::scriptui::ScriptUi {
                dispatch: Some(effectcraft_script::dispatch_ui),
                ..Default::default()
            },
            ..Default::default()
        };
        Engine { session, files, pool }
    }

    /// All registered commands dispatch upstream, including upstream parameter validation.
    pub fn execute(&mut self, command: &str, params: &str) -> Result<String, JsValue> {
        let params: Value = serde_json::from_str(params).map_err(js_error)?;
        let result = self.session.execute_checked(command, params).map_err(js_error)?;
        self.session.poll_render();
        for event in self.session.drain_events() {
            match event {
                effectcraft_engine::Event::Frontend { command, .. } => return Err(js_error(format!("{command}: requires a desktop/browser frontend; unavailable in the headless Worker"))),
                effectcraft_engine::Event::OpenUrl(_) => return Err(js_error("opening URLs in a desktop application is unavailable in the Worker")),
                _ => {}
            }
        }
        serde_json::to_string(&result).map_err(js_error)
    }

    pub fn put_file(&mut self, path: &str, bytes: &[u8]) {
        let bytes: Arc<[u8]> = bytes.into();
        self.files.put(path, bytes.clone());
        self.pool.add_bytes(path, bytes);
    }

    pub fn get_file(&self, path: &str) -> Result<Vec<u8>, JsValue> {
        self.files.read_file(path).map_err(js_error)
    }

    pub fn files(&self) -> Result<String, JsValue> {
        let files = self.files.0.lock().unwrap_or_else(|e| e.into_inner());
        let entries: Vec<_> = files.iter().map(|(path, bytes)| json!({"path": path, "bytes": bytes.len()})).collect();
        serde_json::to_string(&entries).map_err(js_error)
    }

    pub fn load_project(&mut self, project: &str) -> Result<(), JsValue> {
        let project = effectcraft_engine::project::Project::from_json(project).map_err(js_error)?;
        self.session.replace_project(project, None);
        Ok(())
    }

    pub fn project(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.session.project).map_err(js_error)
    }

    pub fn state(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.session.state).map_err(js_error)
    }

    /// Full-resolution CPU rendering unless the caller requests a smaller preview.
    pub fn render_png(&self, options: &str) -> Result<Vec<u8>, JsValue> {
        use image::ImageEncoder;
        let options: Frame = serde_json::from_str(options).map_err(js_error)?;
        let comp = self.session.resolve_comp(options.comp.as_ref()).map_err(js_error)?;
        let time = match options.time {
            Some(t) if t.is_finite() => effectcraft_engine::time::Tick::from_seconds_f64(t),
            Some(_) => return Err(js_error("time must be finite")),
            None => self.session.time(),
        };
        let (width, height, rgba) = self.session.render_rgba8_alpha(comp, time, options.max_side, options.transparent).map_err(js_error)?;
        let mut bytes = Vec::new();
        image::codecs::png::PngEncoder::new(&mut bytes).write_image(&rgba, width, height, image::ExtendedColorType::Rgba8).map_err(js_error)?;
        Ok(bytes)
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
