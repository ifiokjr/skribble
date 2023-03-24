use std::path::PathBuf;

use crate::config::*;
use crate::css::Class;
use crate::plugin::AnyEmptyResult;
use crate::plugin::AnyResult;
use crate::plugin::GeneratedFiles;
use crate::plugin::WrappedPluginConfig;
use crate::runner::MergedConfig;

pub trait Plugin {
  /// Get the id of the plugin. This should be globally unique and can be the
  /// published crate_name of the plugin.
  fn get_id(&self) -> String;

  #[allow(unused)]
  fn read_options(&mut self, options: &Options) -> AnyEmptyResult {
    Ok(())
  }

  /// Receive a mutable slice of the configuration. The config received is not
  /// the original configuration but created at the start just for the
  /// plugins. It will be merged into the [`StyleConfig`].
  #[allow(unused)]
  fn mutate_config(&self, config: &mut WrappedPluginConfig, options: &Options) -> AnyEmptyResult {
    Ok(())
  }

  /// Generate code from the configuration. This is called after the config
  /// has been generated.
  #[allow(unused)]
  fn generate_code(&self, config: &MergedConfig) -> AnyResult<GeneratedFiles> {
    Ok(GeneratedFiles::default())
  }

  /// Each plugin can implement a custom scanner that feeds back classes from
  /// the provided byte data.
  #[allow(unused)]
  fn scan_code(&self, file_path: PathBuf, bytes: Vec<u8>) -> AnyResult<Vec<Class>> {
    Ok(vec![])
  }

  /// Set a readable name of the plugin. This is used for error messages and
  /// serialization.
  ///
  /// It defaults to the id of the plugin.
  fn get_name(&self) -> String {
    self.get_id()
  }

  /// Get the markdown description of the plugin. Defaults to an empty string.
  fn get_description(&self) -> String {
    "".into()
  }
}

impl<P: Plugin + 'static> From<P> for Box<dyn Plugin> {
  fn from(plugin: P) -> Self {
    Box::new(plugin)
  }
}
