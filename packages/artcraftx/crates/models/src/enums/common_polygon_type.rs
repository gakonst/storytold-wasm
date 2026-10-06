use serde_derive::{Deserialize, Serialize};

/// Mesh polygon types a model can be asked for.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommonPolygonType {
  Triangle,
  Quad,
}
