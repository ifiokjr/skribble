use std::hash::Hash;
use std::hash::Hasher;
use std::path::PathBuf;

use derive_more::Deref;
use derive_more::DerefMut;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, TypedBuilder)]
pub struct GeneratedFile {
  /// A relative path where the file should be written to.
  #[builder(setter(into))]
  pub path: PathBuf,

  /// The contents of the file.
  #[builder(setter(into))]
  pub content: String,
}

impl GeneratedFile {
  pub fn with_content(&self, content: impl Into<String>) -> Self {
    Self {
      path: self.path.clone(),
      content: content.into(),
    }
  }
}

impl Hash for GeneratedFile {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.path.hash(state);
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Deref, DerefMut)]
pub struct GeneratedFiles(IndexSet<GeneratedFile>);

impl GeneratedFiles {
  pub fn merge(&mut self, other: impl Into<Self>) {
    self.0.extend(other.into());
  }
}

impl From<IndexSet<GeneratedFile>> for GeneratedFiles {
  fn from(files: IndexSet<GeneratedFile>) -> Self {
    Self(files)
  }
}

impl IntoIterator for GeneratedFiles {
  type IntoIter = <IndexSet<GeneratedFile> as IntoIterator>::IntoIter;
  type Item = GeneratedFile;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}
