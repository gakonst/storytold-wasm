use crate::configs::video_model_config::VideoModelConfig;
use crate::configs::video_models::strings;
use crate::enums::common_aspect_ratio::CommonAspectRatio;
use crate::enums::common_bitrate::CommonBitrate;
use crate::enums::common_resolution::CommonResolution;
use crate::enums::model_creator::ModelCreator;
use crate::enums::video_model::VideoModel;

const SEEDANCE_ASPECT_RATIOS: &[CommonAspectRatio] = &[
  CommonAspectRatio::WideTwentyOneByNine,
  CommonAspectRatio::WideSixteenByNine,
  CommonAspectRatio::WideFourByThree,
  CommonAspectRatio::Square,
  CommonAspectRatio::TallThreeByFour,
  CommonAspectRatio::TallNineBySixteen,
];

/// Bytedance Seedance video models: 2.5 (Higgsfield), the 2.0 family (with
/// references), 1.5 Pro, and 1.0 Lite.
pub fn seedance_video_models() -> Vec<VideoModelConfig> {
  let mut models = Vec::new();

  // Higgsfield-only. Same reference surface as 2.0, longer clips (up to 30s),
  // and no 4K tier.
  models.push(VideoModelConfig {
    model: VideoModel::Seedance2p5,
    model_creator: ModelCreator::Bytedance,
    full_name: "Seedance 2.5".to_string(),
    selector_name: "Seedance 2.5".to_string(),
    selector_description: "Newest Seedance; up to 30 seconds".to_string(),
    selector_badges: strings(&["~10 min."]),
    progress_bar_ms: 600_000,
    supports_system_prompt: false,
    text_prompt_max_length: Some(10_000),
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
    show_generate_with_sound_toggle: true,
    aspect_ratio_options: SEEDANCE_ASPECT_RATIOS.to_vec(),
    aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
    resolution_options: vec![CommonResolution::FourEightyP, CommonResolution::SevenTwentyP, CommonResolution::TenEightyP],
    resolution_default: Some(CommonResolution::SevenTwentyP),
    bitrate_options: vec![CommonBitrate::Normal, CommonBitrate::High],
    bitrate_default: Some(CommonBitrate::High),
    duration_seconds_min: Some(4),
    duration_seconds_max: Some(30),
    duration_seconds_options: Some((4..=30).collect()),
    duration_seconds_default: Some(5),
    batch_size_max: 4,
    batch_size_options: Some(vec![1, 2, 3, 4]),
    batch_size_default: 1,
    ..Default::default()
  });

  // Higgsfield-only. Video-to-video: the first video reference is the clip to
  // edit; images and audio are extra references. Output keeps the source's
  // length and framing, so there is no duration or aspect control.
  models.push(VideoModelConfig {
    model: VideoModel::Seedance2p5Edit,
    model_creator: ModelCreator::Bytedance,
    full_name: "Seedance 2.5 Edit".to_string(),
    selector_name: "Seedance 2.5 Edit".to_string(),
    selector_description: "Edit an existing video with a prompt".to_string(),
    extra_info: Some("Video-to-video. Attach the clip to edit as a video reference; add image and audio references to guide the edit.".to_string()),
    selector_badges: strings(&["~10 min."]),
    progress_bar_ms: 600_000,
    supports_system_prompt: false,
    text_to_video_supported: false,
    text_prompt_max_length: Some(10_000),
    image_references_supported: true,
    image_references_max: Some(9),
    video_references_supported: true,
    video_references_max: Some(1),
    video_references_max_total_duration_seconds: Some(15),
    audio_references_supported: true,
    audio_references_max: Some(3),
    audio_references_max_total_duration_seconds: Some(15),
    show_generate_with_sound_toggle: true,
    resolution_options: vec![CommonResolution::FourEightyP, CommonResolution::SevenTwentyP, CommonResolution::TenEightyP],
    resolution_default: Some(CommonResolution::SevenTwentyP),
    bitrate_options: vec![CommonBitrate::Normal, CommonBitrate::High],
    bitrate_default: Some(CommonBitrate::High),
    batch_size_max: 4,
    batch_size_options: Some(vec![1, 2, 3, 4]),
    batch_size_default: 1,
    ..Default::default()
  });

  models.push(seedance_2p0(
    VideoModel::Seedance2p0,
    "Seedance 2.0",
    "High quality model",
    "~15 min.",
    900_000,
    Some("The Chinese Volcengine (ByteDance China API platform) version of Seedance 2.0. Checkpoint is from January 2026. This may be better at some characters than the other Seedance models."),
    &[CommonResolution::FourEightyP, CommonResolution::SevenTwentyP, CommonResolution::TenEightyP, CommonResolution::FourK],
    true,
  ));
  models.push(seedance_2p0(
    VideoModel::Seedance2p0Fast,
    "Seedance 2.0 Fast",
    "Fast and high quality",
    "~5 min.",
    300_000,
    Some("The Chinese Volcengine (ByteDance China API platform) version of Seedance 2.0 Fast. Checkpoint is from January 2026. This may be better at some characters than the other Seedance models."),
    &[CommonResolution::FourEightyP, CommonResolution::SevenTwentyP],
    true,
  ));
  models.push(seedance_2p0(
    VideoModel::Seedance2p0BytePlus,
    "Seedance 2.0 Plus",
    "Seedance 2.0 (BytePlus version)",
    "~15 min.",
    900_000,
    Some("The Chinese BytePlus (ByteDance's Western API platform) version of Seedance 2.0. This has fewer restrictions on faces and IP."),
    &[CommonResolution::FourEightyP, CommonResolution::SevenTwentyP, CommonResolution::TenEightyP, CommonResolution::FourK],
    false,
  ));
  models.push(seedance_2p0(
    VideoModel::Seedance2p0BytePlusFast,
    "Seedance 2.0 Plus Fast",
    "Seedance 2.0 Fast (BytePlus version)",
    "~5 min.",
    300_000,
    Some("The Chinese BytePlus (ByteDance's Western API platform) version of Seedance 2.0 Fast. This has fewer restrictions on faces and IP."),
    &[CommonResolution::FourEightyP, CommonResolution::SevenTwentyP],
    false,
  ));
  models.push(seedance_2p0(
    VideoModel::Seedance2p0Ultra,
    "Seedance 2.0 Ultra",
    "Seedance 2.0; less filtered",
    "~15 min.",
    900_000,
    None,
    &[CommonResolution::FourEightyP, CommonResolution::SevenTwentyP, CommonResolution::TenEightyP, CommonResolution::FourK],
    true,
  ));
  models.push(seedance_2p0(
    VideoModel::Seedance2p0UltraFast,
    "Seedance 2.0 Ultra Fast",
    "Seedance 2.0 Fast; less filtered",
    "~5 min.",
    300_000,
    None,
    &[CommonResolution::FourEightyP, CommonResolution::SevenTwentyP],
    true,
  ));
  models.push(seedance_2p0(
    VideoModel::Seedance2p0BytePlusUltra,
    "Seedance 2.0 Plus Ultra",
    "Seedance 2.0 (BytePlus version); less filtered",
    "~15 min.",
    900_000,
    Some("This is the same BytePlus version of Seedance 2.0, but with even fewer restrictions around content. Horror movies, action movie violence, and more is possible."),
    &[CommonResolution::FourEightyP, CommonResolution::SevenTwentyP, CommonResolution::TenEightyP, CommonResolution::FourK],
    false,
  ));
  models.push(seedance_2p0(
    VideoModel::Seedance2p0BytePlusUltraFast,
    "Seedance 2.0 Plus Ultra Fast",
    "Seedance 2.0 Fast (BytePlus version); less filtered",
    "~5 min.",
    300_000,
    Some("This is the same BytePlus version of Seedance 2.0 Fast, but with even fewer restrictions around content. Horror movies, action movie violence, and more is possible."),
    &[CommonResolution::FourEightyP, CommonResolution::SevenTwentyP],
    false,
  ));
  models.push(seedance_2p0(
    VideoModel::Seedance2p0Mini,
    "Seedance 2.0 Mini",
    "Chinese Volcengine version",
    "~5 min.",
    300_000,
    Some("The Chinese Volcengine (ByteDance China API platform) version of Seedance 2.0 Mini. This may be better at some characters than the other Seedance models."),
    &[CommonResolution::FourEightyP, CommonResolution::SevenTwentyP],
    true,
  ));
  models.push(seedance_2p0(
    VideoModel::Seedance2p0BytePlusMini,
    "Seedance 2.0 Plus Mini",
    "BytePlus version",
    "~5 min.",
    300_000,
    Some("The Chinese BytePlus (ByteDance's Western API platform) version of Seedance 2.0 Mini. This has fewer restrictions on faces and IP."),
    &[CommonResolution::FourEightyP, CommonResolution::SevenTwentyP],
    false,
  ));
  models.push(seedance_2p0(
    VideoModel::Seedance2p0BytePlusUltraMini,
    "Seedance 2.0 Plus Ultra Mini",
    "BytePlus version; less filtered",
    "~5 min.",
    300_000,
    Some("This is the same BytePlus version of Seedance 2.0 Mini, but with even fewer restrictions around content. Horror movies, action movie violence, and more is possible."),
    &[CommonResolution::FourEightyP, CommonResolution::SevenTwentyP],
    false,
  ));

  models.push(VideoModelConfig {
    model: VideoModel::Seedance1p5Pro,
    model_creator: ModelCreator::Bytedance,
    full_name: "Seedance 1.5 Pro".to_string(),
    selector_name: "Seedance 1.5 Pro".to_string(),
    selector_description: "High quality video model".to_string(),
    selector_badges: strings(&["2 min."]),
    progress_bar_ms: 180_000,
    supports_system_prompt: false,
    starting_keyframe_supported: true,
    ending_keyframe_supported: true,
    show_generate_with_sound_toggle: true,
    aspect_ratio_options: vec![
      CommonAspectRatio::WideSixteenByNine,
      CommonAspectRatio::Square,
      CommonAspectRatio::TallThreeByFour,
      CommonAspectRatio::WideFourByThree,
      CommonAspectRatio::TallNineBySixteen,
      CommonAspectRatio::WideTwentyOneByNine,
      CommonAspectRatio::Auto,
    ],
    aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
    resolution_options: vec![CommonResolution::FourEightyP, CommonResolution::SevenTwentyP, CommonResolution::TenEightyP],
    resolution_default: Some(CommonResolution::SevenTwentyP),
    duration_seconds_min: Some(4),
    duration_seconds_max: Some(12),
    duration_seconds_options: Some((4..=12).collect()),
    duration_seconds_default: Some(5),
    ..Default::default()
  });

  models.push(VideoModelConfig {
    model: VideoModel::Seedance10Lite,
    model_creator: ModelCreator::Bytedance,
    full_name: "Seedance 1.0 Lite".to_string(),
    selector_name: "Seedance 1.0 Lite".to_string(),
    selector_description: "Fast video model".to_string(),
    selector_badges: strings(&["2 min."]),
    progress_bar_ms: 100_000,
    starting_keyframe_supported: true,
    starting_keyframe_required: true,
    text_to_video_supported: false,
    aspect_ratio_options: vec![
      CommonAspectRatio::Auto,
      CommonAspectRatio::WideTwentyOneByNine,
      CommonAspectRatio::WideSixteenByNine,
      CommonAspectRatio::WideFourByThree,
      CommonAspectRatio::Square,
      CommonAspectRatio::TallThreeByFour,
      CommonAspectRatio::TallNineBySixteen,
    ],
    aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
    resolution_options: vec![CommonResolution::FourEightyP, CommonResolution::SevenTwentyP, CommonResolution::TenEightyP],
    resolution_default: Some(CommonResolution::SevenTwentyP),
    duration_seconds_options: Some(vec![5, 10]),
    duration_seconds_default: Some(5),
    ..Default::default()
  });

  models
}

