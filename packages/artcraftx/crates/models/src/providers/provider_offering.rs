use crate::enums::generation_provider::GenerationProvider;
use serde_derive::Serialize;

/// The models one provider offers.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProviderOffering<Model, Config> {
  pub provider: GenerationProvider,
  /// In the provider's display order.
  pub models: Vec<OfferedModel<Model, Config>>,
}

/// One model as offered by one provider.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OfferedModel<Model, Config> {
  pub model: Model,
  /// When set, this provider's version of the model differs from the base
  /// config and this replaces it. `None` = same as the base config.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub overrides: Option<Config>,
}

impl<Model, Config> OfferedModel<Model, Config> {
  pub fn same_as_base(model: Model) -> Self {
    Self { model, overrides: None }
  }

  /// This provider runs the model with a different capability surface than
  /// the base config (fewer resolutions, other durations, ...).
  pub fn with_overrides(model: Model, overrides: Config) -> Self {
    Self { model, overrides: Some(overrides) }
  }
}

impl<Model: Copy + PartialEq, Config> ProviderOffering<Model, Config> {
  /// Every model this provider offers (base config, no overrides).
  pub fn of(provider: GenerationProvider, models: &[Model]) -> Self {
    Self {
      provider,
      models: models.iter().copied().map(OfferedModel::same_as_base).collect(),
    }
  }

  pub fn offers(&self, model: Model) -> bool {
    self.models.iter().any(|offered| offered.model == model)
  }

  /// This provider's replacement config for `model`, if it has one.
  pub fn overrides_for(&self, model: Model) -> Option<&Config> {
    self.models.iter()
        .find(|offered| offered.model == model)
        .and_then(|offered| offered.overrides.as_ref())
  }
}

/// The config `provider` runs `model` with: its override when the offering
/// carries one, else `base`. Callers that plan a request against a specific
/// provider should read options from this, not from the base table.
pub fn effective_config<'a, Model: Copy + PartialEq, Config>(
  offerings: &'a [ProviderOffering<Model, Config>],
  provider: GenerationProvider,
  model: Model,
  base: &'a Config,
) -> &'a Config {
  offerings.iter()
      .find(|offering| offering.provider == provider)
      .and_then(|offering| offering.overrides_for(model))
      .unwrap_or(base)
}

/// The providers that offer `model`, in table order (the first is the
/// default provider for the model).
pub fn providers_for_model<Model: Copy + PartialEq, Config>(
  offerings: &[ProviderOffering<Model, Config>],
  model: Model,
) -> Vec<GenerationProvider> {
  offerings.iter()
      .filter(|offering| offering.offers(model))
      .map(|offering| offering.provider)
      .collect()
}

/// Whether `provider` offers `model`.
pub fn is_offered<Model: Copy + PartialEq, Config>(
  offerings: &[ProviderOffering<Model, Config>],
  provider: GenerationProvider,
  model: Model,
) -> bool {
  offerings.iter().any(|offering| offering.provider == provider && offering.offers(model))
}
