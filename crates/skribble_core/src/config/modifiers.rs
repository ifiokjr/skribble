use derive_more::Deref;
use derive_more::DerefMut;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::Group;
use super::Priority;
use super::StringList;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Modifier {
  /// The name of the parent modifier will be used to reference
  #[builder(setter(into))]
  pub name: String,
  /// The values.
  #[builder(setter(into))]
  pub values: StringList,
  /// The description for this item
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority for this item.
  #[builder(default, setter(into))]
  pub priority: Priority,
}

impl Modifier {
  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();

    if self.name != other.name {
      panic!("Cannot merge modifiers with different names");
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

/// This is the setup for named modifiers.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Deref, DerefMut)]
pub struct Modifiers(Vec<Group<Modifier>>);

impl Modifiers {
  /// Extend an existing group or create a new one if it does not exist.
  pub fn extend_group(&mut self, group: impl Into<Group<Modifier>>) {
    let group = group.into();

    if let Some(existing_group) = self.0.iter_mut().find(|g| g.name == group.name) {
      existing_group.merge(group);
    } else {
      self.0.push(group);
    }
  }
}

impl From<Vec<Group<Modifier>>> for Modifiers {
  fn from(modifiers: Vec<Group<Modifier>>) -> Self {
    Self(modifiers)
  }
}

impl IntoIterator for Modifiers {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = Group<Modifier>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for Modifiers
where
  V: Into<Group<Modifier>>,
{
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let modifiers = iter.into_iter().map(|value| value.into()).collect();

    Self(modifiers)
  }
}
