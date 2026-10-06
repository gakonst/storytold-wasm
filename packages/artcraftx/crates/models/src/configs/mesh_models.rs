//! The built-in 3D mesh model table. Picker order = table order.

use crate::configs::mesh_model_config::MeshModelConfig;
use crate::enums::common_mesh_output_type::CommonMeshOutputType;
use crate::enums::common_polygon_type::CommonPolygonType;
use crate::enums::mesh_model::MeshModel;
use crate::enums::model_creator::ModelCreator;
use once_cell::sync::Lazy;

pub static MESH_MODELS: Lazy<Vec<MeshModelConfig>> = Lazy::new(mesh_models);

/// Look up one model's config.
pub fn mesh_model_config(model: MeshModel) -> &'static MeshModelConfig {
  MESH_MODELS.iter()
      .find(|config| config.model == model)
      .expect("every MeshModel variant has a config (see tests)")
}

const NORMAL_AND_GEOMETRY: &[CommonMeshOutputType] = &[CommonMeshOutputType::Normal, CommonMeshOutputType::Geometry];
const NORMAL_LOW_POLY_AND_GEOMETRY: &[CommonMeshOutputType] = &[
  CommonMeshOutputType::Normal,
  CommonMeshOutputType::LowPoly,
  CommonMeshOutputType::Geometry,
];
const TRIANGLE_AND_QUAD: &[CommonPolygonType] = &[CommonPolygonType::Triangle, CommonPolygonType::Quad];

fn strings(items: &[&str]) -> Vec<String> {
  items.iter().map(|s| s.to_string()).collect()
}

