//! The image models a first-party (cookie-session) Higgsfield account runs,
//! and how their option menus differ from the base table.
//!
//! Option sets mirror the higgsfield.ai image generator (read off the web app
//! on 2026-08-31, see `higgsfield_client::endpoints::generate::image`). Where
//! Higgsfield offers fewer choices than ArtCraft the override narrows the
//! menu so the picker only shows what the account can actually run, and the
//! router snaps anything else to the nearest tier.

use crate::configs::image_model_config::ImageModelConfig;
use crate::configs::image_models::image_model_config;
use crate::enums::common_aspect_ratio::CommonAspectRatio;
use crate::enums::common_resolution::CommonResolution;
use crate::enums::generation_provider::GenerationProvider;
use crate::enums::image_model::ImageModel;
use crate::providers::image_providers::ImageProviderOffering;
use crate::providers::provider_offering::OfferedModel;

/// Higgsfield caps every image batch at 4.
pub const HIGGSFIELD_IMAGE_BATCH_MAX: u16 = 4;

/// Higgsfield's Nano Banana Pro / 2 / 2 Lite and GPT Image 2 aspect menu
/// (Nano Banana adds 5:4 and 4:5).
const NANO_BANANA_ASPECT_RATIOS: &[CommonAspectRatio] = &[
  CommonAspectRatio::Auto,
  CommonAspectRatio::Square,
  CommonAspectRatio::TallThreeByFour,
  CommonAspectRatio::WideFourByThree,
  CommonAspectRatio::TallTwoByThree,
  CommonAspectRatio::WideThreeByTwo,
  CommonAspectRatio::TallNineBySixteen,
  CommonAspectRatio::WideSixteenByNine,
  CommonAspectRatio::WideFiveByFour,
  CommonAspectRatio::TallFourByFive,
  CommonAspectRatio::WideTwentyOneByNine,
];

const GPT_IMAGE_2_ASPECT_RATIOS: &[CommonAspectRatio] = &[
  CommonAspectRatio::Auto,
  CommonAspectRatio::Square,
  CommonAspectRatio::WideThreeByTwo,
  CommonAspectRatio::TallTwoByThree,
  CommonAspectRatio::WideSixteenByNine,
  CommonAspectRatio::TallNineBySixteen,
  CommonAspectRatio::WideFourByThree,
  CommonAspectRatio::TallThreeByFour,
  CommonAspectRatio::WideTwentyOneByNine,
];

/// Higgsfield's Seedream menu: no Auto and no baked-in-resolution values.
const SEEDREAM_ASPECT_RATIOS: &[CommonAspectRatio] = &[
  CommonAspectRatio::Square,
  CommonAspectRatio::WideFourByThree,
  CommonAspectRatio::TallThreeByFour,
  CommonAspectRatio::WideSixteenByNine,
  CommonAspectRatio::WideTwentyOneByNine,
  CommonAspectRatio::TallNineBySixteen,
  CommonAspectRatio::TallTwoByThree,
  CommonAspectRatio::WideThreeByTwo,
];

/// Every image model Higgsfield offers, in picker order.
pub const HIGGSFIELD_IMAGE_MODELS: &[ImageModel] = &[
  ImageModel::NanoBananaPro,
  ImageModel::NanoBanana2,
  ImageModel::NanoBanana2Lite,
  ImageModel::GptImage2,
  ImageModel::Seedream5p0Pro,
  ImageModel::Seedream5Lite,
  ImageModel::Seedream4p5,
];

pub fn higgsfield_image_offering() -> ImageProviderOffering {
  ImageProviderOffering {
    provider: GenerationProvider::Higgsfield,
    models: HIGGSFIELD_IMAGE_MODELS.iter().copied().map(offered).collect(),
  }
}

