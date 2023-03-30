use std::path::Path;

use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::config::*;
use crate::plugin::AnyEmptyResult;
use crate::plugin::AnyResult;
use crate::plugin::GeneratedFiles;
use crate::plugin::PluginConfig;
use crate::runner::RunnerConfig;
use crate::Classes;

/// Used to read the data for each plugin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct PluginData {
  /// Store the globs for files supported by the plugin. This is only relevant
  /// if the plugin is scanning files.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub globs: Vec<String>,
  /// Store the id of the plugin. This should be globally unique and if the
  /// crate is published it should be the published crate name of the plugin.
  #[builder(setter(into))]
  pub id: String,
  /// Store a readable name of the plugin. This is used for error messages and
  #[serde(default)]
  #[builder(default, setter(into, strip_option))]
  pub name: Option<String>,
  /// Store the markdown description of the plugin.
  #[serde(default)]
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
}

pub trait Plugin {
  /// Get the id of the plugin. This should be globally unique and can be the
  /// published crate_name of the plugin.
  fn get_data(&self) -> PluginData;

  #[allow(unused)]
  fn read_options(&mut self, options: &Options) -> AnyEmptyResult {
    Ok(())
  }

  /// Receive a mutable slice of the configuration. The config received is not
  /// the original configuration but created at the start just for the
  /// plugins. It will be merged into the [`StyleConfig`].
  #[allow(unused)]
  fn mutate_config(&self, config: &mut PluginConfig, options: &Options) -> AnyEmptyResult {
    Ok(())
  }

  /// Generate code from the configuration. This is called after the config
  /// has been generated.
  #[allow(unused)]
  fn generate_code(&self, config: &RunnerConfig) -> AnyResult<GeneratedFiles> {
    Ok(GeneratedFiles::default())
  }

  /// Each plugin can implement a custom scanner that feeds back classes from
  /// the provided byte data.
  #[allow(unused)]
  fn scan_code(
    &self,
    config: &RunnerConfig,
    file_path: &Path,
    bytes: Vec<u8>,
  ) -> AnyResult<Classes> {
    Ok(Classes::default())
  }
}

impl<P: Plugin + 'static> From<P> for Box<dyn Plugin> {
  fn from(plugin: P) -> Self {
    Box::new(plugin)
  }
}
