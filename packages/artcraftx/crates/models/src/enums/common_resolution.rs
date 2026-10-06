use serde_derive::{Deserialize, Serialize};

/// Output resolutions. Image models use the `*K` tiers; video models use the
/// `*P` tiers (plus 2K/4K where offered).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommonResolution {
  HalfK,
  OneK,
  TwoK,
  ThreeK,
  FourK,
  FourEightyP,
  SevenTwentyP,
  TenEightyP,
}
