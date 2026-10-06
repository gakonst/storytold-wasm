use serde_derive::{Deserialize, Serialize};

/// Every Gaussian splat ("world") model ArtCraftX knows about. The serde form
/// is the model id the frontend sends on `generate_splat_command` (1:1 with
/// the router's ids).
#[cfg_attr(test, derive(strum::EnumIter))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SplatModel {
  #[serde(rename = "marble_0p1_mini")]
  Marble0p1Mini,
  #[serde(rename = "marble_0p1_plus")]
  Marble0p1Plus,
  #[serde(rename = "marble_1p0")]
  Marble1p0,
  #[serde(rename = "marble_1p0_draft")]
  Marble1p0Draft,
  #[serde(rename = "marble_1p1")]
  Marble1p1,
  #[serde(rename = "marble_1p1_plus")]
  Marble1p1Plus,
  #[serde(rename = "triposplat")]
  TripoSplat,
}
