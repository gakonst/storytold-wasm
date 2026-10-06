use crate::configs::video_model_config::VideoModelConfig;
use crate::configs::video_models::strings;
use crate::enums::model_creator::ModelCreator;
use crate::enums::video_model::VideoModel;

/// Beeble VFX models. SwitchX isn't a generator: it relights / swaps the
/// background of an existing video, so it's excluded from the video picker
/// by id on the frontend.
pub fn beeble_video_models() -> Vec<VideoModelConfig> {
  vec![
    VideoModelConfig {
      model: VideoModel::SwitchX,
      model_creator: ModelCreator::Beeble,
      full_name: "Beeble SwitchX".to_string(),
      selector_name: "Beeble SwitchX".to_string(),
      selector_description: "Relight, change location, swap objects.".to_string(),
      selector_badges: strings(&["5 min."]),
      progress_bar_ms: 5 * 60 * 1000,
      text_prompt_max_length: Some(2000),
      starting_keyframe_required: true,
      starting_keyframe_supported: true,
      text_to_video_supported: false,
      ..Default::default()
    },
  ]
}
