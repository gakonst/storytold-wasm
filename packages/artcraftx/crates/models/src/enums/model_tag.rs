use serde_derive::{Deserialize, Serialize};

/// Filterable capability tags the frontend uses to pick which models appear
/// in which editor. Serialized in camelCase to match the frontend's values.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModelTag {
  /// Shows up in the 2D and 3D editors.
  InstructiveEdit,
  /// Shows up in the inpainting editor and uses a mask.
  MaskedInpainting,
  /// Shows up in the inpainting editor without a mask.
  NonMaskedInpainting,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serializes_to_camel_case() {
    assert_eq!(serde_json::to_string(&ModelTag::InstructiveEdit).unwrap(), "\"instructiveEdit\"");
  }
}
