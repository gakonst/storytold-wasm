//! # models
//!
//! The single source of truth for every generation model ArtCraftX knows
//! about — image, video, mesh, splat, and audio — and what each one can do.
//!
//! - [`enums`]: the model identifiers (one enum per modality), plus the
//!   shared vocabulary they're described with (providers, creators, aspect
//!   ratios, resolutions, ...). These are first-party types: nothing here
//!   depends on a third-party API client or on the router.
//! - [`configs`]: one config struct per modality describing capabilities
//!   (what inputs/options a model accepts) and desktop presentation (picker
//!   copy, badges, which providers can run it, which pages show it), and the
//!   built-in tables of every model.
//! - [`providers`]: which providers (services) offer which models, with room
//!   for per-provider overrides.
//!
//! The Tauri list commands serve these tables to the frontend verbatim, so
//! the wire format is the serde form of the structs in [`configs`]. Enum
//! serializations are the identifiers the frontend sends back on generate
//! requests — NEVER change an existing one; only add.

pub mod configs;
pub mod enums;
pub mod providers;
