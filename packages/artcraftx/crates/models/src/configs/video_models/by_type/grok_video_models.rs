use crate::configs::video_model_config::VideoModelConfig;
use crate::configs::video_models::strings;
use crate::enums::common_resolution::CommonResolution;
use crate::enums::legacy_video_size::LegacyVideoSize;
use crate::enums::model_creator::ModelCreator;
use crate::enums::video_model::VideoModel;

/// Grok Imagine video models.
pub fn grok_video_models() -> Vec<VideoModelConfig> {
  vec![
    // First-party (cookie session). The prompt box sends its size as
    // `grok_aspect_ratio` (landscape / portrait / square).
    VideoModelConfig {
      model: VideoModel::GrokImagineVideo,
      model_creator: ModelCreator::Grok,
      full_name: "Grok Video".to_string(),
      selector_name: "Grok Video".to_string(),
      selector_description: "Fastest video model".to_string(),
      selector_badges: strings(&["20 sec."]),
      progress_bar_ms: 50_000,
      text_prompt_max_length: Some(4096),
      starting_keyframe_supported: true,
      starting_keyframe_required: true,
      text_to_video_supported: false,
      legacy_size_options: vec![LegacyVideoSize::Landscape, LegacyVideoSize::Portrait, LegacyVideoSize::Square],
      ..Default::default()
    },
    // Served via Artcraft (storyteller-web / router) for now, even though the
    // model is made by xAI. See `generate_video_command.rs`.
    VideoModelConfig {
      model: VideoModel::GrokImagineVideo1p5,
      model_creator: ModelCreator::Grok,
      full_name: "Grok Imagine 1.5".to_string(),
      selector_name: "Grok Imagine 1.5".to_string(),
      selector_description: "Image-to-video by xAI".to_string(),
      extra_info: Some("Fast and high quality. Requires a starting frame.".to_string()),
      selector_badges: strings(&["Preview"]),
      progress_bar_ms: 50_000,
      text_prompt_max_length: Some(4096),
      starting_keyframe_supported: true,
      starting_keyframe_required: true,
      text_to_video_supported: false,
      legacy_size_options: vec![LegacyVideoSize::Landscape, LegacyVideoSize::Portrait, LegacyVideoSize::Square],
      resolution_options: vec![CommonResolution::FourEightyP, CommonResolution::SevenTwentyP],
      resolution_default: Some(CommonResolution::SevenTwentyP),
      duration_seconds_min: Some(1),
      duration_seconds_max: Some(15),
      duration_seconds_max_with_image_references: Some(10),
      duration_seconds_default: Some(8),
      ..Default::default()
    },
  ]
}