/// The base config, or a Higgsfield-specific replacement when the menus
/// differ. Every override starts from the base so presentation stays in sync.
fn offered(model: ImageModel) -> OfferedModel<ImageModel, ImageModelConfig> {
  let base = image_model_config(model).clone();
  let overrides = match model {
    // Nano Banana Pro: the same 11 ratios, 1K / 2K / 4K, up to 4 images.
    ImageModel::NanoBananaPro => None,
    // Nano Banana 2: no 0.5K tier on Higgsfield.
    ImageModel::NanoBanana2 => Some(ImageModelConfig {
      aspect_ratio_options: NANO_BANANA_ASPECT_RATIOS.to_vec(),
      resolution_options: vec![CommonResolution::OneK, CommonResolution::TwoK, CommonResolution::FourK],
      ..capped(base)
    }),
    // Higgsfield-only: the base config already describes it.
    ImageModel::NanoBanana2Lite => None,
    // GPT Image 2: no "1:1 HD", no 3K.
    ImageModel::GptImage2 => Some(ImageModelConfig {
      aspect_ratio_options: GPT_IMAGE_2_ASPECT_RATIOS.to_vec(),
      resolution_options: vec![CommonResolution::OneK, CommonResolution::TwoK, CommonResolution::FourK],
      ..capped(base)
    }),
    // Seedream 5.0 Pro: no Auto ratio; 1K / 2K (Higgsfield's 1.5K has no
    // common tier); batches of 4, not 8.
    ImageModel::Seedream5p0Pro => Some(ImageModelConfig {
      aspect_ratio_options: SEEDREAM_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::Square),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Square),
      resolution_options: vec![CommonResolution::OneK, CommonResolution::TwoK],
      resolution_default: Some(CommonResolution::OneK),
      ..capped(base)
    }),
    // Seedream 5 Lite: a real resolution menu (2K / 3K / 4K) instead of the
    // baked-in "auto_2k" style ratios.
    ImageModel::Seedream5Lite => Some(ImageModelConfig {
      aspect_ratio_options: SEEDREAM_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::WideSixteenByNine),
      resolution_options: vec![CommonResolution::TwoK, CommonResolution::ThreeK, CommonResolution::FourK],
      resolution_default: Some(CommonResolution::TwoK),
      ..capped(base)
    }),
    // Seedream 4.5: likewise, 2K / 4K.
    ImageModel::Seedream4p5 => Some(ImageModelConfig {
      aspect_ratio_options: SEEDREAM_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::WideSixteenByNine),
      resolution_options: vec![CommonResolution::TwoK, CommonResolution::FourK],
      resolution_default: Some(CommonResolution::TwoK),
      ..capped(base)
    }),
    other => unreachable!("{other:?} is not a Higgsfield image model"),
  };
  match overrides {
    Some(overrides) => OfferedModel::with_overrides(model, overrides),
    None => OfferedModel::same_as_base(model),
  }
}