fn mesh_models() -> Vec<MeshModelConfig> {
  vec![
    // Text and/or image input with multi-view support and full output shaping
    // controls (output type, polygon type, face count, PBR).
    MeshModelConfig {
      model: MeshModel::Hunyuan3d3,
      model_creator: ModelCreator::Tencent,
      full_name: "Hunyuan 3D 3.0".to_string(),
      selector_name: "Hunyuan 3.0".to_string(),
      selector_description: "Highest quality 3D mesh generation".to_string(),
      selector_badges: strings(&["~2 min."]),
      progress_bar_ms: 120_000,
      text_prompt_supported: true,
      image_input_supported: true,
      multi_view_supported: true,
      mesh_output_types: NORMAL_LOW_POLY_AND_GEOMETRY.to_vec(),
      polygon_types: TRIANGLE_AND_QUAD.to_vec(),
      face_count_supported: true,
      pbr_supported: true,
      ..Default::default()
    },
    // Text or (multi-view) image input. No low-poly mode or polygon type
    // selection (unlike v3).
    MeshModelConfig {
      model: MeshModel::Hunyuan3d3p1Pro,
      model_creator: ModelCreator::Tencent,
      full_name: "Hunyuan 3D 3.1 Pro".to_string(),
      selector_name: "Hunyuan 3.1 Pro".to_string(),
      selector_description: "Newest Hunyuan, highest quality".to_string(),
      selector_badges: strings(&["~2 min."]),
      progress_bar_ms: 120_000,
      text_prompt_supported: true,
      image_input_supported: true,
      multi_view_supported: true,
      mesh_output_types: NORMAL_AND_GEOMETRY.to_vec(),
      face_count_supported: true,
      pbr_supported: true,
      ..Default::default()
    },
    // The fast, low-cost tier. Text or single-image input, minimal options.
    MeshModelConfig {
      model: MeshModel::Hunyuan3d3p1Rapid,
      model_creator: ModelCreator::Tencent,
      full_name: "Hunyuan 3D 3.1 Rapid".to_string(),
      selector_name: "Hunyuan 3.1 Rapid".to_string(),
      selector_description: "Newest Hunyuan, fast".to_string(),
      selector_badges: strings(&["~1 min."]),
      progress_bar_ms: 60_000,
      text_prompt_supported: true,
      image_input_supported: true,
      mesh_output_types: NORMAL_AND_GEOMETRY.to_vec(),
      pbr_supported: true,
      ..Default::default()
    },
    // Sketch-to-3D. Requires both a sketch image and a text prompt.
    MeshModelConfig {
      model: MeshModel::Hunyuan3d3Sketch,
      model_creator: ModelCreator::Tencent,
      full_name: "Hunyuan 3D 3 Sketch".to_string(),
      selector_name: "Hunyuan 3 Sketch".to_string(),
      selector_description: "3D mesh from a sketch".to_string(),
      selector_badges: strings(&["~2 min."]),
      progress_bar_ms: 120_000,
      text_prompt_supported: true,
      sketch_input_supported: true,
      face_count_supported: true,
      pbr_supported: true,
      ..Default::default()
    },
    // Image-to-3D only (exactly one input image).
    MeshModelConfig {
      model: MeshModel::Hunyuan3d2p1,
      model_creator: ModelCreator::Tencent,
      full_name: "Hunyuan 3D 2.1".to_string(),
      selector_name: "Hunyuan 2.1".to_string(),
      selector_description: "Faster, lower fidelity 3D mesh".to_string(),
      selector_badges: strings(&["~45 sec."]),
      progress_bar_ms: 60_000,
      image_input_supported: true,
      mesh_output_types: NORMAL_AND_GEOMETRY.to_vec(),
      ..Default::default()
    },
    MeshModelConfig {
      model: MeshModel::Hunyuan3d2p0,
      model_creator: ModelCreator::Tencent,
      full_name: "Hunyuan 3D 2.0".to_string(),
      selector_name: "Hunyuan 2.0".to_string(),
      selector_description: "Faster, lower fidelity 3D mesh".to_string(),
      selector_badges: strings(&["~45 sec."]),
      progress_bar_ms: 60_000,
      image_input_supported: true,
      mesh_output_types: NORMAL_AND_GEOMETRY.to_vec(),
      ..Default::default()
    },
    // Splits an existing mesh (FBX) into semantic parts.
    MeshModelConfig {
      model: MeshModel::Hunyuan3d3p1Part,
      model_creator: ModelCreator::Tencent,
      full_name: "Hunyuan 3D 3.1 Part".to_string(),
      selector_name: "Hunyuan 3.1 Part".to_string(),
      selector_description: "Mesh splitting".to_string(),
      extra_info: Some("Splits an existing 3D mesh into semantically meaningful parts".to_string()),
      selector_badges: strings(&["~2 min."]),
      progress_bar_ms: 120_000,
      mesh_input_supported: true,
      ..Default::default()
    },
    // Retopologizes an existing mesh (GLB/OBJ).
    MeshModelConfig {
      model: MeshModel::Hunyuan3d3p1SmartTopology,
      model_creator: ModelCreator::Tencent,
      full_name: "Hunyuan 3D 3.1 Smart Topology".to_string(),
      selector_name: "Hunyuan 3.1 Smart Topology".to_string(),
      selector_description: "Retopology".to_string(),
      extra_info: Some("Retopologizes an existing 3D mesh into a cleaner, more efficient topology".to_string()),
      selector_badges: strings(&["~2 min."]),
      progress_bar_ms: 120_000,
      mesh_input_supported: true,
      polygon_types: TRIANGLE_AND_QUAD.to_vec(),
      ..Default::default()
    },
    // Text, single-image, or multi-view image input with texture/geometry
    // quality tiers and quad output.
    MeshModelConfig {
      model: MeshModel::Tripo3dH3p1,
      model_creator: ModelCreator::Tripo,
      full_name: "Tripo3D H3.1".to_string(),
      selector_name: "Tripo3D H3.1".to_string(),
      selector_description: "High quality with texture controls".to_string(),
      selector_badges: strings(&["~2 min."]),
      progress_bar_ms: 120_000,
      text_prompt_supported: true,
      image_input_supported: true,
      multi_view_supported: true,
      mesh_output_types: NORMAL_AND_GEOMETRY.to_vec(),
      polygon_types: TRIANGLE_AND_QUAD.to_vec(),
      face_count_supported: true,
      pbr_supported: true,
      texture_toggle_supported: true,
      texture_quality_supported: true,
      geometry_quality_supported: true,
      ..Default::default()
    },
    // Text or single-image input with low-poly mode and quad output.
    MeshModelConfig {
      model: MeshModel::MeshyV6,
      model_creator: ModelCreator::Meshy,
      full_name: "Meshy 6".to_string(),
      selector_name: "Meshy 6".to_string(),
      selector_description: "High quality with low-poly output".to_string(),
      selector_badges: strings(&["~2 min."]),
      progress_bar_ms: 120_000,
      text_prompt_supported: true,
      image_input_supported: true,
      mesh_output_types: NORMAL_LOW_POLY_AND_GEOMETRY.to_vec(),
      polygon_types: TRIANGLE_AND_QUAD.to_vec(),
      face_count_supported: true,
      pbr_supported: true,
      texture_toggle_supported: true,
      ..Default::default()
    },
    // Text or image(s) input; fast, low-cost.
    MeshModelConfig {
      model: MeshModel::Rodin2p5Fast,
      model_creator: ModelCreator::Deemos,
      full_name: "Rodin 2.5 Fast".to_string(),
      selector_name: "Rodin 2.5 Fast".to_string(),
      selector_description: "Fast, inexpensive 3D mesh".to_string(),
      selector_badges: strings(&["~1 min."]),
      progress_bar_ms: 60_000,
      text_prompt_supported: true,
      image_input_supported: true,
      mesh_output_types: NORMAL_AND_GEOMETRY.to_vec(),
      pbr_supported: true,
      texture_toggle_supported: true,
      ..Default::default()
    },
  ]
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;
  use strum::IntoEnumIterator;

  #[test]
  fn every_model_has_exactly_one_config() {
    let listed: Vec<MeshModel> = MESH_MODELS.iter().map(|c| c.model).collect();
    let unique: HashSet<MeshModel> = listed.iter().copied().collect();
    assert_eq!(listed.len(), unique.len(), "duplicate mesh model configs");
    for model in MeshModel::iter() {
      assert!(unique.contains(&model), "no config for {model:?}");
    }
    for config in MESH_MODELS.iter() {
      assert!(!config.full_name.is_empty() && !config.selector_name.is_empty(), "{:?} needs names", config.model);
    }
  }
}
