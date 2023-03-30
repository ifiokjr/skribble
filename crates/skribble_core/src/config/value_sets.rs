use derive_more::Deref;
use derive_more::DerefMut;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::Priority;
use crate::CssValues;

/// The value atom.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ValueSet {
  #[builder(setter(into))]
  pub name: String,
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The values for this set.
  #[builder(setter(into))]
  pub values: CssValues,
}

impl ValueSet {
  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();

    if self.name != other.name {
      panic!("Cannot merge groups with different names");
    }

    if let Some(description) = other.description {
      self.description = Some(description);
    }

    if other.priority < self.priority {
      self.priority = other.priority;
    }

    self.values.extend(other.values);
  }
}

/// A set of values that referenced by .

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Deref, DerefMut)]
pub struct ValueSets(Vec<ValueSet>);

impl<T: Into<ValueSet>> From<Vec<T>> for ValueSets {
  fn from(value_sets: Vec<T>) -> Self {
    Self::from_iter(value_sets)
  }
}

impl IntoIterator for ValueSets {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = ValueSet;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for ValueSets
where
  V: Into<ValueSet>,
{
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let atoms = iter.into_iter().map(|v| v.into()).collect();

    Self(atoms)
  }
}
