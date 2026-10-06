use crate::{js_error, json};
use models::configs::{audio_models::AUDIO_MODELS, image_models::IMAGE_MODELS, mesh_models::MESH_MODELS, splat_models::SPLAT_MODELS, video_models::VIDEO_MODELS};
use models::providers::{audio_providers::AUDIO_PROVIDERS, image_providers::IMAGE_PROVIDERS, mesh_providers::MESH_PROVIDERS, splat_providers::SPLAT_PROVIDERS, video_providers::VIDEO_PROVIDERS};
use models::providers::provider_offering::{effective_config, is_offered, providers_for_model};
use serde_json::json;
use wasm_bindgen::JsValue;

pub fn catalog() -> Result<String, JsValue> {
    json(json!({
        "image": {"models": *IMAGE_MODELS, "providers": *IMAGE_PROVIDERS},
        "video": {"models": *VIDEO_MODELS, "providers": *VIDEO_PROVIDERS},
        "mesh": {"models": *MESH_MODELS, "providers": *MESH_PROVIDERS},
        "splat": {"models": *SPLAT_MODELS, "providers": *SPLAT_PROVIDERS},
        "audio": {"models": *AUDIO_MODELS, "providers": *AUDIO_PROVIDERS}
    }))
}

pub fn model_config(modality: &str, name: &str, provider: Option<String>) -> Result<String, JsValue> {
    let provider = provider.map(|p| serde_json::from_value::<models::enums::generation_provider::GenerationProvider>(json!(p)))
        .transpose().map_err(js_error)?;
    macro_rules! config {
        ($models:expr, $providers:expr) => {{
            // Search the authoritative tables, including disabled models. No hand-picked subset.
            let base = $models.iter().find(|c| serde_json::to_value(c.model).ok() == Some(json!(name)))
                .ok_or_else(|| js_error("unknown model"))?;
            let providers = providers_for_model(&$providers, base.model);
            if let Some(provider) = provider {
                if !is_offered(&$providers, provider, base.model) {
                    return Err(js_error("provider does not offer this model"));
                }
                json(json!({"config": effective_config(&$providers, provider, base.model, base), "providers": providers, "provider": provider}))
            } else {
                json(json!({"config": base, "providers": providers}))
            }
        }};
    }
    match modality {
        "image" => config!(IMAGE_MODELS, IMAGE_PROVIDERS),
        "video" => config!(VIDEO_MODELS, VIDEO_PROVIDERS),
        "mesh" => config!(MESH_MODELS, MESH_PROVIDERS),
        "splat" => config!(SPLAT_MODELS, SPLAT_PROVIDERS),
        "audio" => config!(AUDIO_MODELS, AUDIO_PROVIDERS),
        _ => Err(js_error("unknown modality")),
    }
}
