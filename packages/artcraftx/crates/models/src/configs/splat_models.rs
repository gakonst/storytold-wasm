//! The built-in Gaussian splat ("world") model table. Picker order = table order.

use crate::configs::splat_model_config::SplatModelConfig;
use crate::enums::model_creator::ModelCreator;
use crate::enums::splat_model::SplatModel;
use once_cell::sync::Lazy;

/// World Labs MultiImage input accepts up to this many reference images.
const MARBLE_MAX_IMAGE_REFERENCES: u16 = 4;

pub static SPLAT_MODELS: Lazy<Vec<SplatModelConfig>> = Lazy::new(splat_models);

/// Look up one model's config.
pub fn splat_model_config(model: SplatModel) -> &'static SplatModelConfig {
  SPLAT_MODELS.iter()
      .find(|config| config.model == model)
      .expect("every SplatModel variant has a config (see tests)")
}

fn splat_models() -> Vec<SplatModelConfig> {
  vec![
    marble(SplatModel::Marble1p1, "Marble 1.1", "Latest generation, best quality", "~5 min.", 300_000, false),
    marble(SplatModel::Marble1p1Plus, "Marble 1.1 Plus", "Highest quality, best for final renders", "~5 min.", 300_000, false),
    marble(SplatModel::Marble1p0, "Marble 1.0", "Previous generation, high quality", "~5 min.", 300_000, false),
    marble(SplatModel::Marble1p0Draft, "Marble 1.0 Draft", "Fast generation, good for quick drafts", "~30 sec.", 45_000, false),
    marble(SplatModel::Marble0p1Plus, "Marble Plus", "Legacy Marble 0.1 Plus", "~5 min.", 300_000, true),
    marble(SplatModel::Marble0p1Mini, "Marble Mini", "Legacy Marble 0.1 Mini", "~1 min.", 60_000, true),
    // TripoSplat reconstructs an object-scale Gaussian splat from exactly one
    // image. No prompt, video, panorama, or recaption toggle.
    SplatModelConfig {
      model: SplatModel::TripoSplat,
      model_creator: ModelCreator::Tripo,
      full_name: "TripoSplat".to_string(),
      selector_name: "TripoSplat".to_string(),
      selector_description: "Reconstructs a splat from a single image".to_string(),
      extra_info: Some("Reconstructs a 3D Gaussian splat from a single image".to_string()),
      selector_badges: vec!["~1 min.".to_string()],
      progress_bar_ms: 60_000,
      image_references_supported: true,
      image_references_max: Some(1),
      ..Default::default()
    },
  ]
}

/// All Marble models share the same capability surface: text prompt, image
/// references (up to 4, multi-view), a reference video, 360-degree panorama
/// input, and the "disable recaption" toggle.
fn marble(model: SplatModel, full_name: &str, description: &str, badge: &str, progress_bar_ms: u32, is_disabled: bool) -> SplatModelConfig {
  SplatModelConfig {
    model,
    model_creator: ModelCreator::WorldLabs,
    full_name: full_name.to_string(),
    selector_name: full_name.to_string(),
    selector_description: description.to_string(),
    selector_badges: vec![badge.to_string()],
    progress_bar_ms,
    text_prompt_supported: true,
    image_references_supported: true,
    image_references_max: Some(MARBLE_MAX_IMAGE_REFERENCES),
    video_reference_supported: true,
    panorama_supported: true,
    disable_recaption_supported: true,
    is_disabled,
    ..Default::default()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;
  use strum::IntoEnumIterator;

  #[test]
  fn every_model_has_exactly_one_config() {
    let listed: Vec<SplatModel> = SPLAT_MODELS.iter().map(|c| c.model).collect();
    let unique: HashSet<SplatModel> = listed.iter().copied().collect();
    assert_eq!(listed.len(), unique.len(), "duplicate splat model configs");
    for model in SplatModel::iter() {
      assert!(unique.contains(&model), "no config for {model:?}");
    }
  }
}
