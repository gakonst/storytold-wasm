use crate::enums::common_mesh_output_type::CommonMeshOutputType;
use crate::enums::common_polygon_type::CommonPolygonType;
use crate::enums::mesh_model::MeshModel;
use crate::enums::model_creator::ModelCreator;
use crate::enums::model_tag::ModelTag;
use serde_derive::Serialize;

/// Everything ArtCraftX knows about one 3D mesh model.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MeshModelConfig {
  // ── Identity ──
  pub model: MeshModel,
  pub model_creator: ModelCreator,
  pub full_name: String,

  // ── Desktop presentation ──
  pub selector_name: String,
  pub selector_description: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub extra_info: Option<String>,
  pub selector_badges: Vec<String>,
  pub tags: Vec<ModelTag>,
  pub progress_bar_ms: u32,

  // ── Capabilities ──
  pub text_prompt_supported: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text_prompt_max_length: Option<u16>,
  pub image_input_supported: bool,
  pub sketch_input_supported: bool,
  pub multi_view_supported: bool,
  /// Takes an existing mesh as input (part splitting, retopology).
  pub mesh_input_supported: bool,
  pub mesh_output_types: Vec<CommonMeshOutputType>,
  pub polygon_types: Vec<CommonPolygonType>,
  pub face_count_supported: bool,
  pub pbr_supported: bool,
  pub texture_toggle_supported: bool,
  pub texture_quality_supported: bool,
  pub geometry_quality_supported: bool,
  pub is_disabled: bool,
}

impl Default for MeshModelConfig {
  fn default() -> Self {
    Self {
      model: MeshModel::Hunyuan3d3,
      model_creator: ModelCreator::ArtCraft,
      full_name: String::new(),
      selector_name: String::new(),
      selector_description: String::new(),
      extra_info: None,
      selector_badges: Vec::new(),
      tags: Vec::new(),
      progress_bar_ms: 120_000,
      text_prompt_supported: false,
      text_prompt_max_length: Some(3000),
      image_input_supported: false,
      sketch_input_supported: false,
      multi_view_supported: false,
      mesh_input_supported: false,
      mesh_output_types: Vec::new(),
      polygon_types: Vec::new(),
      face_count_supported: false,
      pbr_supported: false,
      texture_toggle_supported: false,
      texture_quality_supported: false,
      geometry_quality_supported: false,
      is_disabled: false,
    }
  }
}
