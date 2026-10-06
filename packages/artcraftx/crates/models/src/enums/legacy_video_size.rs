use serde_derive::{Deserialize, Serialize};

/// The coarse size vocabulary of the first-party Grok and Sora video paths,
/// which predate [`CommonAspectRatio`](super::common_aspect_ratio::CommonAspectRatio).
/// The frontend sends these as `grok_aspect_ratio` / `sora_orientation`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegacyVideoSize {
  Landscape,
  Portrait,
  Square,
}
