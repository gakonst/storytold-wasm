//! thumbnails
//!
//! Pure-Rust thumbnail generation for local media files: static JPEG
//! thumbnails for images and videos, plus a ~1-second animated WebP preview
//! for videos. Video support covers H.264 in MP4 — what every current
//! provider emits — via vendored decoders (no ffmpeg, no runtime deps on
//! the customer machine). Anything else returns a typed "unsupported"
//! error for the caller to placeholder.

// Never allow these
#![forbid(private_bounds)]
#![forbid(private_interfaces)]
#![forbid(unused_must_use)]

// Okay to toggle
#![forbid(unreachable_patterns)]
#![forbid(unused_imports)]
#![forbid(unused_mut)]
#![forbid(unused_variables)]

// Always allow
#![allow(dead_code)]

pub mod error;
pub mod image_thumbnail;
#[cfg(feature = "video")]
pub mod mp4_frames;
#[cfg(feature = "video")]
pub mod video_thumbnail;

pub use error::ThumbnailError;
pub use image_thumbnail::generate_image_thumbnail;
#[cfg(feature = "video")]
pub use video_thumbnail::generate_video_thumbnails;

/// Longest edge of static thumbnails (matches the frontend's
/// `THUMBNAIL_SIZES.LARGE`).
pub const STATIC_THUMBNAIL_MAX_DIMENSION: u32 = 512;

/// Longest edge of animated previews (kept small; these hold ~24 frames).
pub const ANIMATED_PREVIEW_MAX_DIMENSION: u32 = 320;

/// How much leading video the animated preview covers.
pub const ANIMATED_PREVIEW_MAX_DURATION_MS: u32 = 3_100;

/// Cap on frames decoded for the animated preview (~3s at 30fps). Decoding
/// stops at whichever of this and the duration comes first.
pub const ANIMATED_PREVIEW_MAX_DECODED_FRAMES: usize = 96;

/// Frame rate the animated preview is subsampled to. Half of the usual
/// 24fps keeps motion smooth at a third of the frames.
pub const ANIMATED_PREVIEW_TARGET_FPS: u32 = 12;

/// Lossy WebP quality (0-100) for the animated preview. At 320px, 70 is
/// visually clean and ~10x smaller than lossless.
pub const ANIMATED_PREVIEW_QUALITY: f32 = 70.0;

/// Bump when the thumbnail output changes (size, duration, quality, ...) so
/// the thumbnail worker regenerates cache entries made by older versions.
///
/// History:
///   1 - 512px jpg + ~1s lossless 320px webp (decoder truncated B-frame
///       streams to a few frames)
///   2 - ~3s 12fps lossy-q70 webp; B-frame streams decode fully
pub const THUMBNAIL_GENERATOR_VERSION: i64 = 2;
