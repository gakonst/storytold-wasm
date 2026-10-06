use serde_derive::{Deserialize, Serialize};

/// Every audio model ArtCraftX knows about. The serde form is the model id
/// the frontend sends on `generate_audio_command` (1:1 with the router's ids).
#[cfg_attr(test, derive(strum::EnumIter))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioModel {
  #[serde(rename = "suno_music")]
  SunoMusic,
  #[serde(rename = "suno_remix")]
  SunoRemix,
  #[serde(rename = "suno_sounds")]
  SunoSounds,
  #[serde(rename = "suno_sample")]
  SunoSample,
  #[serde(rename = "seed_audio_1p0")]
  SeedAudio1p0,
}
