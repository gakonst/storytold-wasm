//! The built-in image model table. Picker order = table order.

use crate::configs::image_model_config::ImageModelConfig;
use crate::enums::common_aspect_ratio::CommonAspectRatio;
use crate::enums::common_quality::CommonQuality;
use crate::enums::common_resolution::CommonResolution;
use crate::enums::image_model::ImageModel;
use crate::enums::model_creator::ModelCreator;
use crate::enums::model_tag::ModelTag;
use once_cell::sync::Lazy;

pub static IMAGE_MODELS: Lazy<Vec<ImageModelConfig>> = Lazy::new(image_models);

/// Look up one model's config.
pub fn image_model_config(model: ImageModel) -> &'static ImageModelConfig {
  IMAGE_MODELS.iter()
      .find(|config| config.model == model)
      .expect("every ImageModel variant has a config (see tests)")
}

fn image_models() -> Vec<ImageModelConfig> {
  let mut models = Vec::new();

  models.extend(google_models());
  models.extend(openai_models());
  models.extend(bytedance_models());
  models.extend(black_forest_labs_models());
  models.extend(midjourney_models());
  models.extend(grok_models());
  models.extend(editing_only_models());

  models
}

// ── Aspect ratio sets shared by several models ──

const NANO_BANANA_ASPECT_RATIOS: &[CommonAspectRatio] = &[
  CommonAspectRatio::Auto,
  CommonAspectRatio::Square,
  CommonAspectRatio::WideFiveByFour,
  CommonAspectRatio::WideFourByThree,
  CommonAspectRatio::WideThreeByTwo,
  CommonAspectRatio::WideSixteenByNine,
  CommonAspectRatio::WideTwentyOneByNine,
  CommonAspectRatio::TallFourByFive,
  CommonAspectRatio::TallThreeByFour,
  CommonAspectRatio::TallTwoByThree,
  CommonAspectRatio::TallNineBySixteen,
];

const FLUX_1_ASPECT_RATIOS: &[CommonAspectRatio] = &[
  CommonAspectRatio::Square,
  CommonAspectRatio::SquareHd,
  CommonAspectRatio::WideFourByThree,
  CommonAspectRatio::WideSixteenByNine,
  CommonAspectRatio::TallThreeByFour,
  CommonAspectRatio::TallNineBySixteen,
];

const MIDJOURNEY_ASPECT_RATIOS: &[CommonAspectRatio] = &[
  CommonAspectRatio::Auto,
  CommonAspectRatio::Square,
  CommonAspectRatio::TallTwoByThree,
  CommonAspectRatio::TallThreeByFour,
  CommonAspectRatio::TallFourByFive,
  CommonAspectRatio::TallNineBySixteen,
  CommonAspectRatio::WideThreeByTwo,
  CommonAspectRatio::WideFourByThree,
  CommonAspectRatio::WideFiveByFour,
  CommonAspectRatio::WideSixteenByNine,
  CommonAspectRatio::WideTwentyOneByNine,
];

const SEEDREAM_5_PRO_ASPECT_RATIOS: &[CommonAspectRatio] = &[
  CommonAspectRatio::Auto,
  CommonAspectRatio::WideTwentyOneByNine,
  CommonAspectRatio::WideSixteenByNine,
  CommonAspectRatio::WideThreeByTwo,
  CommonAspectRatio::WideFourByThree,
  CommonAspectRatio::Square,
  CommonAspectRatio::TallThreeByFour,
  CommonAspectRatio::TallTwoByThree,
  CommonAspectRatio::TallNineBySixteen,
];

const ANGLES_ASPECT_RATIOS: &[CommonAspectRatio] = &[
  CommonAspectRatio::Square,
  CommonAspectRatio::SquareHd,
  CommonAspectRatio::WideFourByThree,
  CommonAspectRatio::WideSixteenByNine,
  CommonAspectRatio::TallThreeByFour,
  CommonAspectRatio::TallNineBySixteen,
];

const OPENAI_QUALITIES: &[CommonQuality] = &[CommonQuality::High, CommonQuality::Medium, CommonQuality::Low];

fn strings(items: &[&str]) -> Vec<String> {
  items.iter().map(|s| s.to_string()).collect()
}

// ── Google ──

