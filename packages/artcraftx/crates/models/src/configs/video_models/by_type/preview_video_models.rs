use crate::configs::video_model_config::VideoModelConfig;
use crate::configs::video_models::strings;
use crate::enums::model_creator::ModelCreator;
use crate::enums::video_model::VideoModel;

/// Unreleased preview models the router knows about. Hidden from the picker.
pub fn preview_video_models() -> Vec<VideoModelConfig> {
  let preview = |model: VideoModel, full_name: &str| VideoModelConfig {
    model,
    model_creator: ModelCreator::ArtCraft,
    full_name: full_name.to_string(),
    selector_name: full_name.to_string(),
    selector_description: "Preview".to_string(),
    selector_badges: strings(&["Preview"]),
    starting_keyframe_supported: true,
    ending_keyframe_supported: true,
    is_disabled: true,
    ..Default::default()
  };
  vec![
    preview(VideoModel::PreviewModel, "Preview Model"),
    preview(VideoModel::PreviewModelFast, "Preview Model Fast"),
  ]
}
