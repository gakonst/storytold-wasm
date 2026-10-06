use crate::enums::common_aspect_ratio::CommonAspectRatio;
use crate::enums::common_bitrate::CommonBitrate;
use crate::enums::common_quality::CommonQuality;
use crate::enums::common_resolution::CommonResolution;
use crate::enums::legacy_video_size::LegacyVideoSize;
use crate::enums::model_creator::ModelCreator;
use crate::enums::model_tag::ModelTag;
use crate::enums::video_model::VideoModel;
use serde_derive::Serialize;

/// Everything ArtCraftX knows about one video model.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct VideoModelConfig {
  // ── Identity ──
  pub model: VideoModel,
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
  /// Whether the prompt box offers the system-prompt toggle.
  pub supports_system_prompt: bool,

  // ── Capabilities ──
  /// Can generate from a text prompt ALONE. When false, an image (starting
  /// frame / reference) is also required.
  pub text_to_video_supported: bool,
  pub text_prompt_supported: bool,
  /// `None` = no limit.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text_prompt_max_length: Option<u16>,
  pub negative_text_prompt_supported: bool,
  pub starting_keyframe_supported: bool,
  /// The model needs an image (it can't run text-only).
  pub starting_keyframe_required: bool,
  pub ending_keyframe_supported: bool,
  /// Multi-image "reference mode".
  pub image_references_supported: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub image_references_max: Option<u16>,
  pub video_references_supported: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub video_references_max: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub video_references_max_total_duration_seconds: Option<u16>,
  pub audio_references_supported: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub audio_references_max: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub audio_references_max_total_duration_seconds: Option<u16>,
  pub character_references_supported: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub character_references_max: Option<u16>,
  /// Show the "generate with sound" toggle.
  pub show_generate_with_sound_toggle: bool,
  /// Empty = no aspect-ratio control (or the model uses `legacy_size_options`).
  pub aspect_ratio_options: Vec<CommonAspectRatio>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub aspect_ratio_default: Option<CommonAspectRatio>,
  /// The coarse landscape/portrait/square picker of the first-party Grok and
  /// Sora paths (sent as `grok_aspect_ratio` / `sora_orientation`). Only set
  /// when `aspect_ratio_options` is empty.
  pub legacy_size_options: Vec<LegacyVideoSize>,
  pub resolution_options: Vec<CommonResolution>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub resolution_default: Option<CommonResolution>,
  pub bitrate_options: Vec<CommonBitrate>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bitrate_default: Option<CommonBitrate>,
  pub quality_options: Vec<CommonQuality>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quality_default: Option<CommonQuality>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub duration_seconds_min: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub duration_seconds_max: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub duration_seconds_max_with_image_references: Option<u16>,
  /// When set, the picker offers exactly these durations instead of a range.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub duration_seconds_options: Option<Vec<u16>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub duration_seconds_default: Option<u16>,
  pub batch_size_min: u16,
  pub batch_size_max: u16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub batch_size_options: Option<Vec<u16>>,
  pub batch_size_default: u16,
  pub is_disabled: bool,
}

impl Default for VideoModelConfig {
  fn default() -> Self {
    Self {
      model: VideoModel::Seedance2p0,
      model_creator: ModelCreator::ArtCraft,
      full_name: String::new(),
      selector_name: String::new(),
      selector_description: String::new(),
      extra_info: None,
      selector_badges: Vec::new(),
      tags: Vec::new(),
      progress_bar_ms: 100_000,
      supports_system_prompt: true,
      text_to_video_supported: true,
      text_prompt_supported: true,
      text_prompt_max_length: Some(3000),
      negative_text_prompt_supported: false,
      starting_keyframe_supported: false,
      starting_keyframe_required: false,
      ending_keyframe_supported: false,
      image_references_supported: false,
      image_references_max: None,
      video_references_supported: false,
      video_references_max: None,
      video_references_max_total_duration_seconds: None,
      audio_references_supported: false,
      audio_references_max: None,
      audio_references_max_total_duration_seconds: None,
      character_references_supported: false,
      character_references_max: None,
      show_generate_with_sound_toggle: false,
      aspect_ratio_options: Vec::new(),
      aspect_ratio_default: None,
      legacy_size_options: Vec::new(),
      resolution_options: Vec::new(),
      resolution_default: None,
      bitrate_options: Vec::new(),
      bitrate_default: None,
      quality_options: Vec::new(),
      quality_default: None,
      duration_seconds_min: None,
      duration_seconds_max: None,
      duration_seconds_max_with_image_references: None,
      duration_seconds_options: None,
      duration_seconds_default: None,
      batch_size_min: 1,
      batch_size_max: 1,
      batch_size_options: None,
      batch_size_default: 1,
      is_disabled: false,
    }
  }
}
