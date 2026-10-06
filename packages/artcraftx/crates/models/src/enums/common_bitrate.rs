use serde_derive::{Deserialize, Serialize};

/// Video bitrate tiers (Seedance 2.0).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommonBitrate {
  Normal,
  High,
}
