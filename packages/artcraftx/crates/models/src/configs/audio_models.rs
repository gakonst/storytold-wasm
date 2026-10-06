//! The built-in audio model table. Picker order = table order.

use crate::configs::audio_model_config::AudioModelConfig;
use crate::enums::audio_model::AudioModel;
use crate::enums::model_creator::ModelCreator;
use once_cell::sync::Lazy;

pub static AUDIO_MODELS: Lazy<Vec<AudioModelConfig>> = Lazy::new(audio_models);

/// Look up one model's config.
pub fn audio_model_config(model: AudioModel) -> &'static AudioModelConfig {
  AUDIO_MODELS.iter()
      .find(|config| config.model == model)
      .expect("every AudioModel variant has a config (see tests)")
}

fn strings(items: &[&str]) -> Vec<String> {
  items.iter().map(|s| s.to_string()).collect()
}

fn audio_models() -> Vec<AudioModelConfig> {
  vec![
    // Full songs from a text prompt with optional style direction.
    AudioModelConfig {
      model: AudioModel::SunoMusic,
      model_creator: ModelCreator::Suno,
      full_name: "Suno Music".to_string(),
      selector_name: "Suno Music".to_string(),
      selector_description: "Full songs from a text prompt".to_string(),
      selector_badges: strings(&["~2 min."]),
      style_prompt_supported: true,
      instrumental_toggle_supported: true,
      ..Default::default()
    },
    // Remix an existing track (exactly one audio reference).
    AudioModelConfig {
      model: AudioModel::SunoRemix,
      model_creator: ModelCreator::Suno,
      full_name: "Suno Remix".to_string(),
      selector_name: "Suno Remix".to_string(),
      selector_description: "Remix an existing track".to_string(),
      selector_badges: strings(&["~2 min."]),
      style_prompt_supported: true,
      keep_lyrics_supported: true,
      audio_references_supported: true,
      audio_references_max: Some(1),
      ..Default::default()
    },
    // Sound effects with loop, BPM, and musical key controls.
    AudioModelConfig {
      model: AudioModel::SunoSounds,
      model_creator: ModelCreator::Suno,
      full_name: "Suno Sounds".to_string(),
      selector_name: "Suno Sounds".to_string(),
      selector_description: "Sound effects with beat control".to_string(),
      selector_badges: strings(&["~1 min."]),
      progress_bar_ms: 60_000,
      loopable_toggle_supported: true,
      bpm_supported: true,
      musical_key_supported: true,
      ..Default::default()
    },
    // Build a song from a sample (exactly one audio reference).
    AudioModelConfig {
      model: AudioModel::SunoSample,
      model_creator: ModelCreator::Suno,
      full_name: "Suno Sample".to_string(),
      selector_name: "Suno Sample".to_string(),
      selector_description: "Build a song from a sample".to_string(),
      selector_badges: strings(&["~2 min."]),
      style_prompt_supported: true,
      instrumental_toggle_supported: true,
      audio_references_supported: true,
      audio_references_max: Some(1),
      ..Default::default()
    },
    // Sound generation with audio/image references and output shaping controls.
    AudioModelConfig {
      model: AudioModel::SeedAudio1p0,
      model_creator: ModelCreator::Bytedance,
      full_name: "Seed Audio 1.0".to_string(),
      selector_name: "Seed Audio 1.0".to_string(),
      selector_description: "Sound generation with fine tuning".to_string(),
      selector_badges: strings(&["~1 min."]),
      progress_bar_ms: 60_000,
      audio_references_supported: true,
      audio_references_max: Some(3),
      image_references_supported: true,
      image_references_max: Some(1),
      sample_rate_hz_options: vec![8000, 16000, 24000, 32000, 44100, 48000],
      sample_rate_hz_default: Some(24000),
      speed_supported: true,
      volume_supported: true,
      pitch_supported: true,
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
    let listed: Vec<AudioModel> = AUDIO_MODELS.iter().map(|c| c.model).collect();
    let unique: HashSet<AudioModel> = listed.iter().copied().collect();
    assert_eq!(listed.len(), unique.len(), "duplicate audio model configs");
    for model in AudioModel::iter() {
      assert!(unique.contains(&model), "no config for {model:?}");
    }
    for config in AUDIO_MODELS.iter() {
      if let Some(default) = config.sample_rate_hz_default {
        assert!(config.sample_rate_hz_options.contains(&default), "{:?} sample rate default not offered", config.model);
      }
    }
  }
}