/// Apply the batch cap every Higgsfield image model shares.
fn capped(base: ImageModelConfig) -> ImageModelConfig {
  ImageModelConfig {
    batch_size_max: HIGGSFIELD_IMAGE_BATCH_MAX,
    batch_size_options: None,
    batch_size_default: base.batch_size_default.min(HIGGSFIELD_IMAGE_BATCH_MAX),
    ..base
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::providers::image_providers::{provider_offers_image_model, providers_for_image_model, IMAGE_PROVIDERS};
  use crate::providers::provider_offering::effective_config;

  fn effective(model: ImageModel) -> ImageModelConfig {
    effective_config(&IMAGE_PROVIDERS, GenerationProvider::Higgsfield, model, image_model_config(model)).clone()
  }

  #[test]
  fn offers_exactly_the_supported_models() {
    let offering = higgsfield_image_offering();
    let offered: Vec<ImageModel> = offering.models.iter().map(|m| m.model).collect();
    assert_eq!(offered, HIGGSFIELD_IMAGE_MODELS.to_vec());
    assert!(!provider_offers_image_model(GenerationProvider::Higgsfield, ImageModel::Flux1Dev));
    assert!(!provider_offers_image_model(GenerationProvider::Higgsfield, ImageModel::Midjourney8));
  }

  #[test]
  fn nano_banana_2_lite_is_higgsfield_only() {
    assert_eq!(providers_for_image_model(ImageModel::NanoBanana2Lite), vec![GenerationProvider::Higgsfield]);
  }

  #[test]
  fn overrides_keep_identity_and_presentation() {
    for offered in &higgsfield_image_offering().models {
      let base = image_model_config(offered.model);
      if let Some(overrides) = &offered.overrides {
        assert_eq!(overrides.model, base.model);
        assert_eq!(overrides.full_name, base.full_name);
        assert_eq!(overrides.selector_name, base.selector_name);
        assert_eq!(overrides.can_edit_images, base.can_edit_images);
        assert_eq!(overrides.image_refs_supported, base.image_refs_supported);
      }
    }
  }

  #[test]
  fn every_default_is_within_its_override_menu() {
    for model in HIGGSFIELD_IMAGE_MODELS {
      let config = effective(*model);
      assert!(config.batch_size_max <= HIGGSFIELD_IMAGE_BATCH_MAX, "{model:?}");
      assert!(config.batch_size_default <= config.batch_size_max, "{model:?}");
      if let Some(default) = config.aspect_ratio_default {
        assert!(config.aspect_ratio_options.contains(&default), "{model:?} aspect default not offered");
      }
      if let Some(default) = config.aspect_ratio_default_when_editing {
        assert!(config.aspect_ratio_options.contains(&default), "{model:?} editing aspect default not offered");
      }
      if let Some(default) = config.resolution_default {
        assert!(config.resolution_options.contains(&default), "{model:?} resolution default not offered");
      }
      if let Some(default) = config.quality_default {
        assert!(config.quality_options.contains(&default), "{model:?} quality default not offered");
      }
    }
  }

  #[test]
  fn menus_narrow_to_what_higgsfield_runs() {
    // 0.5K is an ArtCraft-only tier for Nano Banana 2.
    assert!(image_model_config(ImageModel::NanoBanana2).resolution_options.contains(&CommonResolution::HalfK));
    assert!(!effective(ImageModel::NanoBanana2).resolution_options.contains(&CommonResolution::HalfK));
    // GPT Image 2 has no 3K and no "1:1 HD" on Higgsfield.
    let gpt = effective(ImageModel::GptImage2);
    assert!(!gpt.resolution_options.contains(&CommonResolution::ThreeK));
    assert!(!gpt.aspect_ratio_options.contains(&CommonAspectRatio::SquareHd));
    assert!(gpt.aspect_ratio_options.contains(&CommonAspectRatio::WideTwentyOneByNine));
    // Seedream 5.0 Pro: batches of 4, no Auto.
    let pro = effective(ImageModel::Seedream5p0Pro);
    assert_eq!(pro.batch_size_max, 4);
    assert!(!pro.aspect_ratio_options.contains(&CommonAspectRatio::Auto));
    // Seedream 5 Lite / 4.5 gain a real resolution menu.
    assert_eq!(effective(ImageModel::Seedream5Lite).resolution_options, vec![CommonResolution::TwoK, CommonResolution::ThreeK, CommonResolution::FourK]);
    assert_eq!(effective(ImageModel::Seedream4p5).resolution_options, vec![CommonResolution::TwoK, CommonResolution::FourK]);
    // Nano Banana Pro is identical to the base config.
    assert!(higgsfield_image_offering().overrides_for(ImageModel::NanoBananaPro).is_none());
  }

  #[test]
  fn effective_config_falls_back_to_base_for_other_providers() {
    let base = image_model_config(ImageModel::NanoBanana2);
    let artcraft = effective_config(&IMAGE_PROVIDERS, GenerationProvider::Artcraft, ImageModel::NanoBanana2, base);
    assert_eq!(artcraft, base);
  }
}
