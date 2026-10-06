//! Per-modality model configs and the built-in tables of every model.
//!
//! Each `*ModelConfig` is what the Tauri list command serves the frontend:
//! identity, capabilities (which inputs and options a model accepts), and
//! desktop presentation (picker copy, badges, providers, page flags). The
//! `*_models()` tables are the data; they're declared with
//! `..Default::default()` so an entry only spells out what's true for it.

pub mod audio_model_config;
pub mod audio_models;
pub mod image_model_config;
pub mod image_models;
pub mod mesh_model_config;
pub mod mesh_models;
pub mod splat_model_config;
pub mod splat_models;
pub mod video_model_config;
pub mod video_models;
