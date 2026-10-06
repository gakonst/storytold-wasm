//! Which providers offer which image models.

use crate::configs::image_model_config::ImageModelConfig;
use crate::enums::generation_provider::GenerationProvider;
use crate::enums::image_model::ImageModel;
use crate::providers::higgsfield_image_offering::higgsfield_image_offering;
use crate::providers::provider_offering::{is_offered, providers_for_model, ProviderOffering};
use once_cell::sync::Lazy;

pub type ImageProviderOffering = ProviderOffering<ImageModel, ImageModelConfig>;

pub static IMAGE_PROVIDERS: Lazy<Vec<ImageProviderOffering>> = Lazy::new(image_providers);

pub fn providers_for_image_model(model: ImageModel) -> Vec<GenerationProvider> {
  providers_for_model(&IMAGE_PROVIDERS, model)
}

pub fn provider_offers_image_model(provider: GenerationProvider, model: ImageModel) -> bool {
  is_offered(&IMAGE_PROVIDERS, provider, model)
}

fn image_providers() -> Vec<ImageProviderOffering> {
  vec![
    // ArtCraft (storyteller-web) runs everything except the first-party Grok
    // Imagine models.
    ImageProviderOffering::of(GenerationProvider::Artcraft, &[
      ImageModel::NanoBananaPro,
      ImageModel::NanoBanana2,
      ImageModel::NanoBanana,
      ImageModel::GptImage2,
      ImageModel::GptImage1p5,
      ImageModel::GptImage1,
      ImageModel::Seedream5p0Pro,
      ImageModel::Seedream5p0ProUltra,
      ImageModel::Seedream5Lite,
      ImageModel::Seedream4p5,
      ImageModel::Seedream4,
      ImageModel::FluxPro11Ultra,
      ImageModel::FluxPro11,
      ImageModel::Flux1Dev,
      ImageModel::Flux1Schnell,
      ImageModel::Midjourney8,
      ImageModel::Midjourney7,
      ImageModel::Midjourney7Niji,
      ImageModel::QwenEdit2511Angles,
      ImageModel::Flux2LoraAngles,
      ImageModel::FluxProKontextMax,
      ImageModel::FluxPro1,
      ImageModel::FluxDevJuggernaut,
    ]),
    // FAL (API key).
    ImageProviderOffering::of(GenerationProvider::Fal, &[
      ImageModel::NanoBananaPro,
      ImageModel::NanoBanana2,
      ImageModel::Flux1Dev,
      ImageModel::Flux1Schnell,
    ]),
    // First-party (cookie-session) Grok: the imagine websocket only.
    ImageProviderOffering::of(GenerationProvider::Grok, &[
      ImageModel::GrokImagineImage,
      ImageModel::GrokImagineImageQuality,
    ]),
    // First-party (cookie-session) Midjourney: Midjourney models only.
    ImageProviderOffering::of(GenerationProvider::Midjourney, &[
      ImageModel::Midjourney8,
      ImageModel::Midjourney7,
      ImageModel::Midjourney7Niji,
    ]),
    // First-party (cookie-session) Higgsfield, with per-model overrides where
    // its menus differ from ArtCraft's.
    higgsfield_image_offering(),
  ]
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::configs::image_models::IMAGE_MODELS;
  use crate::providers::tests_common::check_offerings;

  #[test]
  fn offerings_are_consistent_with_the_model_table() {
    let known: Vec<ImageModel> = IMAGE_MODELS.iter().filter(|c| !c.is_disabled).map(|c| c.model).collect();
    check_offerings(&IMAGE_PROVIDERS, &known, |config| config.model);
  }

  #[test]
  fn first_party_providers_are_exclusive() {
    assert_eq!(providers_for_image_model(ImageModel::GrokImagineImage), vec![GenerationProvider::Grok]);
    assert_eq!(providers_for_image_model(ImageModel::GrokImagineImageQuality), vec![GenerationProvider::Grok]);
    assert!(!provider_offers_image_model(GenerationProvider::Artcraft, ImageModel::GrokImagineImage));
    assert!(!provider_offers_image_model(GenerationProvider::Midjourney, ImageModel::GrokImagineImage));
    assert_eq!(providers_for_image_model(ImageModel::Midjourney7), vec![GenerationProvider::Artcraft, GenerationProvider::Midjourney]);
    assert!(!provider_offers_image_model(GenerationProvider::Grok, ImageModel::Midjourney7));
    // ArtCraft is the default provider for the shared models.
    assert_eq!(providers_for_image_model(ImageModel::Flux1Dev)[0], GenerationProvider::Artcraft);
    assert_eq!(providers_for_image_model(ImageModel::NanoBananaPro), vec![GenerationProvider::Artcraft, GenerationProvider::Fal, GenerationProvider::Higgsfield]);
  }
}
