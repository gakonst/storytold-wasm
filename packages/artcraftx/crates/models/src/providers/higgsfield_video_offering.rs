//! The video models a first-party (cookie-session) Higgsfield account runs,
//! and how their option menus differ from the base table.
//!
//! Option sets mirror the higgsfield.ai video generator (see
//! `higgsfield_client::endpoints::generate::video`). Kling 3.0's "mode" menu
//! (std / pro / 4K) is split across the two Kling 3.0 model ids: Standard is
//! the 720p tier, Pro the 1080p / 4K tiers.

use crate::configs::video_model_config::VideoModelConfig;
use crate::configs::video_models::video_model_config;
use crate::enums::common_bitrate::CommonBitrate;
use crate::enums::common_resolution::CommonResolution;
use crate::enums::generation_provider::GenerationProvider;
use crate::enums::video_model::VideoModel;
use crate::providers::provider_offering::OfferedModel;
use crate::providers::video_providers::VideoProviderOffering;

/// Higgsfield caps every video batch at 4.
pub const HIGGSFIELD_VIDEO_BATCH_MAX: u16 = 4;

/// Every video model Higgsfield offers, in picker order.
pub const HIGGSFIELD_VIDEO_MODELS: &[VideoModel] = &[
  VideoModel::Seedance2p5,
  VideoModel::Seedance2p5Edit,
  VideoModel::Seedance2p0,
  VideoModel::Seedance2p0Mini,
  VideoModel::MinimaxH3,
  VideoModel::Kling3p0Pro,
  VideoModel::Kling3p0Standard,
  VideoModel::GrokImagineVideo1p5,
];

pub fn higgsfield_video_offering() -> VideoProviderOffering {
  VideoProviderOffering {
    provider: GenerationProvider::Higgsfield,
    models: HIGGSFIELD_VIDEO_MODELS.iter().copied().map(offered).collect(),
  }
}

/// The base config, or a Higgsfield-specific replacement when the menus
/// differ. Every override starts from the base so presentation stays in sync.
fn offered(model: VideoModel) -> OfferedModel<VideoModel, VideoModelConfig> {
  let base = video_model_config(model).clone();
  let overrides = match model {
    // Higgsfield-only models: the base config already describes them.
    VideoModel::Seedance2p5 | VideoModel::Seedance2p5Edit | VideoModel::MinimaxH3 => None,
    // Seedance 2.0: no character references; the web app defaults to the
    // high bitrate.
    VideoModel::Seedance2p0 => Some(VideoModelConfig {
      character_references_supported: false,
      character_references_max: None,
      bitrate_default: Some(CommonBitrate::High),
      ..capped(base)
    }),
    // Seedance 2.0 Mini: no character references and no bitrate menu.
    VideoModel::Seedance2p0Mini => Some(VideoModelConfig {
      character_references_supported: false,
      character_references_max: None,
      bitrate_options: Vec::new(),
      bitrate_default: None,
      ..capped(base)
    }),
    // Kling 3.0 Standard = the "std" mode: 720p only.
    VideoModel::Kling3p0Standard => Some(VideoModelConfig {
      resolution_options: vec![CommonResolution::SevenTwentyP],
      resolution_default: Some(CommonResolution::SevenTwentyP),
      ..capped(base)
    }),
    // Kling 3.0 Pro = the "pro" and "4K" modes.
    VideoModel::Kling3p0Pro => Some(VideoModelConfig {
      resolution_options: vec![CommonResolution::TenEightyP, CommonResolution::FourK],
      resolution_default: Some(CommonResolution::TenEightyP),
      ..capped(base)
    }),
    // Grok Imagine 1.5: text-to-video works, a 1080p tier exists, and image
    // references are accepted alongside the start frame.
    VideoModel::GrokImagineVideo1p5 => Some(VideoModelConfig {
      text_to_video_supported: true,
      starting_keyframe_required: false,
      image_references_supported: true,
      image_references_max: Some(4),
      resolution_options: vec![CommonResolution::FourEightyP, CommonResolution::SevenTwentyP, CommonResolution::TenEightyP],
      resolution_default: Some(CommonResolution::SevenTwentyP),
      duration_seconds_min: Some(1),
      duration_seconds_max: Some(15),
      duration_seconds_max_with_image_references: None,
      duration_seconds_options: Some((1..=15).collect()),
      duration_seconds_default: Some(6),
      ..capped(base)
    }),
    other => unreachable!("{other:?} is not a Higgsfield video model"),
  };
  match overrides {
    Some(overrides) => OfferedModel::with_overrides(model, overrides),
    None => OfferedModel::same_as_base(model),
  }
}

