//! The built-in video model table, grouped by family. Picker order = table order.

use crate::configs::video_model_config::VideoModelConfig;
use crate::enums::video_model::VideoModel;
use once_cell::sync::Lazy;

mod by_type;

pub static VIDEO_MODELS: Lazy<Vec<VideoModelConfig>> = Lazy::new(video_models);

/// Look up one model's config.
pub fn video_model_config(model: VideoModel) -> &'static VideoModelConfig {
  VIDEO_MODELS.iter()
      .find(|config| config.model == model)
      .expect("every VideoModel variant has a config (see tests)")
}

fn video_models() -> Vec<VideoModelConfig> {
  let mut models = Vec::new();

  models.extend(by_type::seedance_video_models::seedance_video_models());
  models.extend(by_type::kling_video_models::kling_video_models());
  models.extend(by_type::veo_video_models::veo_video_models());
  models.extend(by_type::vidu_video_models::vidu_video_models());
  models.extend(by_type::happy_horse_video_models::happy_horse_video_models());
  models.extend(by_type::minimax_video_models::minimax_video_models());
  models.extend(by_type::sora_video_models::sora_video_models());
  models.extend(by_type::grok_video_models::grok_video_models());
  models.extend(by_type::beeble_video_models::beeble_video_models());
  models.extend(by_type::preview_video_models::preview_video_models());

  models
}

pub(crate) fn strings(items: &[&str]) -> Vec<String> {
  items.iter().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;
  use strum::IntoEnumIterator;

  #[test]
  fn every_model_has_exactly_one_config() {
    let listed: Vec<VideoModel> = VIDEO_MODELS.iter().map(|c| c.model).collect();
    let unique: HashSet<VideoModel> = listed.iter().copied().collect();
    assert_eq!(listed.len(), unique.len(), "duplicate video model configs");
    for model in VideoModel::iter() {
      assert!(unique.contains(&model), "no config for {model:?}");
    }
  }

  #[test]
  fn defaults_are_within_their_options() {
    for config in VIDEO_MODELS.iter() {
      assert!(!config.full_name.is_empty() && !config.selector_name.is_empty(), "{:?} needs names", config.model);
      assert!(
        config.aspect_ratio_options.is_empty() || config.legacy_size_options.is_empty(),
        "{:?} must use either aspect ratios or legacy sizes, not both", config.model,
      );
      if let Some(default) = config.aspect_ratio_default {
        assert!(config.aspect_ratio_options.contains(&default), "{:?} aspect default not offered", config.model);
      }
      if let Some(default) = config.resolution_default {
        assert!(config.resolution_options.contains(&default), "{:?} resolution default not offered", config.model);
      }
      if let (Some(default), Some(options)) = (config.duration_seconds_default, &config.duration_seconds_options) {
        assert!(options.contains(&default), "{:?} duration default not offered", config.model);
      }
      if let (Some(default), Some(min), Some(max)) = (config.duration_seconds_default, config.duration_seconds_min, config.duration_seconds_max) {
        assert!(min <= default && default <= max, "{:?} duration default out of range", config.model);
      }
      if config.starting_keyframe_required {
        assert!(config.starting_keyframe_supported, "{:?} requires a keyframe it doesn't support", config.model);
      }
    }
  }

  #[test]
  fn serializes_with_string_ids() {
    let json = serde_json::to_value(video_model_config(VideoModel::GrokImagineVideo)).unwrap();
    assert_eq!(json["model"], "grok_imagine_video");
    assert_eq!(json["legacy_size_options"], serde_json::json!(["landscape", "portrait", "square"]));
  }
}
