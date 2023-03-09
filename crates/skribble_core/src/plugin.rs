use crate::config::*;

pub trait Plugin {
  /// Get the id of the plugin. This should be globally unique and can be the
  /// published crate_name of the plugin.
  fn get_id(&self) -> String;

  /// Here the plugin can inspect a reference to the received configuration.
  #[allow(unused)]
  fn load_config(&mut self, config: &StyleConfig) -> AnyResult {
    Ok(())
  }

  /// Receive a mutable slice of the configuration. The config received is not
  /// the original configuration but created at the start just for the plugins.
  /// It will be merged into the [`StyleConfig`].
  #[allow(unused)]
  fn mutate_config(&mut self, config: &mut ConfigEnum) -> AnyResult {
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

#[non_exhaustive]
pub enum ConfigEnum {
  Keyframes(Keyframes),
  CssVariables(CssVariables),
  MediaQueries(MediaQueries),
  ParentModifiers(ParentModifiers),
  Modifiers(Modifiers),
  NamedRules(NamedRules),
  NamedClasses(NamedClasses),
  Palette(Palette),
  Atoms(Atoms),
  Groups(Groups),
  AdditionalFields(AdditionalFields),
}
