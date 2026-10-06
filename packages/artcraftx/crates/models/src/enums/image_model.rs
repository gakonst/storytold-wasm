use serde_derive::{Deserialize, Serialize};

/// Every image model ArtCraftX knows about. The serde form is the model id
/// the frontend sends on `generate_image_command` (and the router's ids).
///
/// Roughly 1:1 with `router::api::RouterImageModel`, plus the desktop-only
/// editing / inpainting models the router doesn't dispatch.
#[cfg_attr(test, derive(strum::EnumIter))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageModel {
  // ── Black Forest Labs ──
  #[serde(rename = "flux_1_dev")]
  Flux1Dev,
  #[serde(rename = "flux_1_schnell")]
  Flux1Schnell,
  #[serde(rename = "flux_pro_1p1")]
  FluxPro11,
  #[serde(rename = "flux_pro_1p1_ultra")]
  FluxPro11Ultra,
  #[serde(rename = "flux_pro_kontext_max")]
  FluxProKontextMax,
  /// Inpainting.
  #[serde(rename = "flux_pro_1")]
  FluxPro1,
  /// Inpainting.
  #[serde(rename = "flux_dev_juggernaut")]
  FluxDevJuggernaut,
  #[serde(rename = "flux_2_lora_angles")]
  Flux2LoraAngles,
  // ── OpenAI ──
  #[serde(rename = "gpt_image_1")]
  GptImage1,
  #[serde(rename = "gpt_image_1p5")]
  GptImage1p5,
  #[serde(rename = "gpt_image_2")]
  GptImage2,
  // ── Grok (first-party imagine) ──
  #[serde(rename = "grok_imagine_image")]
  GrokImagineImage,
  #[serde(rename = "grok_imagine_image_q")]
  GrokImagineImageQuality,
  // ── Midjourney ──
  #[serde(rename = "midjourney_7")]
  Midjourney7,
  #[serde(rename = "midjourney_7_niji")]
  Midjourney7Niji,
  #[serde(rename = "midjourney_8")]
  Midjourney8,
  // ── Google ──
  #[serde(rename = "nano_banana")]
  NanoBanana,
  #[serde(rename = "nano_banana_2")]
  NanoBanana2,
  /// Higgsfield's Nano Banana 2 Lite pipeline (quality tiers instead of a
  /// resolution menu).
  #[serde(rename = "nano_banana_2_lite")]
  NanoBanana2Lite,
  #[serde(rename = "nano_banana_pro")]
  NanoBananaPro,
  // ── Bytedance ──
  #[serde(rename = "seedream_4")]
  Seedream4,
  #[serde(rename = "seedream_4p5")]
  Seedream4p5,
  #[serde(rename = "seedream_5_lite")]
  Seedream5Lite,
  #[serde(rename = "seedream_5p0_pro")]
  Seedream5p0Pro,
  #[serde(rename = "seedream_5p0_pro_u")]
  Seedream5p0ProUltra,
  // ── Alibaba ──
  #[serde(rename = "qwen_edit_2511_angles")]
  QwenEdit2511Angles,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ids_match_the_router_and_frontend() {
    assert_eq!(serde_json::to_string(&ImageModel::FluxPro11).unwrap(), "\"flux_pro_1p1\"");
    assert_eq!(serde_json::to_string(&ImageModel::GrokImagineImageQuality).unwrap(), "\"grok_imagine_image_q\"");
    assert_eq!(serde_json::to_string(&ImageModel::Seedream5p0ProUltra).unwrap(), "\"seedream_5p0_pro_u\"");
    assert_eq!(serde_json::from_str::<ImageModel>("\"flux_pro_1\"").unwrap(), ImageModel::FluxPro1);
    assert_eq!(serde_json::to_string(&ImageModel::NanoBanana2Lite).unwrap(), "\"nano_banana_2_lite\"");
  }
}