fn google_models() -> Vec<ImageModelConfig> {
  vec![
    ImageModelConfig {
      model: ImageModel::NanoBananaPro,
      model_creator: ModelCreator::Google,
      full_name: "Nano Banana Pro".to_string(),
      selector_name: "Nano Banana Pro".to_string(),
      selector_description: "Powerful instructive editing".to_string(),
      selector_badges: strings(&["30 sec."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 25_000,
      can_edit_images: true,
      text_prompt_max_length: None,
      image_refs_supported: true,
      image_refs_max: Some(4),
      aspect_ratio_options: NANO_BANANA_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Auto),
      resolution_options: vec![CommonResolution::OneK, CommonResolution::TwoK, CommonResolution::FourK],
      resolution_default: Some(CommonResolution::OneK),
      batch_size_max: 4,
      batch_size_default: 1,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::NanoBanana2,
      model_creator: ModelCreator::Google,
      full_name: "Nano Banana 2".to_string(),
      selector_name: "Nano Banana 2".to_string(),
      selector_description: "Fast instructive editing".to_string(),
      selector_badges: strings(&["25 sec."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 25_000,
      can_edit_images: true,
      text_prompt_max_length: None,
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: NANO_BANANA_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Auto),
      resolution_options: vec![CommonResolution::HalfK, CommonResolution::OneK, CommonResolution::TwoK, CommonResolution::FourK],
      resolution_default: Some(CommonResolution::OneK),
      batch_size_max: 4,
      batch_size_default: 1,
      ..Default::default()
    },
    // Higgsfield-only: Nano Banana 2 at a fixed 1K with a High / Minimal
    // "thinking" menu (surfaced as quality High / Low).
    ImageModelConfig {
      model: ImageModel::NanoBanana2Lite,
      model_creator: ModelCreator::Google,
      full_name: "Nano Banana 2 Lite".to_string(),
      selector_name: "Nano Banana 2 Lite".to_string(),
      selector_description: "Fastest instructive editing".to_string(),
      selector_badges: strings(&["20 sec."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 20_000,
      can_edit_images: true,
      text_prompt_max_length: None,
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: NANO_BANANA_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Auto),
      quality_options: vec![CommonQuality::High, CommonQuality::Low],
      quality_default: Some(CommonQuality::High),
      batch_size_max: 4,
      batch_size_default: 1,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::NanoBanana,
      model_creator: ModelCreator::Google,
      full_name: "Nano Banana".to_string(),
      selector_name: "Nano Banana".to_string(),
      selector_description: "Fast instructive editing".to_string(),
      selector_badges: strings(&["25 sec."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 25_000,
      can_edit_images: true,
      text_prompt_max_length: None,
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: NANO_BANANA_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Auto),
      batch_size_max: 4,
      batch_size_default: 1,
      ..Default::default()
    },
  ]
}

// ── OpenAI ──

fn openai_models() -> Vec<ImageModelConfig> {
  vec![
    ImageModelConfig {
      model: ImageModel::GptImage2,
      model_creator: ModelCreator::OpenAi,
      full_name: "GPT Image 2".to_string(),
      selector_name: "GPT Image 2".to_string(),
      selector_description: "Smart with great text support".to_string(),
      selector_badges: strings(&["2 min."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 120_000,
      can_edit_images: true,
      text_prompt_max_length: None,
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: vec![
        CommonAspectRatio::Auto,
        CommonAspectRatio::Square,
        CommonAspectRatio::SquareHd,
        CommonAspectRatio::TallThreeByFour,
        CommonAspectRatio::TallNineBySixteen,
        CommonAspectRatio::WideFourByThree,
        CommonAspectRatio::WideSixteenByNine,
      ],
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Auto),
      // NB: GPT Image 2 has no native resolution; the backend emulates it.
      resolution_options: vec![CommonResolution::OneK, CommonResolution::TwoK, CommonResolution::ThreeK, CommonResolution::FourK],
      resolution_default: Some(CommonResolution::OneK),
      quality_options: OPENAI_QUALITIES.to_vec(),
      quality_default: Some(CommonQuality::High),
      batch_size_max: 4,
      batch_size_default: 1,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::GptImage1p5,
      model_creator: ModelCreator::OpenAi,
      full_name: "GPT Image 1.5".to_string(),
      selector_name: "GPT Image 1.5".to_string(),
      selector_description: "Faster, improved".to_string(),
      selector_badges: strings(&["60 sec."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 60_000,
      can_edit_images: true,
      text_prompt_max_length: None,
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: vec![CommonAspectRatio::Square, CommonAspectRatio::Wide, CommonAspectRatio::Tall],
      aspect_ratio_default: Some(CommonAspectRatio::Square),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Auto),
      quality_options: OPENAI_QUALITIES.to_vec(),
      quality_default: Some(CommonQuality::High),
      batch_size_max: 4,
      batch_size_default: 1,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::GptImage1,
      model_creator: ModelCreator::OpenAi,
      full_name: "GPT Image 1 (GPT-4o)".to_string(),
      selector_name: "GPT Image 1 (GPT-4o)".to_string(),
      selector_description: "Slow, but super smart".to_string(),
      selector_badges: strings(&["60 sec."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 60_000,
      can_edit_images: true,
      text_prompt_max_length: None,
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: vec![CommonAspectRatio::Square, CommonAspectRatio::WideThreeByTwo, CommonAspectRatio::TallTwoByThree],
      aspect_ratio_default: Some(CommonAspectRatio::Square),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Auto),
      quality_options: OPENAI_QUALITIES.to_vec(),
      quality_default: Some(CommonQuality::High),
      batch_size_max: 4,
      batch_size_default: 1,
      ..Default::default()
    },
  ]
}

// ── Bytedance ──

fn bytedance_models() -> Vec<ImageModelConfig> {
  vec![
    ImageModelConfig {
      model: ImageModel::Seedream5p0Pro,
      model_creator: ModelCreator::Bytedance,
      full_name: "Seedream 5.0 Pro".to_string(),
      selector_name: "Seedream 5.0 Pro".to_string(),
      selector_description: "Highest quality Seedream".to_string(),
      selector_badges: strings(&["60 sec."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 60_000,
      can_edit_images: true,
      text_prompt_max_length: Some(4000),
      image_refs_supported: true,
      image_refs_max: Some(14),
      aspect_ratio_options: SEEDREAM_5_PRO_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::Square),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Auto),
      resolution_options: vec![CommonResolution::OneK, CommonResolution::TwoK],
      resolution_default: Some(CommonResolution::TwoK),
      batch_size_max: 8,
      batch_size_default: 1,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::Seedream5p0ProUltra,
      model_creator: ModelCreator::Bytedance,
      full_name: "Seedream 5.0 Pro Ultra".to_string(),
      selector_name: "Seedream 5.0 Pro Ultra".to_string(),
      selector_description: "Highest quality Seedream".to_string(),
      selector_badges: strings(&["90 sec."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 90_000,
      can_edit_images: true,
      text_prompt_max_length: Some(4000),
      image_refs_supported: true,
      image_refs_max: Some(14),
      aspect_ratio_options: SEEDREAM_5_PRO_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::Square),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Auto),
      resolution_options: vec![CommonResolution::OneK, CommonResolution::TwoK],
      resolution_default: Some(CommonResolution::TwoK),
      batch_size_max: 8,
      batch_size_default: 1,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::Seedream5Lite,
      model_creator: ModelCreator::Bytedance,
      full_name: "Seedream 5 Lite".to_string(),
      selector_name: "Seedream 5 Lite".to_string(),
      selector_description: "Fast".to_string(),
      selector_badges: strings(&["60 sec."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 60_000,
      can_edit_images: true,
      text_prompt_max_length: Some(4000),
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: vec![
        CommonAspectRatio::Auto2k,
        CommonAspectRatio::Auto3k,
        CommonAspectRatio::Square,
        CommonAspectRatio::SquareHd,
        CommonAspectRatio::WideFourByThree,
        CommonAspectRatio::WideSixteenByNine,
        CommonAspectRatio::TallThreeByFour,
        CommonAspectRatio::TallNineBySixteen,
      ],
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Auto3k),
      batch_size_max: 4,
      batch_size_default: 1,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::Seedream4p5,
      model_creator: ModelCreator::Bytedance,
      full_name: "Seedream 4.5".to_string(),
      selector_name: "Seedream 4.5".to_string(),
      selector_description: "Fast".to_string(),
      selector_badges: strings(&["60 sec."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 60_000,
      can_edit_images: true,
      text_prompt_max_length: Some(4000),
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: vec![
        CommonAspectRatio::Auto2k,
        CommonAspectRatio::Auto4k,
        CommonAspectRatio::Square,
        CommonAspectRatio::SquareHd,
        CommonAspectRatio::WideFourByThree,
        CommonAspectRatio::WideSixteenByNine,
        CommonAspectRatio::TallThreeByFour,
        CommonAspectRatio::TallNineBySixteen,
      ],
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Auto2k),
      batch_size_max: 4,
      batch_size_default: 1,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::Seedream4,
      model_creator: ModelCreator::Bytedance,
      full_name: "Seedream 4".to_string(),
      selector_name: "Seedream 4".to_string(),
      selector_description: "Fast".to_string(),
      selector_badges: strings(&["60 sec."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 60_000,
      can_edit_images: true,
      text_prompt_max_length: Some(4000),
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: vec![
        CommonAspectRatio::Auto,
        CommonAspectRatio::Auto2k,
        CommonAspectRatio::Auto4k,
        CommonAspectRatio::Square,
        CommonAspectRatio::SquareHd,
        CommonAspectRatio::WideFourByThree,
        CommonAspectRatio::WideSixteenByNine,
        CommonAspectRatio::TallThreeByFour,
        CommonAspectRatio::TallNineBySixteen,
      ],
      aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
      aspect_ratio_default_when_editing: Some(CommonAspectRatio::Auto),
      batch_size_max: 4,
      batch_size_default: 1,
      ..Default::default()
    },
  ]
}

// ── Black Forest Labs ──

fn black_forest_labs_models() -> Vec<ImageModelConfig> {
  vec![
    ImageModelConfig {
      model: ImageModel::FluxPro11Ultra,
      model_creator: ModelCreator::BlackForestLabs,
      full_name: "FLUX 1.1 [pro] ultra".to_string(),
      selector_name: "Flux Pro 1.1 Ultra".to_string(),
      selector_description: "Higher quality model".to_string(),
      selector_badges: strings(&["35 sec."]),
      progress_bar_ms: 35_000,
      text_prompt_max_length: Some(4000),
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: vec![
        CommonAspectRatio::Square,
        CommonAspectRatio::WideThreeByTwo,
        CommonAspectRatio::WideFourByThree,
        CommonAspectRatio::WideSixteenByNine,
        CommonAspectRatio::WideTwentyOneByNine,
        CommonAspectRatio::TallTwoByThree,
        CommonAspectRatio::TallThreeByFour,
        CommonAspectRatio::TallNineBySixteen,
        CommonAspectRatio::TallNineByTwentyOne,
      ],
      aspect_ratio_default: Some(CommonAspectRatio::Square),
      batch_size_max: 4,
      batch_size_default: 4,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::FluxPro11,
      model_creator: ModelCreator::BlackForestLabs,
      full_name: "FLUX 1.1 [pro]".to_string(),
      selector_name: "Flux Pro 1.1".to_string(),
      selector_description: "High quality model".to_string(),
      selector_badges: strings(&["10 sec."]),
      progress_bar_ms: 10_000,
      text_prompt_max_length: Some(4000),
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: vec![
        CommonAspectRatio::Square,
        CommonAspectRatio::WideFourByThree,
        CommonAspectRatio::WideSixteenByNine,
        CommonAspectRatio::TallThreeByFour,
        CommonAspectRatio::TallNineBySixteen,
      ],
      aspect_ratio_default: Some(CommonAspectRatio::Square),
      batch_size_max: 4,
      batch_size_default: 4,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::Flux1Dev,
      model_creator: ModelCreator::BlackForestLabs,
      full_name: "FLUX.1 [dev]".to_string(),
      selector_name: "Flux 1 Dev".to_string(),
      selector_description: "Fast, but lower quality".to_string(),
      selector_badges: strings(&["10 sec."]),
      progress_bar_ms: 10_000,
      text_prompt_max_length: Some(4000),
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: FLUX_1_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::Square),
      batch_size_max: 4,
      batch_size_default: 4,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::Flux1Schnell,
      model_creator: ModelCreator::BlackForestLabs,
      full_name: "FLUX.1 [schnell]".to_string(),
      selector_name: "Flux 1 Schnell".to_string(),
      selector_description: "Fastest image gen, but lowest quality".to_string(),
      selector_badges: strings(&["10 sec."]),
      progress_bar_ms: 10_000,
      text_prompt_max_length: Some(4000),
      image_refs_supported: true,
      image_refs_max: Some(6),
      aspect_ratio_options: FLUX_1_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::Square),
      batch_size_max: 4,
      batch_size_default: 4,
      ..Default::default()
    },
  ]
}

// ── Midjourney ──

fn midjourney_models() -> Vec<ImageModelConfig> {
  let midjourney = |model: ImageModel, full_name: &str, description: &str| ImageModelConfig {
    model,
    model_creator: ModelCreator::Midjourney,
    full_name: full_name.to_string(),
    selector_name: full_name.to_string(),
    selector_description: description.to_string(),
    selector_badges: strings(&["45 sec."]),
    progress_bar_ms: 45_000,
    text_prompt_max_length: Some(6000),
    image_refs_supported: true,
    image_refs_max: Some(1),
    aspect_ratio_options: MIDJOURNEY_ASPECT_RATIOS.to_vec(),
    aspect_ratio_default: Some(CommonAspectRatio::Square),
    // NB: Midjourney produces a 2x2 grid per prompt.
    batch_size_min: 4,
    batch_size_max: 4,
    batch_size_options: Some(vec![4]),
    batch_size_default: 4,
    ..Default::default()
  };

  vec![
    midjourney(ImageModel::Midjourney8, "Midjourney v8", "Stunning style and quality"),
    midjourney(ImageModel::Midjourney7, "Midjourney v7", "Stunning style and quality"),
    midjourney(ImageModel::Midjourney7Niji, "Midjourney v7 Niji", "Anime style"),
  ]
}

// ── Grok (first-party imagine websocket; see `enqueue_image_generation.rs`) ──

fn grok_models() -> Vec<ImageModelConfig> {
  vec![
    ImageModelConfig {
      model: ImageModel::GrokImagineImage,
      model_creator: ModelCreator::Grok,
      full_name: "Grok Imagine".to_string(),
      selector_name: "Grok Imagine".to_string(),
      selector_description: "Fast image generation on your Grok account".to_string(),
      selector_badges: strings(&["10 sec."]),
      progress_bar_ms: 15_000,
      text_prompt_max_length: Some(4096),
      aspect_ratio_options: vec![
        CommonAspectRatio::Square,
        CommonAspectRatio::WideThreeByTwo,
        CommonAspectRatio::WideSixteenByNine,
        CommonAspectRatio::TallTwoByThree,
        CommonAspectRatio::TallNineBySixteen,
      ],
      aspect_ratio_default: Some(CommonAspectRatio::Square),
      // NB: Grok's imagine websocket returns two images per prompt.
      batch_size_min: 2,
      batch_size_max: 2,
      batch_size_options: Some(vec![2]),
      batch_size_default: 2,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::GrokImagineImageQuality,
      model_creator: ModelCreator::Grok,
      full_name: "Grok Imagine (Quality)".to_string(),
      selector_name: "Grok Imagine Quality".to_string(),
      selector_description: "Higher quality on your Grok account".to_string(),
      selector_badges: strings(&["30 sec."]),
      progress_bar_ms: 40_000,
      text_prompt_max_length: Some(4096),
      aspect_ratio_options: vec![
        CommonAspectRatio::Square,
        CommonAspectRatio::WideThreeByTwo,
        CommonAspectRatio::WideFourByThree,
        CommonAspectRatio::WideSixteenByNine,
        CommonAspectRatio::WideTwentyOneByNine,
        CommonAspectRatio::TallTwoByThree,
        CommonAspectRatio::TallNineBySixteen,
      ],
      aspect_ratio_default: Some(CommonAspectRatio::Square),
      batch_size_min: 2,
      batch_size_max: 2,
      batch_size_options: Some(vec![2]),
      batch_size_default: 2,
      ..Default::default()
    },
  ]
}

// ── Editing-only models (not on the text-to-image page) ──

fn editing_only_models() -> Vec<ImageModelConfig> {
  vec![
    ImageModelConfig {
      model: ImageModel::QwenEdit2511Angles,
      model_creator: ModelCreator::Alibaba,
      full_name: "Qwen Edit 2511 Angles".to_string(),
      selector_name: "Qwen Edit 2511 Angles".to_string(),
      selector_description: "Angle manipulation with optional prompt".to_string(),
      selector_badges: strings(&["30 sec."]),
      progress_bar_ms: 30_000,
      can_text_to_image: false,
      can_edit_angles: true,
      text_prompt_max_length: Some(800),
      image_refs_supported: true,
      image_refs_max: Some(1),
      aspect_ratio_options: ANGLES_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::SquareHd),
      batch_size_max: 4,
      batch_size_default: 1,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::Flux2LoraAngles,
      model_creator: ModelCreator::BlackForestLabs,
      full_name: "Flux 2 LoRA Angles".to_string(),
      selector_name: "Flux 2 LoRA Angles".to_string(),
      selector_description: "Angle manipulation".to_string(),
      selector_badges: strings(&["30 sec."]),
      progress_bar_ms: 30_000,
      can_text_to_image: false,
      can_edit_angles: true,
      text_prompt_max_length: Some(4000),
      image_refs_supported: true,
      image_refs_max: Some(1),
      aspect_ratio_options: ANGLES_ASPECT_RATIOS.to_vec(),
      aspect_ratio_default: Some(CommonAspectRatio::SquareHd),
      batch_size_max: 4,
      batch_size_default: 1,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::FluxProKontextMax,
      model_creator: ModelCreator::BlackForestLabs,
      full_name: "Flux Pro Kontext Max".to_string(),
      selector_name: "Flux Pro Kontext Max".to_string(),
      selector_description: "Fast instructive editing".to_string(),
      selector_badges: strings(&["20 sec."]),
      tags: vec![ModelTag::InstructiveEdit],
      progress_bar_ms: 20_000,
      can_text_to_image: false,
      can_edit_images: true,
      text_prompt_max_length: Some(4000),
      batch_size_max: 4,
      batch_size_default: 4,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::FluxPro1,
      model_creator: ModelCreator::BlackForestLabs,
      full_name: "Flux Pro Inpaint".to_string(),
      selector_name: "Flux Pro (Inpainting)".to_string(),
      selector_description: "Fast inpainting".to_string(),
      selector_badges: strings(&["30 sec."]),
      tags: vec![ModelTag::MaskedInpainting],
      progress_bar_ms: 30_000,
      can_text_to_image: false,
      can_edit_images: true,
      uses_inpainting_mask: true,
      editing_is_inpainting: true,
      text_prompt_max_length: Some(4000),
      // NB: FAL only allows one image for this model.
      batch_size_max: 1,
      batch_size_default: 1,
      ..Default::default()
    },
    ImageModelConfig {
      model: ImageModel::FluxDevJuggernaut,
      model_creator: ModelCreator::BlackForestLabs,
      full_name: "Flux Dev Juggernaut Inpaint".to_string(),
      selector_name: "Flux Dev Juggernaut".to_string(),
      selector_description: "Fast inpainting, low quality".to_string(),
      selector_badges: strings(&["10 sec."]),
      tags: vec![ModelTag::MaskedInpainting],
      progress_bar_ms: 10_000,
      can_text_to_image: false,
      can_edit_images: true,
      uses_inpainting_mask: true,
      editing_is_inpainting: true,
      text_prompt_max_length: Some(4000),
      batch_size_max: 4,
      batch_size_default: 4,
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
    let listed: Vec<ImageModel> = IMAGE_MODELS.iter().map(|c| c.model).collect();
    let unique: HashSet<ImageModel> = listed.iter().copied().collect();
    assert_eq!(listed.len(), unique.len(), "duplicate image model configs");
    for model in ImageModel::iter() {
      assert!(unique.contains(&model), "no config for {model:?}");
    }
  }

  #[test]
  fn defaults_are_within_their_options() {
    for config in IMAGE_MODELS.iter() {
      assert!(!config.full_name.is_empty() && !config.selector_name.is_empty(), "{:?} needs names", config.model);
      assert!(config.batch_size_min <= config.batch_size_default && config.batch_size_default <= config.batch_size_max, "{:?} batch sizes", config.model);
      if let Some(default) = config.aspect_ratio_default {
        assert!(config.aspect_ratio_options.contains(&default), "{:?} aspect default not offered", config.model);
      }
      if let Some(default) = config.resolution_default {
        assert!(config.resolution_options.contains(&default), "{:?} resolution default not offered", config.model);
      }
      if let Some(default) = config.quality_default {
        assert!(config.quality_options.contains(&default), "{:?} quality default not offered", config.model);
      }
    }
  }

  #[test]
  fn serializes_with_string_ids() {
    let json = serde_json::to_value(image_model_config(ImageModel::FluxPro11)).unwrap();
    assert_eq!(json["model"], "flux_pro_1p1");
    assert_eq!(json["model_creator"], "black_forest_labs");
    assert_eq!(json["aspect_ratio_default"], "square");
    // Unlimited prompt length is simply absent.
    let nb = serde_json::to_value(image_model_config(ImageModel::NanoBananaPro)).unwrap();
    assert!(nb.get("text_prompt_max_length").is_none());
  }
}
