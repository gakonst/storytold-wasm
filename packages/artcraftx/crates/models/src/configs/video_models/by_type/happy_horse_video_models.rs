use crate::configs::video_model_config::VideoModelConfig;
use crate::configs::video_models::strings;
use crate::enums::common_aspect_ratio::CommonAspectRatio;
use crate::enums::common_resolution::CommonResolution;
use crate::enums::model_creator::ModelCreator;
use crate::enums::video_model::VideoModel;

/// Happy Horse (Alibaba) video models.
pub fn happy_horse_video_models() -> Vec<VideoModelConfig> {
  vec![
    VideoModelConfig {
      model: VideoModel::HappyHorse1p0,
      model_creator: ModelCreator::Alibaba,
      full_name: "Happy Horse 1.0".to_string(),
      selector_name: "Happy Horse 1.0".to_string(),
      selector_description: "High quality video model".to_string(),
      selector_badges: strings(&["2 min."]),
      progress_bar_ms: 300_000,
      text_prompt_max_length: Some(2500),
      starting_keyframe_supported: true,
      show_generate_with_sound_toggle: true,
      aspect_ratio_options: vec![
        CommonAspectRatio::WideSixteenByNine,
        CommonAspectRatio::WideFourByThree,
        CommonAspectRatio::Square,
        CommonAspectRatio::TallThreeByFour,
        CommonAspectRatio::TallNineBySixteen,
      ],
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      resolution_options: vec![CommonResolution::SevenTwentyP, CommonResolution::TenEightyP],
      resolution_default: Some(CommonResolution::SevenTwentyP),
      duration_seconds_min: Some(3),
      duration_seconds_max: Some(15),
      duration_seconds_options: Some((3..=15).collect()),
      duration_seconds_default: Some(5),
      batch_size_max: 4,
      batch_size_options: Some(vec![1, 2, 4]),
      batch_size_default: 1,
      ..Default::default()
    },
  ]
}
