//! Headless byte-oriented bindings. No DOM, filesystem, GPU or browser storage required.
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod capabilities;
use lightcraft_engine::{Session, catalog::{MemStore, PhotoId}, export::{ExportOptions, export_photo}, library::LibraryStores};
use serde_json::{Value, json};
use std::{collections::BTreeMap, sync::{Arc, Mutex}};
use wasm_bindgen::prelude::*;

type Originals = Arc<Mutex<BTreeMap<String, Vec<u8>>>>;
fn err(e: impl std::fmt::Display) -> JsValue { JsValue::from_str(&e.to_string()) }
fn json_text(v: &impl serde::Serialize) -> Result<String, JsValue> { serde_json::to_string(v).map_err(err) }
fn read(files: &Originals, path: &str) -> Result<Vec<u8>, String> {
    files.lock().unwrap_or_else(|e| e.into_inner()).get(path).cloned().ok_or_else(|| format!("original not uploaded: {path}"))
}

#[wasm_bindgen]
pub struct LightCraft {
    session: Session,
    files: MemStore,
    originals: Originals,
    last_export: Value,
}

#[wasm_bindgen]
impl LightCraft {
    /// `snapshot` is the output of snapshot(); empty starts a new library.
    #[wasm_bindgen(constructor)]
    pub fn new(snapshot: &str, demo: bool) -> Result<LightCraft, JsValue> {
        let files = MemStore::new();
        if !snapshot.is_empty() {
            let saved: BTreeMap<String, Vec<u8>> = serde_json::from_str(snapshot).map_err(err)?;
            for (name, bytes) in saved { files.set(&name, bytes); }
        }
        let mut session = Session::new();
        session.clock = Box::new(|| {
            #[cfg(target_arch = "wasm32")]
            { String::from(js_sys::Date::new_0().to_iso_string()) }
            #[cfg(not(target_arch = "wasm32"))]
            { lightcraft_engine::import::system_clock() }
        });
        session.open_library_in(LibraryStores { dir: "worker".into(), catalog: Box::new(files.clone()), files: Box::new(files.clone()), on_disk: false }, demo).map_err(err)?;
        // There are no sidecar files on this host. Explicit byte downloads are available.
        session.xmp.auto_write = false;
        let originals: Originals = Arc::default();
        let bytes = originals.clone();
        session.media.file_loader = Some(Arc::new(move |path, edge| lightcraft_engine::files::load_bytes(&read(&bytes, path)?, edge)));
        let bytes = originals.clone();
        session.media.file_probe = Some(Arc::new(move |path| lightcraft_engine::files::probe_bytes(path, &read(&bytes, path)?)));
        let bytes = originals.clone();
        session.media.file_bytes = Some(Arc::new(move |path| read(&bytes, path)));
        let bytes = originals.clone();
        session.media.availability.set_probe(Arc::new(move |path| bytes.lock().unwrap_or_else(|e| e.into_inner()).contains_key(path)));
        Ok(Self { session, files, originals, last_export: Value::Null })
    }

    /// Upload bytes, returning the stable virtual path. Import via library.import or import().
    pub fn upload(&mut self, name: &str, bytes: &[u8]) -> Result<String, JsValue> {
        let name = name.rsplit(['/', '\\']).next().filter(|s| !s.is_empty()).ok_or_else(|| err("empty file name"))?;
        let path = format!("worker/{}/{name}", lightcraft_preview::hash_bytes(bytes));
        self.originals.lock().unwrap_or_else(|e| e.into_inner()).insert(path.clone(), bytes.to_vec());
        Ok(path)
    }

    pub fn import(&mut self, name: &str, bytes: &[u8], options: &str) -> Result<String, JsValue> {
        let path = self.upload(name, bytes)?;
        let mut p: Value = serde_json::from_str(options).map_err(err)?;
        let p = p.as_object_mut().ok_or_else(|| err("import options must be an object"))?;
        p.insert("paths".into(), json!([path]));
        self.execute("library.import", &json_text(p)?)
    }

    pub fn commands(&self) -> Result<String, JsValue> {
        let commands: Vec<_> = self.session.commands().into_iter().map(|mut c| {
            if let Some(reason) = capabilities::unavailable(c.id) { c.enabled = false; c.disabled_reason = Some(reason.into()); }
            c
        }).collect();
        json_text(&commands)
    }

    pub fn execute(&mut self, id: &str, params: &str) -> Result<String, JsValue> {
        let p: Value = serde_json::from_str(params).map_err(err)?;
        if !p.is_object() { return Err(err("command parameters must be an object")); }
        capabilities::check(id, &p).map_err(err)?;
        json_text(&self.session.execute(id, &p).map_err(err)?)
    }

    /// Engine export, including high-depth PNG/TIFF, original bytes and raw-to-DNG.
    pub fn export(&mut self, id: u64, options: &str) -> Result<Vec<u8>, JsValue> {
        let p: Value = serde_json::from_str(options).map_err(err)?;
        if !p.is_object() { return Err(err("export options must be an object")); }
        if let Some(format) = p.get("format") {
            let name = format.as_str().ok_or_else(|| err("format must be a string"))?;
            if lightcraft_engine::export::ExportFormat::parse(name).is_none() { return Err(err(format!("unsupported export format: {name}"))); }
        }
        if p.pointer("/watermark/path").is_some() || p.pointer("/watermark/image").is_some() {
            return Err(err("filesystem watermark unavailable; text watermarks are supported"));
        }
        let o = ExportOptions::from_json(&p);
        let id = if id == 0 { self.session.active().ok_or_else(|| err("no active photo"))? } else { PhotoId(id) };
        let out = export_photo(&mut self.session, id, &o, 1).map_err(err)?;
        self.last_export = json!({"fileName": out.file_name, "width": out.width, "height": out.height, "sidecars": out.sidecars});
        Ok(out.bytes)
    }

    pub fn export_info(&self) -> Result<String, JsValue> { json_text(&self.last_export) }

    /// Full-resolution HDR/panorama output as DNG bytes; never writes a native file.
    pub fn merge(&mut self, id: &str, params: &str) -> Result<Vec<u8>, JsValue> {
        let p: Value = serde_json::from_str(params).map_err(err)?;
        let (kind, finish) = lightcraft_engine::merge::parse(id, &p).map_err(err)?;
        let job = self.session.plan_merge(kind, finish, &self.session.targets(&p), false).map_err(err)?;
        let out = job.run(&|_, _| true).map_err(err)?;
        self.last_export = out.info;
        Ok(out.dng)
    }

    /// Engine library files, ready for client storage. Uploaded originals are transported separately.
    pub fn snapshot(&mut self) -> Result<String, JsValue> {
        self.session.persist().map_err(err)?;
        self.session.save_view();
        json_text(&*self.files.files.lock().unwrap_or_else(|e| e.into_inner()))
    }
}

#[wasm_bindgen]
pub fn metadata(bytes: &[u8]) -> Result<String, JsValue> {
    json_text(&lightcraft_meta::extract(bytes))
}
