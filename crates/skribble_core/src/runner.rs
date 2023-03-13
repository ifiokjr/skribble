use std::sync::Arc;
use std::sync::Mutex;

use indexmap::IndexMap;


use crate::CssVariable;
use crate::Error;
use crate::Keyframe;
use crate::Plugin;
use crate::Result;
use crate::StyleConfig;
use crate::WrappedPluginConfig;

pub struct SkribbleRunner {
  config: StyleConfig,
  plugins: Vec<Arc<Mutex<Box<dyn Plugin>>>>,
}

impl SkribbleRunner {
  pub fn new(config: StyleConfig) -> Self {
    Self {
      config,
      plugins: vec![],
    }
  }

  /// Run the plugins to mutate the config and get the transformed config which
  /// is used.
  pub fn run(&mut self) -> Result<()> {
    self.extract_plugins();
    self.provide_options_to_plugins()?;

    let mut wrapped_config = self.generate_wrapped_config()?;
    self.merge(&mut wrapped_config);

    // TODO ignoring options around how the config should be extended for now.

    Ok(())
  }

  /// Extract the plugins from the config and sort them by priority.
  fn extract_plugins(&mut self) {
    let mut containers = vec![];

    for container in self.config.plugins.iter() {
      containers.push(container);
    }

    containers.sort_by(|a, z| a.priority.cmp(&z.priority));
    self.plugins = containers.iter().map(|c| c.plugin.clone()).collect();
  }

  /// Provide options to the plugins.
  fn provide_options_to_plugins(&mut self) -> Result<()> {
    let options = &self.config.options;

    for container in self.plugins.iter_mut() {
      let mut plugin = container.lock().unwrap();
      let plugin = plugin.as_mut();
      plugin.read_options(options).map_err(|e| {
        Error::PluginReadConfigError {
          id: plugin.get_id(),
          source: e,
        }
      })?;
    }

    Ok(())
  }

  fn generate_wrapped_config(&self) -> Result<WrappedPluginConfig> {
    let mut wrapped_config = WrappedPluginConfig::default();

    for container in self.plugins.iter() {
      let plugin = container.lock().unwrap();
      let plugin = plugin.as_ref();
      plugin.mutate_config(&mut wrapped_config).map_err(|e| {
        Error::PluginMutateConfigError {
          id: plugin.get_id(),
          source: e,
        }
      })?;
    }

    Ok(wrapped_config)
  }

  fn merge(&self, wrapped_config: &WrappedPluginConfig) -> MergedConfig {
    // let merge_rules = &self.config.options.merge_rules;
    let mut keyframes = IndexMap::<String, Keyframe>::new();
    let mut css_variables = IndexMap::<String, CssVariable>::new();
    // let mut keyframes = self.config.keyframes.clone();
    // keyframes.extend(wrapped_config.keyframes);

    for keyframe in wrapped_config.keyframes.iter() {
      let key = keyframe.name.clone();

      match keyframes.get_mut(&key) {
        Some(existing) => {
          existing.merge(keyframe);
        }
        None => {
          keyframes.insert(key, keyframe.clone());
        }
      }
    }

    for css_variable in wrapped_config.css_variables.iter() {
      let key = css_variable.name.clone();

      match css_variables.get_mut(&key) {
        Some(existing) => {
          existing.merge(css_variable);
        }
        None => {
          css_variables.insert(key, css_variable.clone());
        }
      }
    }

    keyframes.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    css_variables.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));

    MergedConfig {
      keyframes,
      css_variables,
    }
  }
}

/// The configuration after all plugins have been run.
#[derive(Clone, Default)]
pub struct MergedConfig {
  pub keyframes: IndexMap<String, Keyframe>,
  pub css_variables: IndexMap<String, CssVariable>,
}
