use serde_derive::{Deserialize, Serialize};

/// Who made a model (for icons and grouping in the picker).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelCreator {
  /// Qwen, Wan, Happy Horse
  Alibaba,
  #[serde(rename = "artcraft")]
  ArtCraft,
  Beeble,
  BlackForestLabs,
  Bytedance,
  /// Rodin
  Deemos,
  Fal,
  Google,
  Grok,
  Hailuo,
  Higgsfield,
  Kling,
  Krea,
  Meshy,
  Midjourney,
  OpenAi,
  OpenArt,
  Recraft,
  Replicate,
  Runway,
  Stability,
  Suno,
  /// Hunyuan
  Tencent,
  TensorArt,
  Tripo,
  Vidu,
  WorldLabs,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serializes_to_snake_case() {
    assert_eq!(serde_json::to_string(&ModelCreator::BlackForestLabs).unwrap(), "\"black_forest_labs\"");
    assert_eq!(serde_json::to_string(&ModelCreator::ArtCraft).unwrap(), "\"artcraft\"");
    assert_eq!(serde_json::to_string(&ModelCreator::OpenAi).unwrap(), "\"open_ai\"");
  }
}
