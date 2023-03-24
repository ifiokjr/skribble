use std::ops::Deref;
use std::ops::DerefMut;

use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::Priority;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, TypedBuilder)]
pub struct Group<T: Clone> {
  #[builder(setter(into))]
  pub name: String,
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The items in this group.
  #[builder(setter(into))]
  pub items: Vec<T>,
}

impl<T: Clone> Group<T> {
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

    self.items.extend(other.items);
  }
}

impl<T: Clone> IntoIterator for Group<T> {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = T;

  fn into_iter(self) -> Self::IntoIter {
    self.items.into_iter()
  }
}

impl<T: Clone> Deref for Group<T> {
  type Target = Vec<T>;

  fn deref(&self) -> &Self::Target {
    &self.items
  }
}

impl<T: Clone> DerefMut for Group<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.items
  }
}
