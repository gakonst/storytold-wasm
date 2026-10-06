use crate::enums::audio_model::AudioModel;
use crate::enums::model_creator::ModelCreator;
use crate::enums::model_tag::ModelTag;
use serde_derive::Serialize;

/// Everything ArtCraftX knows about one audio model.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AudioModelConfig {
  // ── Identity ──
  pub model: AudioModel,
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
  /// A separate "style" prompt (genre / mood direction).
  pub style_prompt_supported: bool,
  pub audio_references_supported: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub audio_references_max: Option<u16>,
  pub image_references_supported: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub image_references_max: Option<u16>,
  pub keep_lyrics_supported: bool,
  pub instrumental_toggle_supported: bool,
  pub loopable_toggle_supported: bool,
  pub bpm_supported: bool,
  pub musical_key_supported: bool,
  pub sample_rate_hz_options: Vec<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sample_rate_hz_default: Option<u32>,
  pub speed_supported: bool,
  pub volume_supported: bool,
  pub pitch_supported: bool,
  pub is_disabled: bool,
}

impl Default for AudioModelConfig {
  fn default() -> Self {
    Self {
      model: AudioModel::SunoMusic,
      model_creator: ModelCreator::ArtCraft,
      full_name: String::new(),
      selector_name: String::new(),
      selector_description: String::new(),
      extra_info: None,
      selector_badges: Vec::new(),
      tags: Vec::new(),
      progress_bar_ms: 120_000,
      text_prompt_supported: true,
      text_prompt_max_length: Some(3000),
      style_prompt_supported: false,
      audio_references_supported: false,
      audio_references_max: None,
      image_references_supported: false,
      image_references_max: None,
      keep_lyrics_supported: false,
      instrumental_toggle_supported: false,
      loopable_toggle_supported: false,
      bpm_supported: false,
      musical_key_supported: false,
      sample_rate_hz_options: Vec::new(),
      sample_rate_hz_default: None,
      speed_supported: false,
      volume_supported: false,
      pitch_supported: false,
      is_disabled: false,
    }
  }
}
