use crate::config::*;

pub trait Plugin {
  /// Get the id of the plugin. This should be globally unique and can be the
  /// published crate_name of the plugin.
  fn get_id(&self) -> String;

  #[allow(unused)]
  fn read_options(&mut self, options: &Options) -> AnyResult {
    Ok(())
  }

  /// Receive a mutable slice of the configuration. The config received is not
  /// the original configuration but created at the start just for the plugins.
  /// It will be merged into the [`StyleConfig`].
  #[allow(unused)]
  fn mutate_config(&self, config: &mut WrappedPluginConfig) -> AnyResult {
    Ok(())
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

pub type AnyResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Clone, Default)]
pub struct WrappedPluginConfig {
  pub keyframes: Keyframes,
  pub css_variables: CssVariables,
  pub media_queries: MediaQueries,
  pub parent_modifiers: ParentModifiers,
  pub modifiers: Modifiers,
  pub atoms: Atoms,
  pub named_classes: NamedClasses,
  pub palette: Palette,
  pub value_sets: ValueSets,
  pub groups: VariableGroups,
  pub additional_fields: AdditionalFields,
}
