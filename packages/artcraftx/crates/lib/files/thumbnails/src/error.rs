use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ThumbnailError {
  Io(std::io::Error),

  /// The file isn't a container/codec we can read (e.g. webm, HEVC). The
  /// caller should show a placeholder, not retry.
  Unsupported(String),

  /// The container parsed but no decodable frame came out.
  Decode(String),

  Encode(String),
}

impl ThumbnailError {
  pub fn is_unsupported(&self) -> bool {
    matches!(self, Self::Unsupported(_))
  }
}

impl Error for ThumbnailError {}

impl Display for ThumbnailError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Io(err) => write!(f, "Thumbnail I/O error: {}", err),
      Self::Unsupported(message) => write!(f, "Unsupported media for thumbnailing: {}", message),
      Self::Decode(message) => write!(f, "Could not decode media: {}", message),
      Self::Encode(message) => write!(f, "Could not encode thumbnail: {}", message),
    }
  }
}

impl From<std::io::Error> for ThumbnailError {
  fn from(err: std::io::Error) -> Self {
    Self::Io(err)
  }
}
