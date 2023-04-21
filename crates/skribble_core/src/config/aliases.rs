use derive_more::Deref;
use derive_more::DerefMut;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::Priority;

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize, Deref, DerefMut)]
pub struct Aliases(Vec<Alias>);

impl Aliases {
  pub fn merge(&mut self, other: impl Into<Self>) {
    self.extend(other.into());
  }

  pub fn sort_by_priority(&mut self) {
    self.sort_by(|a, z| a.priority.cmp(&z.priority));
  }
}

impl IntoIterator for Aliases {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = Alias;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl FromIterator<Alias> for Aliases {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = Alias>,
  {
    Self(iter.into_iter().collect())
  }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Alias {
  /// The name of the alias.
  #[builder(setter(into))]
  pub name: String,
  /// A markdown description of what this alias should be used for.
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The classes to be combined. Use spaces to separate each class name.
  #[builder(setter(into))]
  pub classes: String,
  /// When combine is true, it will create a new class that combines all the
  /// styles of the classes specified, in the order they are specified in.
  ///
  /// It defaults to false meaning that the code generation will replace any
  /// reference to this class with a space separated list of the classes
  /// specified.
  #[builder(default, setter(into))]
  #[serde(default)]
  pub combine: bool,
}

impl Alias {
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

    self.combine = other.combine;
    self.classes = other.classes;
  }
}
