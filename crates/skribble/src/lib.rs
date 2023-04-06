#![deny(clippy::all)]
#![forbid(clippy::indexing_slicing)]

doc_comment::doctest!("../readme.md");

pub use skribble_core as core;
pub use skribble_preset as preset;
pub use skribble_rust as rust;

pub use crate::core::Error;
use crate::core::PluginContainer;
pub use crate::core::Result;
use crate::core::SkribbleRunner;
use crate::core::StyleConfig;
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
pub fn run(config: StyleConfig) -> Result<SkribbleRunner> {
  let mut runner = SkribbleRunner::try_new(config)?;
  let _ = runner.initialize()?;
  let _generate_files = runner.generate()?; // Write the files to their destinations.
  let _css_result = runner.scan()?; // Write the css file to the destination.

  Ok(runner)
}
