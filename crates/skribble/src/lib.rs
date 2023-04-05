#![deny(clippy::all)]
#![forbid(clippy::indexing_slicing)]

doc_comment::doctest!("../readme.md");

/// Re-export `skribble_core` as `skribble::core`.
pub use skribble_core as core;
