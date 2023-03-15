#![deny(clippy::all)]

use serde::Deserialize;
use serde::Serialize;
use skribble_core::*;
use typed_builder::TypedBuilder;

/// This plugin generates `rust` code from the configuration.
#[derive(Debug, Clone, Default, Deserialize, TypedBuilder, Serialize)]
pub struct RustPlugin {
  /// The formatter command.
  /// e.g. `dprint`
  #[builder(default, setter(into, strip_option))]
  pub formatter: Option<String>,

  /// The formatter arguments.
  /// e.g. `["fmt", "--stdin", "file.rs"]`
  #[builder(default, setter(into))]
  pub formatter_args: Vec<String>,
}

impl Plugin for RustPlugin {
  fn get_id(&self) -> String {
    "skribble_plugin_rust".into()
  }

  fn generate_code(&self, _config: &MergedConfig) -> AnyResult<GeneratedFiles> {
    let mut files = GeneratedFiles::default();

    files.push(
      GeneratedFile::builder()
        .path("./src/skribble.rs")
        .content(
          "pub struct SkribbleRoot(String);\npub fn sk() -> SkribbleRoot {\n  \
           SkribbleRoot(\"sample\")\n}",
        )
        .build(),
    );

    Ok(files)
  }

  fn get_description(&self) -> String {
    "This plugin provides support for generating rust code from your `skribble` configuration."
      .into()
  }
}

impl RustPlugin {}

#[cfg(test)]
mod tests {
  use skribble_core::*;
  use skribble_preset_default::PresetDefault;

  use super::*;

  #[test]
  fn default_can_be_added_to_runner() {
    let default_preset = PresetDefault::builder().build();
    let rust_plugin = RustPlugin::builder().build();

    let config: StyleConfig = StyleConfig::builder()
      .plugins(vec![
        PluginContainer::from(default_preset),
        PluginContainer::from(rust_plugin),
      ])
      .build();

    let mut runner = SkribbleRunner::new(config);
    let _ = runner.run();
    let _ = runner.generate();
  }
}
