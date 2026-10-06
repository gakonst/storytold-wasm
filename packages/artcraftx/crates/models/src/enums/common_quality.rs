use serde_derive::{Deserialize, Serialize};

/// Quality tiers (used by the OpenAI image models).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommonQuality {
  High,
  Medium,
  Low,
}
