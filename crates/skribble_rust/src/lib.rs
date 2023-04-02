#![deny(clippy::all)]
#![deny(clippy::indexing_slicing)]

use std::path::Path;
use std::process::Command;
use std::process::Stdio;

use generate::generate_file_contents;
use heck::ToPascalCase;
use heck::ToSnakeCase;
use indexmap::IndexMap;
use indoc::indoc;
use scan::scan;
use serde::Deserialize;
use serde::Serialize;
use skribble_core::AnyResult;
use skribble_core::Classes;
use skribble_core::GeneratedFile;
use skribble_core::GeneratedFiles;
use skribble_core::Plugin;
use skribble_core::PluginData;
use skribble_core::RunnerConfig;
use typed_builder::TypedBuilder;

mod error;
mod generate;
mod scan;

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
  /// The method names used in the generated code. This is also used to remap
  /// method names to the stored names.
  #[builder(default, setter(skip))]
  #[serde(skip)]
  method_names: IndexMap<String, String>,
}

impl Plugin for RustPlugin {
  fn get_data(&self) -> PluginData {
    PluginData::builder()
      .id("skribble_rust")
      .name("Rust Plugin")
      .globs(["**/*.rs".into()])
      .description(
        "This plugin provides support for generating rust code from your `skribble` configuration.",
      )
      .build()
  }

  fn generate_code(&mut self, config: &RunnerConfig) -> AnyResult<GeneratedFiles> {
    let (mut contents, method_names) = generate_file_contents(config)?;
    self.method_names = method_names;

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
        let result = String::from_utf8(output.stdout)?;

        contents = if result.trim().is_empty() && !contents.trim().is_empty() {
          contents
        } else {
          result
        }
      }
    }

    let mut files = GeneratedFiles::default();
    files.insert(
      GeneratedFile::builder()
        .path("./src/skribble.rs")
        .content(contents)
        .build(),
    );

    Ok(files)
  }

  fn scan_code(
    &mut self,
    config: &RunnerConfig,
    file_path: &Path,
    bytes: Vec<u8>,
  ) -> AnyResult<Classes> {
    scan(config, file_path, bytes)
  }
}

#[cfg(test)]
mod __tests;
