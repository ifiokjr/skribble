use std::ops::Deref;
use std::ops::DerefMut;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::config::*;
use crate::Class;
use crate::MergedConfig;

pub trait Plugin {
  /// Get the id of the plugin. This should be globally unique and can be the
  /// published crate_name of the plugin.
  fn get_id(&self) -> String;

  #[allow(unused)]
  fn read_options(&mut self, options: &Options) -> AnyEmptyResult {
    Ok(())
  }

  /// Receive a mutable slice of the configuration. The config received is not
  /// the original configuration but created at the start just for the plugins.
  /// It will be merged into the [`StyleConfig`].
  #[allow(unused)]
  fn mutate_config(&self, config: &mut WrappedPluginConfig, options: &Options) -> AnyEmptyResult {
    Ok(())
  }

  /// Generate code from the configuration. This is called after the config has
  /// been generated.
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

pub type AnyError = Box<dyn std::error::Error>;
pub type AnyEmptyResult = Result<(), AnyError>;
pub type AnyResult<T> = Result<T, AnyError>;

#[derive(Clone, Default)]
pub struct WrappedPluginConfig {
  pub layers: Layers,
  pub keyframes: Keyframes,
  pub variables: CssVariables,
  pub media_queries: MediaQueries,
  pub modifiers: Modifiers,
  pub atoms: Atoms,
  pub classes: NamedClasses,
  pub palette: Palette,
  pub value_sets: ValueSets,
  pub groups: VariableGroups,
  pub additional_fields: AdditionalFields,
}

#[derive(Clone, Debug, Deserialize, Serialize, TypedBuilder)]
pub struct GeneratedFile {
  /// A relative path where the file should be written to.
  #[builder(setter(into))]
  pub path: PathBuf,

  /// The contents of the file.
  #[builder(setter(into))]
  pub content: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GeneratedFiles(Vec<GeneratedFile>);

impl GeneratedFiles {
  pub fn merge(&mut self, other: impl Into<Self>) {
    self.0.extend(other.into());
  }
}

impl From<Vec<GeneratedFile>> for GeneratedFiles {
  fn from(files: Vec<GeneratedFile>) -> Self {
    Self(files)
  }
}

impl IntoIterator for GeneratedFiles {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = GeneratedFile;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl Deref for GeneratedFiles {
  type Target = Vec<GeneratedFile>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for GeneratedFiles {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
