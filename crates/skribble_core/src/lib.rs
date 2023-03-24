#![deny(clippy::all)]
#![forbid(clippy::indexing_slicing)]

pub use config::*;
pub use constants::*;
pub use css::*;
pub use error::*;
pub use plugin::*;
pub use runner::*;
pub use utils::*;

mod config;
mod constants;
mod css;
mod error;
mod plugin;
mod runner;
mod utils;
