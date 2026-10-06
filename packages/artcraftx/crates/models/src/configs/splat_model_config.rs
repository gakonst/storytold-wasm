use crate::enums::model_creator::ModelCreator;
use crate::enums::model_tag::ModelTag;
use crate::enums::splat_model::SplatModel;
use serde_derive::Serialize;

/// Everything ArtCraftX knows about one Gaussian splat ("world") model.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SplatModelConfig {
  // ── Identity ──
  pub model: SplatModel,
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
  pub image_references_supported: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub image_references_max: Option<u16>,
  pub video_reference_supported: bool,
  /// 360-degree panorama input.
  pub panorama_supported: bool,
  pub disable_recaption_supported: bool,
  pub is_disabled: bool,
}

impl Default for SplatModelConfig {
  fn default() -> Self {
    Self {
      model: SplatModel::Marble1p1,
      model_creator: ModelCreator::ArtCraft,
      full_name: String::new(),
      selector_name: String::new(),
      selector_description: String::new(),
      extra_info: None,
      selector_badges: Vec::new(),
      tags: Vec::new(),
      progress_bar_ms: 300_000,
      text_prompt_supported: false,
      text_prompt_max_length: Some(3000),
      image_references_supported: false,
      image_references_max: None,
      video_reference_supported: false,
      panorama_supported: false,
      disable_recaption_supported: false,
      is_disabled: false,
    }
  }
}
