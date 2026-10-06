use crate::configs::video_model_config::VideoModelConfig;
use crate::configs::video_models::strings;
use crate::enums::model_creator::ModelCreator;
use crate::enums::video_model::VideoModel;

/// MiniMax (Hailuo) video models.
pub fn minimax_video_models() -> Vec<VideoModelConfig> {
  vec![
    // Higgsfield-only. Fixed 2K output that follows the references' framing:
    // no aspect-ratio or resolution controls.
    VideoModelConfig {
      model: VideoModel::MinimaxH3,
      model_creator: ModelCreator::Hailuo,
      full_name: "MiniMax Hailuo 3".to_string(),
      selector_name: "MiniMax H3".to_string(),
      selector_description: "2K video with references".to_string(),
      extra_info: Some("Always renders at 2K and follows the aspect of the start frame or references.".to_string()),
      selector_badges: strings(&["~5 min."]),
      progress_bar_ms: 300_000,
      supports_system_prompt: false,
      text_prompt_max_length: Some(5_000),
      starting_keyframe_supported: true,
      ending_keyframe_supported: true,
      image_references_supported: true,
      image_references_max: Some(9),
      video_references_supported: true,
      video_references_max: Some(3),
      video_references_max_total_duration_seconds: Some(15),
      audio_references_supported: true,
      audio_references_max: Some(3),
      audio_references_max_total_duration_seconds: Some(15),
      duration_seconds_min: Some(5),
      duration_seconds_max: Some(15),
      duration_seconds_options: Some((5..=15).collect()),
      duration_seconds_default: Some(5),
      ..Default::default()
    },
  ]
}
