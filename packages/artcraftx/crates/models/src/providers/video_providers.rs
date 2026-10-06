//! Which providers offer which video models.

use crate::configs::video_model_config::VideoModelConfig;
use crate::enums::generation_provider::GenerationProvider;
use crate::enums::video_model::VideoModel;
use crate::providers::higgsfield_video_offering::higgsfield_video_offering;
use crate::providers::provider_offering::{is_offered, providers_for_model, ProviderOffering};
use once_cell::sync::Lazy;

pub type VideoProviderOffering = ProviderOffering<VideoModel, VideoModelConfig>;

pub static VIDEO_PROVIDERS: Lazy<Vec<VideoProviderOffering>> = Lazy::new(video_providers);

pub fn providers_for_video_model(model: VideoModel) -> Vec<GenerationProvider> {
  providers_for_model(&VIDEO_PROVIDERS, model)
}

pub fn provider_offers_video_model(provider: GenerationProvider, model: VideoModel) -> bool {
  is_offered(&VIDEO_PROVIDERS, provider, model)
}

fn video_providers() -> Vec<VideoProviderOffering> {
  vec![
    // ArtCraft runs every video model except the Higgsfield-only ones (Grok
    // Imagine video is served through storyteller-web too). Grok and
    // Midjourney offer no video models.
    VideoProviderOffering::of(GenerationProvider::Artcraft, &[
      VideoModel::Seedance2p0,
      VideoModel::Seedance2p0Fast,
      VideoModel::Seedance2p0BytePlus,
      VideoModel::Seedance2p0BytePlusFast,
      VideoModel::Seedance2p0Ultra,
      VideoModel::Seedance2p0UltraFast,
      VideoModel::Seedance2p0BytePlusUltra,
      VideoModel::Seedance2p0BytePlusUltraFast,
      VideoModel::Seedance2p0Mini,
      VideoModel::Seedance2p0BytePlusMini,
      VideoModel::Seedance2p0BytePlusUltraMini,
      VideoModel::Seedance1p5Pro,
      VideoModel::Seedance10Lite,
      VideoModel::Kling3p0Pro,
      VideoModel::Kling3p0Standard,
      VideoModel::Kling2p6Pro,
      VideoModel::Kling2p5TurboPro,
      VideoModel::Kling21Master,
      VideoModel::Kling21Pro,
      VideoModel::Kling16Pro,
      VideoModel::Veo3p1,
      VideoModel::Veo3p1Fast,
      VideoModel::Veo3p1Lite,
      VideoModel::Veo3Fast,
      VideoModel::Veo3,
      VideoModel::Veo2,
      VideoModel::ViduQ3,
      VideoModel::ViduQ3Turbo,
      VideoModel::HappyHorse1p0,
      VideoModel::Sora2,
      VideoModel::Sora2Pro,
      VideoModel::GrokImagineVideo,
      VideoModel::GrokImagineVideo1p5,
      VideoModel::SwitchX,
    ]),
    // First-party (cookie-session) Higgsfield, with per-model overrides where
    // its menus differ from ArtCraft's.
    higgsfield_video_offering(),
  ]
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::configs::video_models::VIDEO_MODELS;
  use crate::providers::tests_common::check_offerings;

  #[test]
  fn offerings_are_consistent_with_the_model_table() {
    let known: Vec<VideoModel> = VIDEO_MODELS.iter().filter(|c| !c.is_disabled).map(|c| c.model).collect();
    check_offerings(&VIDEO_PROVIDERS, &known, |config| config.model);
  }

  #[test]
  fn grok_and_midjourney_offer_no_video() {
    assert!(!VIDEO_PROVIDERS.iter().any(|o| o.provider == GenerationProvider::Grok));
    assert!(!VIDEO_PROVIDERS.iter().any(|o| o.provider == GenerationProvider::Midjourney));
    assert_eq!(providers_for_video_model(VideoModel::Sora2), vec![GenerationProvider::Artcraft]);
    assert!(provider_offers_video_model(GenerationProvider::Artcraft, VideoModel::GrokImagineVideo));
    assert_eq!(providers_for_video_model(VideoModel::Seedance2p0), vec![GenerationProvider::Artcraft, GenerationProvider::Higgsfield]);
  }
}
