use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

/// Compile a ComfyUI API graph. Resource and model constraints belong to ComfyUI.
#[wasm_bindgen]
pub fn build_graph(config: &str) -> Result<String, JsError> {
    let c: Value = serde_json::from_str(config)?;
    let required = |key: &str| c.get(key).cloned().ok_or_else(|| JsError::new(&format!("missing {key}")));
    let task = c["task"].as_str().ok_or_else(|| JsError::new("task must be a string"))?;
    let mut g: Value = serde_json::from_str(include_str!("base.json"))?;
    for (node, field, key) in [("1", "unet_name", "model_file"), ("7", "steps", "steps"), ("8", "noise_seed", "seed")] {
        g[node]["inputs"][field] = required(key)?;
    }
    for (node, field, key) in [("6", "sampler_name", "sampler"), ("7", "scheduler", "scheduler"), ("15", "filename_prefix", "filename_prefix"), ("2", "clip_name", "text_encoder"), ("3", "vae_name", "video_vae"), ("4", "vae_name", "audio_vae")] {
        if let Some(v) = c.get(key) { g[node]["inputs"][field] = v.clone(); }
    }
    let mut inputs = json!({"clip": ["2",0], "vae": ["3",0]});
    for key in ["prompt", "width", "height", "length"] { inputs[key] = required(key)?; }
    let image = c.get("image").cloned().unwrap_or(json!("bench2_jp_01.jpg"));
    let class = match task {
        "t2v" => "MiniMaxH3ImageToVideo",
        "i2v" => {
            g["10"] = json!({"class_type":"LoadImage","inputs":{"image":image}});
            inputs["first_frame"] = json!(["10",0]);
            "MiniMaxH3ImageToVideo"
        },
        "ref2v" => {
            let defaults = ["bench2_jp_01.jpg", "bench2_jp_gate_01.png", "bench2_ashitaka_01.jpg", "bench2_bebop_01.jpg", "bench2_forest_01.jpg", "bench2_forest_02.jpg", "bench2_volcano_01.jpg", "bench2_volcano_02.png", "bench2_jp_02.jpg"];
            let images = if let Some(v) = c.get("images") {
                v.as_array().cloned().ok_or_else(|| JsError::new("images must be an array"))?
            } else {
                let count = c.get("ref_count").map(|v| v.as_u64().ok_or_else(|| JsError::new("ref_count must be a nonnegative integer"))).transpose()?.unwrap_or(1);
                if count <= 1 { vec![image] } else { (0..count).map(|i| json!(defaults[i as usize % defaults.len()])).collect() }
            };
            for (i, image) in images.iter().enumerate() {
                let id = if images.len() == 1 { "10".to_owned() } else { (20+i).to_string() };
                g[&id] = json!({"class_type":"LoadImage", "inputs":{"image":image}});
                let mut source = json!([id,0]);
                if let Some(ds) = c.get("ref_downscale").filter(|v| v.as_f64().unwrap_or(0.0) != 0.0) {
                    let scale = if images.len() > 40 { format!("scale_{id}") } else { (id.parse::<usize>()? + 40).to_string() };
                    g[&scale] = json!({"class_type":"ImageScaleToTotalPixels", "inputs":{"image":source,"upscale_method":"lanczos","megapixels":ds,"resolution_steps":1}});
                    source = json!([scale,0]);
                }
                inputs[format!("ref_images.ref_image_{i}")] = source;
            }
            inputs["audio_vae"] = json!(["4",0]);
            inputs["ref_image_size"] = c.get("ref_image_size").cloned().unwrap_or(json!("match"));
            "MiniMaxH3ReferenceToVideo"
        },
        _ => return Err(JsError::new("unknown task; expected t2v, i2v, or ref2v")),
    };
    g["5"] = json!({"class_type":class,"inputs":inputs});
    Ok(serde_json::to_string(&g)?)
}

/// Same ties-to-even rounding and frame lattice as the Python benchmark.
#[wasm_bindgen]
pub fn snap_length(seconds: f64) -> Result<f64, JsError> {
    if !seconds.is_finite() { return Err(JsError::new("seconds must be finite")); }
    let n = (seconds * 24.0).round_ties_even().max(5.0);
    if !n.is_finite() { return Err(JsError::new("frame count overflow")); }
    Ok(n + (5.0 - n.rem_euclid(17.0)).rem_euclid(17.0))
}
