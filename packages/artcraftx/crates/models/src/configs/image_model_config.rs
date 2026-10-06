use crate::enums::common_aspect_ratio::CommonAspectRatio;
use crate::enums::common_quality::CommonQuality;
use crate::enums::common_resolution::CommonResolution;
use crate::enums::image_model::ImageModel;
use crate::enums::model_creator::ModelCreator;
use crate::enums::model_tag::ModelTag;
use serde_derive::Serialize;

/// Everything ArtCraftX knows about one image model.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ImageModelConfig {
  // ── Identity ──
  pub model: ImageModel,
  pub model_creator: ModelCreator,
  /// Long name (may need abbreviating in narrow UI).
  pub full_name: String,

  // ── Desktop presentation ──
  /// Name shown in the picker.
  pub selector_name: String,
  /// One-line tagline shown under the name.
  pub selector_description: String,
  /// Longer blurb for the (i) info icon, if any.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub extra_info: Option<String>,
  /// Small labels next to the name (e.g. "10 sec.").
  pub selector_badges: Vec<String>,
  pub tags: Vec<ModelTag>,
  /// How long the fake progress bar takes to reach 100% (UI-only).
  pub progress_bar_ms: u32,

  // ── Page placement (which editors show the model) ──
  /// The text-to-image page.
  pub can_text_to_image: bool,
  /// Image editing models that focus on editing a single image.
  pub can_edit_images: bool,
  /// For inpainting models: does it take a mask?
  pub uses_inpainting_mask: bool,
  /// For editing models: is "editing" == "inpainting"?
  pub editing_is_inpainting: bool,
  /// The camera-angle editing page.
  pub can_edit_angles: bool,

  // ── Capabilities ──
  pub text_prompt_supported: bool,
  /// `None` = no limit.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text_prompt_max_length: Option<u16>,
  pub negative_text_prompt_supported: bool,
  /// Reference images (image prompts).
  pub image_refs_supported: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub image_refs_max: Option<u16>,
  pub has_fixed_editing_aspect_ratio: bool,
  /// Empty = the model has no aspect-ratio control.
  pub aspect_ratio_options: Vec<CommonAspectRatio>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub aspect_ratio_default: Option<CommonAspectRatio>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub aspect_ratio_default_when_editing: Option<CommonAspectRatio>,
  /// Empty = the model has no resolution control.
  pub resolution_options: Vec<CommonResolution>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub resolution_default: Option<CommonResolution>,
  pub quality_options: Vec<CommonQuality>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quality_default: Option<CommonQuality>,
  pub batch_size_min: u16,
  pub batch_size_max: u16,
  /// When set, the picker offers exactly these counts instead of a range.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub batch_size_options: Option<Vec<u16>>,
  pub batch_size_default: u16,
  /// Hidden from the picker (kept in the table so ids stay known).
  pub is_disabled: bool,
}

impl Default for ImageModelConfig {
  fn default() -> Self {
    Self {
      model: ImageModel::NanoBananaPro,
      model_creator: ModelCreator::ArtCraft,
      full_name: String::new(),
      selector_name: String::new(),
      selector_description: String::new(),
      extra_info: None,
      selector_badges: Vec::new(),
      tags: Vec::new(),
      progress_bar_ms: 20_000,
      can_text_to_image: true,
      can_edit_images: false,
      uses_inpainting_mask: false,
      editing_is_inpainting: false,
      can_edit_angles: false,
      text_prompt_supported: true,
      text_prompt_max_length: Some(3000),
      negative_text_prompt_supported: false,
      image_refs_supported: false,
      image_refs_max: None,
      has_fixed_editing_aspect_ratio: false,
      aspect_ratio_options: Vec::new(),
      aspect_ratio_default: None,
      aspect_ratio_default_when_editing: None,
      resolution_options: Vec::new(),
      resolution_default: None,
      quality_options: Vec::new(),
      quality_default: None,
      batch_size_min: 1,
      batch_size_max: 4,
      batch_size_options: None,
      batch_size_default: 1,
      is_disabled: false,
    }
  }
}
