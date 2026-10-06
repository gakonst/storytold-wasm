use serde_derive::{Deserialize, Serialize};

/// Every 3D mesh model ArtCraftX knows about. The serde form is the model id
/// the frontend sends on `generate_mesh_command` (1:1 with the router's ids).
#[cfg_attr(test, derive(strum::EnumIter))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeshModel {
  #[serde(rename = "hunyuan_3d_2p0")]
  Hunyuan3d2p0,
  #[serde(rename = "hunyuan_3d_2p1")]
  Hunyuan3d2p1,
  #[serde(rename = "hunyuan_3d_3")]
  Hunyuan3d3,
  #[serde(rename = "hunyuan_3d_3_sketch")]
  Hunyuan3d3Sketch,
  #[serde(rename = "hunyuan_3d_3p1_pro")]
  Hunyuan3d3p1Pro,
  #[serde(rename = "hunyuan_3d_3p1_rapid")]
  Hunyuan3d3p1Rapid,
  #[serde(rename = "hunyuan_3d_3p1_part")]
  Hunyuan3d3p1Part,
  #[serde(rename = "hunyuan_3d_3p1_topology")]
  Hunyuan3d3p1SmartTopology,
  #[serde(rename = "tripo3d_h3p1")]
  Tripo3dH3p1,
  #[serde(rename = "meshy_v6")]
  MeshyV6,
  #[serde(rename = "rodin_2p5_fast")]
  Rodin2p5Fast,
}
