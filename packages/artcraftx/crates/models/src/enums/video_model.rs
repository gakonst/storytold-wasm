use serde_derive::{Deserialize, Serialize};

/// Every video model ArtCraftX knows about. The serde form is the model id
/// the frontend sends on `generate_video_command` (and the router's ids).
///
/// Roughly 1:1 with `router::api::RouterVideoModel`, plus desktop-only
/// models (Beeble SwitchX).
#[cfg_attr(test, derive(strum::EnumIter))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VideoModel {
  // ── Grok ──
  /// First-party Grok Imagine video (cookie session).
  #[serde(rename = "grok_imagine_video")]
  GrokImagineVideo,
  #[serde(rename = "grok_imagine_video_1p5")]
  GrokImagineVideo1p5,
  // ── Kling ──
  #[serde(rename = "kling_1p6_pro")]
  Kling16Pro,
  #[serde(rename = "kling_2p1_pro")]
  Kling21Pro,
  #[serde(rename = "kling_2p1_master")]
  Kling21Master,
  #[serde(rename = "kling_2p5_turbo_pro")]
  Kling2p5TurboPro,
  #[serde(rename = "kling_2p6_pro")]
  Kling2p6Pro,
  #[serde(rename = "kling_3p0_standard")]
  Kling3p0Standard,
  #[serde(rename = "kling_3p0_pro")]
  Kling3p0Pro,
  // ── Bytedance ──
  #[serde(rename = "seedance_1p0_lite")]
  Seedance10Lite,
  #[serde(rename = "seedance_1p5_pro")]
  Seedance1p5Pro,
  #[serde(rename = "seedance_2p0")]
  Seedance2p0,
  #[serde(rename = "seedance_2p0_fast")]
  Seedance2p0Fast,
  #[serde(rename = "seedance_2p0_bp")]
  Seedance2p0BytePlus,
  #[serde(rename = "seedance_2p0_bp_fast")]
  Seedance2p0BytePlusFast,
  #[serde(rename = "seedance_2p0_u")]
  Seedance2p0Ultra,
  #[serde(rename = "seedance_2p0_u_fast")]
  Seedance2p0UltraFast,
  #[serde(rename = "seedance_2p0_bpu")]
  Seedance2p0BytePlusUltra,
  #[serde(rename = "seedance_2p0_bpu_fast")]
  Seedance2p0BytePlusUltraFast,
  #[serde(rename = "seedance_2p0_mini")]
  Seedance2p0Mini,
  #[serde(rename = "seedance_2p0_bp_mini")]
  Seedance2p0BytePlusMini,
  #[serde(rename = "seedance_2p0_bpu_mini")]
  Seedance2p0BytePlusUltraMini,
  #[serde(rename = "seedance_2p5")]
  Seedance2p5,
  /// Seedance 2.5 video-to-video editing: rewrites a source video clip.
  #[serde(rename = "seedance_2p5_edit")]
  Seedance2p5Edit,
  // ── MiniMax (Hailuo) ──
  #[serde(rename = "minimax_h3")]
  MinimaxH3,
  // ── Alibaba ──
  #[serde(rename = "happy_horse_1p0")]
  HappyHorse1p0,
  // ── OpenAI ──
  #[serde(rename = "sora_2")]
  Sora2,
  #[serde(rename = "sora_2_pro")]
  Sora2Pro,
  // ── Google ──
  #[serde(rename = "veo_2")]
  Veo2,
  #[serde(rename = "veo_3")]
  Veo3,
  #[serde(rename = "veo_3_fast")]
  Veo3Fast,
  #[serde(rename = "veo_3p1")]
  Veo3p1,
  #[serde(rename = "veo_3p1_fast")]
  Veo3p1Fast,
  #[serde(rename = "veo_3p1_lite")]
  Veo3p1Lite,
  // ── Vidu ──
  #[serde(rename = "vidu_q3")]
  ViduQ3,
  #[serde(rename = "vidu_q3_turbo")]
  ViduQ3Turbo,
  // ── Beeble ──
  /// Background change / relighting VFX.
  #[serde(rename = "switch_x")]
  SwitchX,
  // ── Preview ──
  #[serde(rename = "preview_model")]
  PreviewModel,
  #[serde(rename = "preview_model_fast")]
  PreviewModelFast,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ids_match_the_router_and_frontend() {
    assert_eq!(serde_json::to_string(&VideoModel::Kling16Pro).unwrap(), "\"kling_1p6_pro\"");
    assert_eq!(serde_json::to_string(&VideoModel::Seedance10Lite).unwrap(), "\"seedance_1p0_lite\"");
    assert_eq!(serde_json::to_string(&VideoModel::Seedance2p0BytePlusUltraMini).unwrap(), "\"seedance_2p0_bpu_mini\"");
    assert_eq!(serde_json::from_str::<VideoModel>("\"switch_x\"").unwrap(), VideoModel::SwitchX);
    assert_eq!(serde_json::to_string(&VideoModel::Seedance2p5).unwrap(), "\"seedance_2p5\"");
    assert_eq!(serde_json::to_string(&VideoModel::Seedance2p5Edit).unwrap(), "\"seedance_2p5_edit\"");
    assert_eq!(serde_json::to_string(&VideoModel::MinimaxH3).unwrap(), "\"minimax_h3\"");
  }
}
