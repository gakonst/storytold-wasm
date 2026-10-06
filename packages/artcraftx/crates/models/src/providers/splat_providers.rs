//! Which providers offer which Gaussian splat models.

use crate::configs::splat_model_config::SplatModelConfig;
use crate::enums::generation_provider::GenerationProvider;
use crate::enums::splat_model::SplatModel;
use crate::providers::provider_offering::{is_offered, providers_for_model, ProviderOffering};
use once_cell::sync::Lazy;

pub type SplatProviderOffering = ProviderOffering<SplatModel, SplatModelConfig>;

pub static SPLAT_PROVIDERS: Lazy<Vec<SplatProviderOffering>> = Lazy::new(splat_providers);

pub fn providers_for_splat_model(model: SplatModel) -> Vec<GenerationProvider> {
  providers_for_model(&SPLAT_PROVIDERS, model)
}

pub fn provider_offers_splat_model(provider: GenerationProvider, model: SplatModel) -> bool {
  is_offered(&SPLAT_PROVIDERS, provider, model)
}

fn splat_providers() -> Vec<SplatProviderOffering> {
  vec![
    SplatProviderOffering::of(GenerationProvider::Artcraft, &[
      SplatModel::Marble1p1,
      SplatModel::Marble1p1Plus,
      SplatModel::Marble1p0,
      SplatModel::Marble1p0Draft,
      SplatModel::TripoSplat,
    ]),
  ]
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::configs::splat_models::SPLAT_MODELS;
  use crate::providers::tests_common::check_offerings;

  #[test]
  fn offerings_are_consistent_with_the_model_table() {
    let known: Vec<SplatModel> = SPLAT_MODELS.iter().filter(|c| !c.is_disabled).map(|c| c.model).collect();
    check_offerings(&SPLAT_PROVIDERS, &known, |config| config.model);
    assert_eq!(providers_for_splat_model(SplatModel::TripoSplat), vec![GenerationProvider::Artcraft]);
  }
}
