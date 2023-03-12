use std::sync::Arc;
use std::sync::Mutex;

use crate::Error;
use crate::StyleConfig;
use crate::WrappedPluginConfig;

pub struct SkribbleRunner {
  config: Arc<Mutex<StyleConfig>>,
}

impl SkribbleRunner {
  pub fn new(config: StyleConfig) -> Self {
    Self {
      config: Arc::new(Mutex::new(config)),
    }
  }

  /// Run the plugins to mutate the config and get the transformed config which
  /// is used.
  pub fn prepare(&self) -> Result<(), Error> {
    let mut wrapped_config = WrappedPluginConfig::default();

    {
      let mut config = self.config.lock().unwrap();
      let options = config.options.clone();
      config.plugins.sort_by(|a, z| a.priority.cmp(&z.priority));

      for container in config.plugins.iter_mut() {
        let mut plugin = container.plugin.lock().unwrap();
        let plugin = plugin.as_mut();
        let _ = plugin.read_options(&options).map_err(|e| {
          Error::PluginReadConfigError {
            id: plugin.get_id(),
            source: e,
          }
        });
      }
    }

    {
      let mut config = self.config.lock().unwrap();
      // let options = &config.options;

      // Feed the options to the configuration.
      for container in config.plugins.iter_mut() {
        let plugin = container.plugin.lock().unwrap();
        let plugin = plugin.as_ref();
        let _ = plugin.mutate_config(&mut wrapped_config).map_err(|e| {
          Error::PluginMutateConfigError {
            id: plugin.get_id(),
            source: e,
          }
        });
      }
    }

    let mut config = self.config.lock().unwrap();

    // TODO ignoring options around how the config should be extended for now.

    Ok(())
  }
}
