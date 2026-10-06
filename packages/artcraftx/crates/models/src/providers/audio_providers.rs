//! Which providers offer which audio models: ArtCraft only.

use crate::configs::audio_model_config::AudioModelConfig;
use crate::enums::audio_model::AudioModel;
use crate::enums::generation_provider::GenerationProvider;
use crate::providers::provider_offering::{is_offered, providers_for_model, ProviderOffering};
use once_cell::sync::Lazy;

pub type AudioProviderOffering = ProviderOffering<AudioModel, AudioModelConfig>;

pub static AUDIO_PROVIDERS: Lazy<Vec<AudioProviderOffering>> = Lazy::new(audio_providers);

pub fn providers_for_audio_model(model: AudioModel) -> Vec<GenerationProvider> {
  providers_for_model(&AUDIO_PROVIDERS, model)
}

pub fn provider_offers_audio_model(provider: GenerationProvider, model: AudioModel) -> bool {
  is_offered(&AUDIO_PROVIDERS, provider, model)
}

fn audio_providers() -> Vec<AudioProviderOffering> {
  vec![
    AudioProviderOffering::of(GenerationProvider::Artcraft, &[
      AudioModel::SunoMusic,
      AudioModel::SunoRemix,
      AudioModel::SunoSounds,
      AudioModel::SunoSample,
      AudioModel::SeedAudio1p0,
    ]),
  ]
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::configs::audio_models::AUDIO_MODELS;
  use crate::providers::tests_common::check_offerings;

  #[test]
  fn offerings_are_consistent_with_the_model_table() {
    let known: Vec<AudioModel> = AUDIO_MODELS.iter().filter(|c| !c.is_disabled).map(|c| c.model).collect();
    check_offerings(&AUDIO_PROVIDERS, &known, |config| config.model);
    assert_eq!(AUDIO_PROVIDERS.len(), 1, "audio is ArtCraft-only");
  }
}
