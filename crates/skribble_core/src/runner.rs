use std::sync::Arc;
use std::sync::Mutex;

use indexmap::IndexMap;

use crate::CssVariable;
use crate::Error;
use crate::Keyframe;
use crate::MediaQuery;
use crate::Modifier;
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

    let wrapped_config = self.generate_wrapped_config()?;
    self.merge(&wrapped_config);

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
    let mut media_queries = IndexMap::<String, IndexMap<String, MediaQuery>>::new();
    let mut parent_modifiers = IndexMap::<String, Modifier>::new();
    let mut modifiers = IndexMap::<String, IndexMap<String, Modifier>>::new();

    // keyframes
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

    // css_variables
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

    // media_queries
    let mut wrapped_media_queries = wrapped_config.media_queries.clone();
    wrapped_media_queries.sort_by(|a, z| z.priority.cmp(&a.priority));

    for media_query_group in wrapped_media_queries.iter() {
      let group_name = media_query_group.name.clone();
      let mut group = IndexMap::<String, MediaQuery>::new();

      for media_query in media_query_group.iter() {
        let key = media_query.name.clone();
        match group.get_mut(&key) {
          Some(existing) => {
            existing.merge(media_query);
          }
          None => {
            group.insert(key, media_query.clone());
          }
        }
      }

      group.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));

      match media_queries.get_mut(&group_name) {
        Some(existing) => {
          existing.extend(group);
        }
        None => {
          media_queries.insert(group_name, group);
        }
      }
    }

    // parent_modifiers
    for parent_modifier in wrapped_config.parent_modifiers.iter() {
      let key = parent_modifier.name.clone();

      match parent_modifiers.get_mut(&key) {
        Some(existing) => {
          existing.merge(parent_modifier);
        }
        None => {
          parent_modifiers.insert(key, parent_modifier.clone());
        }
      }
    }

    // modifiers
    let mut wrapped_modifiers = wrapped_config.modifiers.clone();
    wrapped_modifiers.sort_by(|a, z| z.priority.cmp(&a.priority));

    for modifier_group in wrapped_modifiers.iter() {
      let group_name = modifier_group.name.clone();
      let mut group = IndexMap::<String, Modifier>::new();

      for modifier in modifier_group.iter() {
        let key = modifier.name.clone();
        match group.get_mut(&key) {
          Some(existing) => {
            existing.merge(modifier);
          }
          None => {
            group.insert(key, modifier.clone());
          }
        }
      }

      group.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));

      match modifiers.get_mut(&group_name) {
        Some(existing) => {
          existing.extend(group);
        }
        None => {
          modifiers.insert(group_name, group);
        }
      }
    }

    // sort by priority
    keyframes.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    css_variables.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    parent_modifiers.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));

    MergedConfig {
      keyframes,
      css_variables,
      media_queries,
      parent_modifiers,
      modifiers,
    }
  }
}

/// The configuration after all plugins have been run.
#[derive(Clone, Default)]
pub struct MergedConfig {
  pub keyframes: IndexMap<String, Keyframe>,
  pub css_variables: IndexMap<String, CssVariable>,
  pub media_queries: IndexMap<String, IndexMap<String, MediaQuery>>,
  pub parent_modifiers: IndexMap<String, Modifier>,
  pub modifiers: IndexMap<String, IndexMap<String, Modifier>>,
}