/// Apply the batch cap every Higgsfield video model shares.
fn capped(base: VideoModelConfig) -> VideoModelConfig {
  VideoModelConfig {
    batch_size_max: base.batch_size_max.min(HIGGSFIELD_VIDEO_BATCH_MAX),
    batch_size_options: base.batch_size_options.map(|options| {
      options.into_iter().filter(|count| *count <= HIGGSFIELD_VIDEO_BATCH_MAX).collect()
    }),
    batch_size_default: base.batch_size_default.min(HIGGSFIELD_VIDEO_BATCH_MAX),
    ..base
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::providers::provider_offering::effective_config;
  use crate::providers::video_providers::{provider_offers_video_model, providers_for_video_model, VIDEO_PROVIDERS};

  fn effective(model: VideoModel) -> VideoModelConfig {
    effective_config(&VIDEO_PROVIDERS, GenerationProvider::Higgsfield, model, video_model_config(model)).clone()
  }

  #[test]
  fn offers_exactly_the_supported_models() {
    let offering = higgsfield_video_offering();
    let offered: Vec<VideoModel> = offering.models.iter().map(|m| m.model).collect();
    assert_eq!(offered, HIGGSFIELD_VIDEO_MODELS.to_vec());
    assert!(!provider_offers_video_model(GenerationProvider::Higgsfield, VideoModel::Veo3));
    assert!(!provider_offers_video_model(GenerationProvider::Higgsfield, VideoModel::Seedance2p0Fast));
  }

  #[test]
  fn higgsfield_only_models_have_no_other_provider() {
    for model in [VideoModel::Seedance2p5, VideoModel::Seedance2p5Edit, VideoModel::MinimaxH3] {
      assert_eq!(providers_for_video_model(model), vec![GenerationProvider::Higgsfield], "{model:?}");
    }
  }

  #[test]
  fn overrides_keep_identity_and_presentation() {
    for offered in &higgsfield_video_offering().models {
      let base = video_model_config(offered.model);
      if let Some(overrides) = &offered.overrides {
        assert_eq!(overrides.model, base.model);
        assert_eq!(overrides.full_name, base.full_name);
        assert_eq!(overrides.selector_name, base.selector_name);
        assert_eq!(overrides.starting_keyframe_supported, base.starting_keyframe_supported);
      }
    }
  }

  #[test]
  fn every_default_is_within_its_override_menu() {
    for model in HIGGSFIELD_VIDEO_MODELS {
      let config = effective(*model);
      assert!(config.batch_size_max <= HIGGSFIELD_VIDEO_BATCH_MAX, "{model:?}");
      assert!(config.batch_size_default <= config.batch_size_max, "{model:?}");
      if let Some(default) = config.aspect_ratio_default {
        assert!(config.aspect_ratio_options.contains(&default), "{model:?} aspect default not offered");
      }
      if let Some(default) = config.resolution_default {
        assert!(config.resolution_options.contains(&default), "{model:?} resolution default not offered");
      }
      if let Some(default) = config.bitrate_default {
        assert!(config.bitrate_options.contains(&default), "{model:?} bitrate default not offered");
      }
      if let (Some(default), Some(options)) = (config.duration_seconds_default, &config.duration_seconds_options) {
        assert!(options.contains(&default), "{model:?} duration default not offered");
      }
      if let (Some(min), Some(max), Some(options)) = (config.duration_seconds_min, config.duration_seconds_max, &config.duration_seconds_options) {
        assert_eq!(options.first().copied(), Some(min), "{model:?} duration options start below min");
        assert_eq!(options.last().copied(), Some(max), "{model:?} duration options end above max");
      }
      if !config.starting_keyframe_supported {
        assert!(!config.starting_keyframe_required, "{model:?}");
      }
    }
  }

  #[test]
  fn kling_3p0_splits_the_mode_menu_by_model() {
    assert_eq!(effective(VideoModel::Kling3p0Standard).resolution_options, vec![CommonResolution::SevenTwentyP]);
    assert_eq!(effective(VideoModel::Kling3p0Pro).resolution_options, vec![CommonResolution::TenEightyP, CommonResolution::FourK]);
    // The base (ArtCraft) Kling 3.0 has no resolution menu at all.
    assert!(video_model_config(VideoModel::Kling3p0Pro).resolution_options.is_empty());
  }

  #[test]
  fn seedance_2p0_drops_characters_and_mini_drops_bitrate() {
    assert!(video_model_config(VideoModel::Seedance2p0).character_references_supported);
    assert!(!effective(VideoModel::Seedance2p0).character_references_supported);
    assert!(!effective(VideoModel::Seedance2p0Mini).character_references_supported);
    assert!(effective(VideoModel::Seedance2p0Mini).bitrate_options.is_empty());
    assert_eq!(effective(VideoModel::Seedance2p0).resolution_options, video_model_config(VideoModel::Seedance2p0).resolution_options);
  }

  #[test]
  fn grok_imagine_1p5_gains_text_to_video_and_1080p() {
    let base = video_model_config(VideoModel::GrokImagineVideo1p5);
    let higgsfield = effective(VideoModel::GrokImagineVideo1p5);
    assert!(!base.text_to_video_supported);
    assert!(higgsfield.text_to_video_supported);
    assert!(!higgsfield.starting_keyframe_required);
    assert!(higgsfield.resolution_options.contains(&CommonResolution::TenEightyP));
    assert!(higgsfield.image_references_supported);
  }

  #[test]
  fn seedance_2p5_runs_longer_than_2p0_but_without_4k() {
    let seedance_2p5 = effective(VideoModel::Seedance2p5);
    assert_eq!(seedance_2p5.duration_seconds_max, Some(30));
    assert!(!seedance_2p5.resolution_options.contains(&CommonResolution::FourK));
    assert!(seedance_2p5.video_references_supported && seedance_2p5.audio_references_supported);
    let edit = effective(VideoModel::Seedance2p5Edit);
    assert!(!edit.text_to_video_supported);
    assert_eq!(edit.video_references_max, Some(1));
    assert!(edit.duration_seconds_options.is_none());
  }
}
