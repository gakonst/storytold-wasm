use serde_derive::{Deserialize, Serialize};

/// A service that can run a generation. What the picker offers next to a
/// model (and what the frontend sends as `provider`).
///
/// This is the user-facing provider set; the router's `RouterProvider` is
/// the dispatch-level set (it also has `GrokApi`, `GmiCloud`, `Seedance2Pro`).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationProvider {
  Artcraft,
  Fal,
  /// First-party (cookie-session) Grok.
  Grok,
  /// First-party (cookie-session) Higgsfield.
  Higgsfield,
  /// First-party (cookie-session) Midjourney.
  Midjourney,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serializes_to_frontend_values() {
    assert_eq!(serde_json::to_string(&GenerationProvider::Artcraft).unwrap(), "\"artcraft\"");
    assert_eq!(serde_json::to_string(&GenerationProvider::Higgsfield).unwrap(), "\"higgsfield\"");
    assert_eq!(serde_json::from_str::<GenerationProvider>("\"higgsfield\"").unwrap(), GenerationProvider::Higgsfield);
  }
}
