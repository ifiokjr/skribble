#![deny(clippy::all)]
#![forbid(clippy::indexing_slicing)]

doc_comment::doctest!("../readme.md");
pub use config::*;
pub use constants::*;
pub use css::*;
pub use error::*;
pub use indexmap::indexmap as map;
pub use indexmap::indexset as set;
pub use macros::*;
pub use plugin::*;
pub use runner::*;
pub use traits::*;
pub use utils::*;

mod config;
mod constants;
mod css;
mod error;
mod macros;
mod plugin;
mod runner;
mod traits;
mod utils;
