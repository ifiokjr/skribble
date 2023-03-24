use std::ops::Deref;
use std::ops::DerefMut;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

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
