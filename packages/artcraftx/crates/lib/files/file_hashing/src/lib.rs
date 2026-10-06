//! Content hashing for local files.
//!
//! BLAKE3 is the house hash for file identity: faster than the disk it
//! reads from, 256-bit, and cryptographically collision-resistant — safe to
//! use as a content address (thumbnail cache keys, upload-dedup ledgers).
//! Column/field naming should carry the family (`file_hash_blake3`) so a
//! second family can coexist later.

pub mod hash_file;

pub use hash_file::{hash_bytes_blake3, hash_file_blake3};
