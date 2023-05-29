#![deny(clippy::all)]
#![forbid(clippy::indexing_slicing)]

doc_comment::doctest!("../readme.md");

use std::path::Path;

pub use skribble_core as core;
pub use skribble_core::vfs;
pub use skribble_preset as preset;
pub use skribble_rust as rust;

pub use crate::core::Error;
use crate::core::PluginContainer;
pub use crate::core::Result;
use crate::core::SkribbleRunner;
use crate::core::StyleConfig;
use crate::core::VfsPath;
use crate::preset::PresetPlugin;
use crate::rust::RustPlugin;

/// Create a default `StyleConfig` with the `PresetPlugin` and `RustPlugin`
/// builtin.
///
/// The plugins can be overriden since the `StyleConfig` can be mutated with
/// methods like `config.add_plugins()` and `config.remove_plugin()`.
pub fn create_config() -> StyleConfig {
  let default_preset = PresetPlugin::builder().build();
  let rust_plugin = RustPlugin::builder().build();

  let config: StyleConfig = StyleConfig::builder()
    .plugins(vec![
      PluginContainer::from(default_preset),
      PluginContainer::from(rust_plugin),
    ])
    .build();

  config
}

/// Generate the files that created by plugins and also the css files.
pub fn run_with_config(config: StyleConfig) -> Result<SkribbleRunner> {
  let mut runner = SkribbleRunner::try_new(config)?;
  let _ = runner.initialize()?;
  let _generate_files = runner.generate()?; // Write the files to their destinations.
  let _css_result = runner.scan()?; // Write the css file to the destination.

  Ok(runner)
}

/// Generate the files that created by plugins and also the css files.
///
/// This is likely to change a lot in the future.
pub fn run(
  config: StyleConfig,
  cwd: impl AsRef<Path>,
  vfs: Option<VfsPath>,
) -> Result<SkribbleRunner> {
  let mut runner = SkribbleRunner::new(config, cwd, vfs);
  let _ = runner.initialize()?;
  let mut generated_files = runner.generate()?;
  let css_result = runner.scan()?;

  // Write the files to their destinations.
  runner.write_files(&mut generated_files)?;
  // Write the css file to the destination.
  runner.write_css(&css_result)?;

  Ok(runner)
}

#[cfg(test)]
mod __tests;
