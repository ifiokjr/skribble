use derivative::Derivative;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::Atoms;
use super::CssChunks;
use super::CssVariables;
use super::Keyframes;
use super::MediaQueries;
use super::Modifiers;
use super::NameSet;
use super::NamedClasses;
use super::Options;
use super::Plugins;
use super::PrioritizedString;
use super::Priority;
use super::StringMap;
use super::ValueSets;
use crate::Error;
use crate::Plugin;
use crate::PluginConfig;
use crate::PluginContainer;
use crate::Result;

/// The style configuration which can also use the builder pattern.
#[derive(Derivative, Deserialize, Serialize, TypedBuilder)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct StyleConfig {
  /// The general options.
  #[builder(default, setter(into))]
  pub options: Options,
  /// The css layers.
  #[serde(default = "default_layers")]
  #[builder(default =  default_layers(), setter(into))]
  pub layers: Layers,
  /// Raw css which will be added to the end of the generated css.
  #[builder(default, setter(into))]
  pub css_chunks: CssChunks,
  /// Setup the keyframes.
  #[builder(default, setter(into))]
  pub keyframes: Keyframes,
  /// CSS variables which can be reused throughout the configuration.
  #[builder(default, setter(into))]
  pub variables: CssVariables,
  /// Setup the media queries.
  #[builder(default, setter(into))]
  pub media_queries: MediaQueries,
  /// Modifiers are used to nest styles within a selector. They can be parents
  /// modifiers or child modifiers.
  #[builder(default, setter(into))]
  pub modifiers: Modifiers,
  /// Set up the style rules which determine the styles that each atom name will
  /// correspond to.
  #[builder(default, setter(into))]
  pub atoms: Atoms,
  /// A list of classes with predefined styles.
  #[builder(default, setter(into))]
  pub classes: NamedClasses,
  /// Hardcoded colors for the pallette.
  #[builder(default, setter(into))]
  pub palette: Palette,
  /// The atoms which provide the values.
  #[builder(default, setter(into))]
  pub value_sets: ValueSets,
  /// The plugins which can be used to add new functionality and extend the
  /// configuration.
  #[derivative(Debug = "ignore")]
  #[serde(skip)]
  #[builder(default, setter(into))]
  pub plugins: Plugins,
}

impl Default for StyleConfig {
  fn default() -> Self {
    Self::builder().build()
  }
}

impl StyleConfig {
  pub fn from_json(json: impl AsRef<str>) -> Result<Self> {
    let config: Self = serde_json::from_str(json.as_ref()).map_err(Error::InvalidConfig)?;
    Ok(config)
  }

  pub(crate) fn into_wrapped_config(self) -> (Options, PluginConfig, Plugins) {
    let Self {
      atoms,
      classes,
      css_chunks,
      keyframes,
      layers,
      media_queries,
      modifiers,
      options,
      palette,
      plugins,
      value_sets,
      variables,
    } = self;

    (
      options,
      PluginConfig {
        atoms,
        classes,
        css_chunks,
        keyframes,
        layers,
        media_queries,
        modifiers,
        palette,
        value_sets,
        variables,
      },
      plugins,
    )
  }

  pub fn to_json(&self) -> Result<String> {
    serde_json::to_string(self).map_err(Error::CouldNotSerializeConfig)
  }

  pub fn to_pretty_json(&self) -> Result<String> {
    serde_json::to_string_pretty(self).map_err(Error::CouldNotSerializeConfig)
  }

  pub fn add_plugin<P: Plugin + 'static, T: Into<Priority>>(
    &mut self,
    plugin: P,
    priority: T,
  ) -> &mut Self {
    self
      .plugins
      .push(PluginContainer::new(Box::new(plugin), priority.into()));

    self
  }

  pub fn remove_plugin(&mut self, id: impl AsRef<str>) -> &mut Self {
    let id = id.as_ref();
    self.plugins.retain(|container| container.get_id() != id);
    self
  }
}

/// Create a palette for the configuration.
pub type Palette = StringMap;
/// The additional css layers.
pub type Layers = NameSet;

pub fn default_layers() -> Layers {
  let mut layers = Layers::default();
  let base = PrioritizedString {
    value: "base".into(),
    priority: Priority::LOW,
  };
  let default = PrioritizedString {
    value: "default".into(),
    priority: Priority::DEFAULT,
  };

  layers.insert(base);
  layers.insert(default);

  layers
}
