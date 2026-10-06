use crate::configs::video_model_config::VideoModelConfig;
use crate::configs::video_models::strings;
use crate::enums::common_aspect_ratio::CommonAspectRatio;
use crate::enums::common_resolution::CommonResolution;
use crate::enums::model_creator::ModelCreator;
use crate::enums::video_model::VideoModel;

const VIDU_ASPECT_RATIOS: &[CommonAspectRatio] = &[
  CommonAspectRatio::WideSixteenByNine,
  CommonAspectRatio::TallNineBySixteen,
  CommonAspectRatio::WideFourByThree,
  CommonAspectRatio::TallThreeByFour,
  CommonAspectRatio::Square,
];

/// Vidu (Shengshu) video models.
pub fn vidu_video_models() -> Vec<VideoModelConfig> {
  vec![
    VideoModelConfig {
      model: VideoModel::ViduQ3,
      model_creator: ModelCreator::Vidu,
      full_name: "Vidu Q3".to_string(),
      selector_name: "Vidu Q3".to_string(),
      selector_description: "High-quality model".to_string(),
      selector_badges: strings(&["2 min."]),
      progress_bar_ms: 120_000,
      text_prompt_max_length: Some(5000),
      starting_keyframe_supported: true,
      ending_keyframe_supported: true,
      image_references_supported: true,
      image_references_max: Some(4),
      show_generate_with_sound_toggle: true,
      aspect_ratio_options: VIDU_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      resolution_options: vec![CommonResolution::FourEightyP, CommonResolution::SevenTwentyP, CommonResolution::TenEightyP],
      resolution_default: Some(CommonResolution::SevenTwentyP),
      duration_seconds_min: Some(1),
      duration_seconds_max: Some(16),
      duration_seconds_default: Some(5),
      ..Default::default()
    },
    VideoModelConfig {
      model: VideoModel::ViduQ3Turbo,
      model_creator: ModelCreator::Vidu,
      full_name: "Vidu Q3 Turbo".to_string(),
      selector_name: "Vidu Q3 Turbo".to_string(),
      selector_description: "High-quality model (faster)".to_string(),
      selector_badges: strings(&["90 sec."]),
      progress_bar_ms: 90_000,
      text_prompt_max_length: Some(5000),
      starting_keyframe_supported: true,
      ending_keyframe_supported: true,
      show_generate_with_sound_toggle: true,
      aspect_ratio_options: VIDU_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      resolution_options: vec![CommonResolution::FourEightyP, CommonResolution::SevenTwentyP, CommonResolution::TenEightyP],
      resolution_default: Some(CommonResolution::SevenTwentyP),
      duration_seconds_min: Some(1),
      duration_seconds_max: Some(16),
      duration_seconds_default: Some(5),
      ..Default::default()
    },
  ]
}
