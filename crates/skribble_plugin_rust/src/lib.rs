#![deny(clippy::all)]
#![deny(clippy::indexing_slicing)]

use std::process::Command;
use std::process::Stdio;

use generate::generate_file_contents;
use heck::ToPascalCase;
use heck::ToSnakeCase;
use indexmap::indexmap;
use indexmap::indexset;
use indoc::indoc;
use serde::Deserialize;
use serde::Serialize;
use skribble_core::*;
use typed_builder::TypedBuilder;

mod generate;

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

  fn generate_code(&self, config: &MergedConfig, options: &Options) -> AnyResult<GeneratedFiles> {
    let mut contents = generate_file_contents(config, options);

    if let Some(ref formatter) = self.formatter {
      let input = Command::new("echo")
        .arg(&contents)
        .stdout(Stdio::piped())
        .spawn()?;

      if let Some(stdout) = input.stdout {
        let output = Command::new(formatter)
          .args(&self.formatter_args)
          .stdin(stdout)
          .stdout(std::process::Stdio::piped())
          .output()?;
        contents = String::from_utf8(output.stdout)?;
      }
    }

    let mut files = GeneratedFiles::default();
    files.push(
      GeneratedFile::builder()
        .path("./src/skribble.rs")
        .content(contents)
        .build(),
    );

    Ok(files)
  }

  fn get_description(&self) -> String {
    "This plugin provides support for generating rust code from your `skribble` configuration."
      .into()
  }
}

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
    let result = runner.generate().unwrap();
    let generated = result.get(0).unwrap();
    let content = &generated.content;
    insta::assert_display_snapshot!(content);
  }
}
