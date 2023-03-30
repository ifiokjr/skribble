use derive_more::Deref;
use derive_more::DerefMut;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::Priority;
use crate::Plugin;
use crate::PluginData;

pub(crate) type BoxedPlugin = Box<dyn Plugin>;
#[derive(Deref, DerefMut)]
pub(crate) struct WrappedPlugin {
  #[deref(forward)]
  #[deref_mut(forward)]
  plugin: BoxedPlugin,
  data: PluginData,
}

impl WrappedPlugin {
  pub fn data(&self) -> &PluginData {
    &self.data
  }
}

/// A map of string values.
#[derive(Default, Deref, DerefMut)]
pub struct Plugins(Vec<PluginContainer>);

impl Plugins {
  /// Sort the plugins by priority and deduplicate them.
  pub(crate) fn sort_by_priority(&mut self) {
    self.0.sort_by(|a, z| a.priority.cmp(&z.priority));
  }

  /// Remove the the container plugins.
  pub(crate) fn extract_plugins(self) -> Vec<WrappedPlugin> {
    let mut plugins = vec![];

    for container in self.into_iter() {
      plugins.push(container.extract_plugin());
    }

    plugins
  }
}

impl IntoIterator for Plugins {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = PluginContainer;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V: Into<PluginContainer>> FromIterator<V> for Plugins {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let plugins = iter.into_iter().map(|v| v.into()).collect();

    Self(plugins)
  }
}

impl<T: Into<PluginContainer>> From<Vec<T>> for Plugins {
  fn from(plugins: Vec<T>) -> Self {
    Self::from_iter(plugins)
  }
}

#[derive(Serialize, TypedBuilder)]
pub struct PluginContainer {
  /// Get the default priority of this plugin which will be used to determine
  /// the order in which plugins are loaded. This can be overridden by the
  /// user.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The plugin.
  #[serde(skip)]
  #[builder(setter(transform = |p: impl Plugin + 'static| Box::new(p) as Box<dyn Plugin>))]
  plugin: BoxedPlugin,
}

impl PluginContainer {
  /// Get the plugin.
  pub(crate) fn extract_plugin(self) -> WrappedPlugin {
    WrappedPlugin {
      data: self.plugin.get_data(),
      plugin: self.plugin,
    }
  }
}

impl<P: Plugin + 'static> From<P> for PluginContainer {
  fn from(plugin: P) -> Self {
    Self {
      priority: Default::default(),
      plugin: Box::new(plugin),
    }
  }
}
