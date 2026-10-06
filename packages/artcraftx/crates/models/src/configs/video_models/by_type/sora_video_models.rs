use crate::configs::video_model_config::VideoModelConfig;
use crate::configs::video_models::strings;
use crate::enums::common_resolution::CommonResolution;
use crate::enums::legacy_video_size::LegacyVideoSize;
use crate::enums::model_creator::ModelCreator;
use crate::enums::video_model::VideoModel;

/// Sora (OpenAI) video models. The prompt box sends their size as
/// `sora_orientation` (landscape / portrait).
pub fn sora_video_models() -> Vec<VideoModelConfig> {
  vec![
    VideoModelConfig {
      model: VideoModel::Sora2,
      model_creator: ModelCreator::OpenAi,
      full_name: "Sora 2".to_string(),
      selector_name: "Sora 2".to_string(),
      selector_description: "Smart video model".to_string(),
      selector_badges: strings(&["2 min."]),
      progress_bar_ms: 100_000,
      text_prompt_max_length: Some(2000),
      legacy_size_options: vec![LegacyVideoSize::Landscape, LegacyVideoSize::Portrait],
      resolution_options: vec![CommonResolution::SevenTwentyP],
      resolution_default: Some(CommonResolution::SevenTwentyP),
      duration_seconds_options: Some(vec![4, 8, 12]),
      duration_seconds_default: Some(4),
      ..Default::default()
    },
    VideoModelConfig {
      model: VideoModel::Sora2Pro,
      model_creator: ModelCreator::OpenAi,
      full_name: "Sora 2 Pro".to_string(),
      selector_name: "Sora 2 Pro".to_string(),
      selector_description: "Smart video model".to_string(),
      selector_badges: strings(&["2 min."]),
      progress_bar_ms: 100_000,
      text_prompt_max_length: Some(2000),
      legacy_size_options: vec![LegacyVideoSize::Landscape, LegacyVideoSize::Portrait],
      resolution_options: vec![CommonResolution::SevenTwentyP, CommonResolution::TenEightyP],
      resolution_default: Some(CommonResolution::TenEightyP),
      duration_seconds_options: Some(vec![4, 8, 12]),
      duration_seconds_default: Some(4),
      ..Default::default()
    },
  ]
}