/// The Seedance 2.0 family shares one capability surface: keyframes, image /
/// video / audio (and sometimes character) references, bitrate, and batches.
fn seedance_2p0(
  model: VideoModel,
  full_name: &str,
  description: &str,
  badge: &str,
  progress_bar_ms: u32,
  extra_info: Option<&str>,
  resolutions: &[CommonResolution],
  character_references_supported: bool,
) -> VideoModelConfig {
  VideoModelConfig {
    model,
    model_creator: ModelCreator::Bytedance,
    full_name: full_name.to_string(),
    selector_name: full_name.to_string(),
    selector_description: description.to_string(),
    extra_info: extra_info.map(|s| s.to_string()),
    selector_badges: strings(&[badge]),
    progress_bar_ms,
    supports_system_prompt: false,
    text_prompt_max_length: Some(10_000),
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
    character_references_supported,
    character_references_max: Some(9),
    aspect_ratio_options: SEEDANCE_ASPECT_RATIOS.to_vec(),
    aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
    resolution_options: resolutions.to_vec(),
    resolution_default: Some(CommonResolution::SevenTwentyP),
    bitrate_options: vec![CommonBitrate::Normal, CommonBitrate::High],
    bitrate_default: Some(CommonBitrate::Normal),
    duration_seconds_min: Some(4),
    duration_seconds_max: Some(15),
    duration_seconds_options: Some((4..=15).collect()),
    duration_seconds_default: Some(5),
    batch_size_max: 4,
    batch_size_options: Some(vec![1, 2, 3, 4]),
    batch_size_default: 1,
    ..Default::default()
  }
}
