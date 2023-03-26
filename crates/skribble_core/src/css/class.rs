use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::Arguments;
use crate::ClassSize;
use crate::RunnerConfig;

pub trait SkribbleClass: Clone + Hash + Eq + Ord {
  fn data(&self) -> Class;
}

/// These represent an atomic class and should be
#[derive(Clone, Debug, Default, Deserialize, Serialize, TypedBuilder, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Class {
  /// The selector for this class.
  #[builder(setter(into))]
  selector: String,
  /// The layer for this class.
  #[builder(setter(into))]
  layer: Option<String>,
  /// The names of the media queries.
  #[builder(setter(into))]
  media_queries: IndexSet<String>,
  /// The ordered list of modifiers.
  #[builder(setter(into))]
  modifiers: IndexSet<String>,
  /// The name of the style provided. This must be provided for the `class_name`
  /// to be valid.
  #[builder(setter(into))]
  atom: Option<String>,
  /// The pre-configured value of the atom.
  #[builder(setter(into))]
  value_name: Option<String>,
  /// The name of the shorthand class.
  #[builder(setter(into))]
  named_class: Option<String>,
  /// This is the callable argument when the provided value is a callable
  /// expression.
  #[builder(setter(into))]
  argument: Option<Arguments>,
  /// Used to compare to classes
  #[builder(setter(into))]
  score: ClassSize,
  /// The keyframes used in this class.
  #[builder(setter(into))]
  keyframe: bool,
}

impl Class {
  pub fn get_keyframe(&self) -> Option<&String> {
    if self.keyframe {
      self.value_name.as_ref()
    } else {
      None
    }
  }

  pub fn get_selector(&self) -> &str {
    &self.selector
  }

  pub fn get_layer(&self) -> Option<&String> {
    self.layer.as_ref()
  }

  pub fn get_media_queries(&self) -> &IndexSet<String> {
    &self.media_queries
  }

  pub fn get_modifiers(&self) -> &IndexSet<String> {
    &self.modifiers
  }

  pub fn get_atom(&self) -> Option<&String> {
    self.atom.as_ref()
  }

  pub fn get_value_name(&self) -> Option<&String> {
    self.value_name.as_ref()
  }

  pub fn get_named_class(&self) -> Option<&String> {
    self.named_class.as_ref()
  }

  pub fn get_argument(&self) -> Option<&Arguments> {
    self.argument.as_ref()
  }

  pub fn get_style_declaration(&self, config: &RunnerConfig) -> Vec<String> {
    let mut style_declarations = vec![];

    if let Some(atom) = self.get_atom().and_then(|atom| config.atoms.get(atom)) {
      if let Some(value_set_name) = self.get_value_name() {
        style_declarations.extend(atom.get_style_properties(config, value_set_name));
      }
    }

    style_declarations
  }
}

impl Hash for Class {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.layer.hash(state);
    self.selector.hash(state);
  }
}

impl PartialOrd for Class {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Class {
  fn cmp(&self, other: &Self) -> Ordering {
    self.score.cmp(&other.score)
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Classes(IndexSet<Class>);

impl Classes {
  pub fn merge(&mut self, other: impl Into<Self>) {
    self.0.extend(other.into().0);
    self.sort_by_class();
  }

  pub fn sort_by_class(&mut self) {
    self.0.sort_by(|a, b| a.cmp(b));
  }
}

impl IntoIterator for Classes {
  type IntoIter = <IndexSet<Class> as IntoIterator>::IntoIter;
  type Item = Class;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl FromIterator<Class> for Classes {
  fn from_iter<T: IntoIterator<Item = Class>>(iter: T) -> Self {
    Self(iter.into_iter().collect())
  }
}

impl Deref for Classes {
  type Target = IndexSet<Class>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Classes {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
