use serde_derive::{Deserialize, Serialize};

/// What a mesh model can emit.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommonMeshOutputType {
  Normal,
  LowPoly,
  Geometry,
}
