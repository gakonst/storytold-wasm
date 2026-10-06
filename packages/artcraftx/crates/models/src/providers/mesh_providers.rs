//! Which providers offer which 3D mesh models: ArtCraft only.

use crate::configs::mesh_model_config::MeshModelConfig;
use crate::enums::generation_provider::GenerationProvider;
use crate::enums::mesh_model::MeshModel;
use crate::providers::provider_offering::{is_offered, providers_for_model, ProviderOffering};
use once_cell::sync::Lazy;

pub type MeshProviderOffering = ProviderOffering<MeshModel, MeshModelConfig>;

pub static MESH_PROVIDERS: Lazy<Vec<MeshProviderOffering>> = Lazy::new(mesh_providers);

pub fn providers_for_mesh_model(model: MeshModel) -> Vec<GenerationProvider> {
  providers_for_model(&MESH_PROVIDERS, model)
}

pub fn provider_offers_mesh_model(provider: GenerationProvider, model: MeshModel) -> bool {
  is_offered(&MESH_PROVIDERS, provider, model)
}

fn mesh_providers() -> Vec<MeshProviderOffering> {
  vec![
    MeshProviderOffering::of(GenerationProvider::Artcraft, &[
      MeshModel::Hunyuan3d3,
      MeshModel::Hunyuan3d3p1Pro,
      MeshModel::Hunyuan3d3p1Rapid,
      MeshModel::Hunyuan3d3Sketch,
      MeshModel::Hunyuan3d2p1,
      MeshModel::Hunyuan3d2p0,
      MeshModel::Hunyuan3d3p1Part,
      MeshModel::Hunyuan3d3p1SmartTopology,
      MeshModel::Tripo3dH3p1,
      MeshModel::MeshyV6,
      MeshModel::Rodin2p5Fast,
    ]),
  ]
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::configs::mesh_models::MESH_MODELS;
  use crate::providers::tests_common::check_offerings;

  #[test]
  fn offerings_are_consistent_with_the_model_table() {
    let known: Vec<MeshModel> = MESH_MODELS.iter().filter(|c| !c.is_disabled).map(|c| c.model).collect();
    check_offerings(&MESH_PROVIDERS, &known, |config| config.model);
    assert_eq!(MESH_PROVIDERS.len(), 1, "mesh is ArtCraft-only");
  }
}
