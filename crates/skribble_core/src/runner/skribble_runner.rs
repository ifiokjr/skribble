use std::sync::Arc;
use std::sync::Mutex;

use super::generate_merged_config;
use super::RunnerConfig;
use crate::BoxedPlugin;
use crate::Error;
use crate::GeneratedFiles;
use crate::Options;
use crate::PluginConfig;
use crate::Result;
use crate::StyleConfig;

pub struct SkribbleRunner {
  options: Arc<Options>,
  config: Arc<PluginConfig>,
  plugins: Arc<Mutex<Vec<BoxedPlugin>>>,
  merged_config: Option<RunnerConfig>,
}

impl SkribbleRunner {
  pub fn new(config: StyleConfig) -> Self {
    let (options, wrapped_config, mut plugins) = config.into_wrapped_config();
    let options = Arc::new(options);
    let config = Arc::new(wrapped_config);

    // Extract the plugins from the config and sort them by priority.
    plugins.sort_by_priority();
    let plugins = Arc::new(Mutex::new(plugins.extract_plugins()));

    Self {
      options,
      config,
      plugins,
      merged_config: None,
    }
  }

  /// Run the plugins to mutate the config and get the transformed config which
  /// is used.
  pub fn run(&mut self) -> Result<()> {
    self.provide_options_to_plugins()?;
    let config_from_plugins = self.generate_plugin_config()?;
    self.merge(config_from_plugins);

    // TODO ignoring options around how the config should be extended for now.

    Ok(())
  }

  /// Provide options to the plugins.
  fn provide_options_to_plugins(&mut self) -> Result<()> {
    let options = self.options.as_ref();
    let mut plugins = self.plugins.lock().unwrap();

    for boxed_plugin in plugins.iter_mut() {
      let plugin = boxed_plugin.as_mut();
      plugin.read_options(options).map_err(|e| {
        Error::PluginReadConfigError {
          id: plugin.get_id(),
          source: e,
        }
      })?;
    }

    Ok(())
  }

  /// Run the generate functions on all plugins with the provided merged
  /// configuration.
  pub fn generate(&self) -> Result<GeneratedFiles> {
    let Some(ref config) = self.merged_config else {
      return Err(Error::RunnerNotSetup);
    };

    let plugins = self.plugins.lock().unwrap();
    let mut generated_files = GeneratedFiles::default();

    for boxed_plugin in plugins.iter() {
      let plugin = boxed_plugin.as_ref();
      let generated = plugin.generate_code(config).map_err(|e| {
        Error::PluginGenerateCodeError {
          id: plugin.get_id(),
          source: e,
        }
      })?;

      generated_files.merge(generated);
    }

    Ok(generated_files)
  }

  fn generate_plugin_config(&self) -> Result<PluginConfig> {
    let mut plugin_config = PluginConfig::default();
    let plugins = self.plugins.lock().unwrap();

    for boxed_plugin in plugins.iter() {
      let plugin = boxed_plugin.as_ref();
      plugin
        .mutate_config(&mut plugin_config, &self.options)
        .map_err(|e| {
          Error::PluginMutateConfigError {
            id: plugin.get_id(),
            source: e,
          }
        })?;
    }

    Ok(plugin_config)
  }

  fn merge(&mut self, plugin_config: PluginConfig) {
    self.merged_config = Some(generate_merged_config(
      plugin_config,
      self.options.clone(),
      &self.config,
    ));
  }
}
