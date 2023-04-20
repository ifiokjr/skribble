use std::fmt;

use derive_more::Deref;
use derive_more::DerefMut;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::AnyEmptyResult;
use crate::Priority;
use crate::RunnerConfig;
use crate::ToSkribbleCss;

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize, Deref, DerefMut)]
pub struct CssChunks(Vec<CssChunk>);

impl CssChunks {
  pub fn merge(&mut self, other: impl Into<Self>) {
    self.extend(other.into());
  }

  pub fn sort_by_priority(&mut self) {
    self.sort_by(|a, z| a.priority.cmp(&z.priority));
  }
}

impl IntoIterator for CssChunks {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = CssChunk;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl FromIterator<CssChunk> for CssChunks {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = CssChunk>,
  {
    Self(iter.into_iter().collect())
  }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CssChunk {
  /// The name of the media query.
  #[builder(setter(into))]
  pub name: String,
  /// A markdown description of what this media query should be used for.
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The layer to place the css into. Two layers which are always available are
  /// `default` and `base`. Base is used for the `reset` css.
  #[builder(setter(into))]
  pub layer: String,
  /// The css to add.
  #[builder(setter(into))]
  pub css: String,
  /// Whether to automatically include this chunk. For chunks that are not
  /// automatically included it is up to plugins to ensure the relevant class
  /// name is included.
  #[builder(default, setter(into))]
  #[serde(default)]
  pub auto_include: bool,
}

impl CssChunk {
  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();

    if self.name != other.name {
      panic!("Cannot merge css chunks with different names");
    }

    if let Some(description) = other.description {
      self.description = Some(description);
    }

    if other.priority < self.priority {
      self.priority = other.priority;
    }

    self.auto_include = other.auto_include;
    self.layer = other.layer;
    // append rather than overwrite
    self.css = format!("{}\n{}", self.css, other.css);
  }
}

impl ToSkribbleCss for CssChunk {
  fn write_skribble_css(
    &self,
    writer: &mut dyn fmt::Write,
    _config: &RunnerConfig,
  ) -> AnyEmptyResult {
    writeln!(writer, "{}", self.css)?;
    Ok(())
  }
}
