use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use super::generate_merged_config;
use super::walk_directory;
use super::RunnerConfig;
use crate::Classes;
use crate::Error;
use crate::GeneratedFiles;
use crate::Options;
use crate::PluginConfig;
use crate::Result;
use crate::StyleConfig;
use crate::WrappedPlugin;

pub struct SkribbleRunner {
  options: Arc<Options>,
  base_config: Arc<PluginConfig>,
  plugins: Arc<Mutex<Vec<WrappedPlugin>>>,
  config: Option<RunnerConfig>,
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
      base_config: config,
      plugins,
      config: None,
    }
  }

  /// Run the plugins to mutate the config and get the transformed config which
  /// is used.
  pub fn initialize(&mut self) -> Result<&RunnerConfig> {
    self.provide_options_to_plugins()?;
    let config_from_plugins = self.generate_plugin_config()?;
    self.merge(config_from_plugins);

    self.config.as_ref().ok_or(Error::RunnerNotSetup)
  }

  /// Provide options to the plugins.
  fn provide_options_to_plugins(&mut self) -> Result<()> {
    let options = self.options.as_ref();
    let mut plugins = self.plugins.lock().unwrap();

    for plugin in plugins.iter_mut() {
      plugin.read_options(options).map_err(|source| {
        Error::PluginReadConfigError {
          id: plugin.data().id.clone(),
          source,
        }
      })?;
    }

    Ok(())
  }

  /// Run the generate functions on all plugins with the provided merged
  /// configuration.
  pub fn generate(&self) -> Result<GeneratedFiles> {
    let Some(ref config) = self.config else {
      return Err(Error::RunnerNotSetup);
    };

    let plugins = self.plugins.lock().unwrap();
    let mut generated_files = GeneratedFiles::default();

    for plugin in plugins.iter() {
      let generated = plugin.generate_code(config).map_err(|source| {
        Error::PluginGenerateCodeError {
          id: plugin.data().id.clone(),
          source,
        }
      })?;

      generated_files.merge(generated);
    }

    Ok(generated_files)
  }

  pub fn scan(&self, cwd: impl AsRef<Path>) -> Result<Classes> {
    let Some(_) = self.config else {
      return Err(Error::RunnerNotSetup);
    };

    let entries = walk_directory(cwd, &self.options.files).map_err(Error::FileScanError)?;

    let plugins = self.plugins.lock().unwrap();
    let mut classes = Classes::default();

    for entry in entries.iter() {
      let path = entry.path();
      let bytes = std::fs::read(entry.path())
        .map_err(move |source| Error::FileReadError(path.to_path_buf(), source))?;
      for plugin in plugins.iter() {
        let scanned = plugin.scan_code(path, bytes.clone()).map_err(|source| {
          Error::PluginScanCodeError {
            id: plugin.data().id.clone(),
            source,
          }
        })?;

        classes.merge(scanned);
      }
    }

    Ok(classes)
  }

  fn generate_plugin_config(&self) -> Result<PluginConfig> {
    let mut plugin_config = PluginConfig::default();
    let plugins = self.plugins.lock().unwrap();

    for plugin in plugins.iter() {
      plugin
        .mutate_config(&mut plugin_config, &self.options)
        .map_err(|source| {
          Error::PluginMutateConfigError {
            id: plugin.data().id.clone(),
            source,
          }
        })?;
    }

    Ok(plugin_config)
  }

  fn merge(&mut self, plugin_config: PluginConfig) {
    let config = generate_merged_config(plugin_config, self.options.clone(), &self.base_config);

    self.config = Some(config);
  }
}
