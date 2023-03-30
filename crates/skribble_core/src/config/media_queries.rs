use derive_more::Deref;
use derive_more::DerefMut;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::Group;
use super::Priority;

/// Media queries can should be defined as a map of names to their css queries.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct MediaQueries(Vec<Group<MediaQuery>>);

impl MediaQueries {
  /// Extend an existing group or create a new one if it does not exist.
  pub fn extend_group(&mut self, group: impl Into<Group<MediaQuery>>) {
    let group = group.into();

    if let Some(existing_group) = self.0.iter_mut().find(|g| g.name == group.name) {
      existing_group.merge(group);
    } else {
      self.0.push(group);
    }
  }
}

impl From<Vec<Group<MediaQuery>>> for MediaQueries {
  fn from(breakpoints: Vec<Group<MediaQuery>>) -> Self {
    Self(breakpoints)
  }
}

impl IntoIterator for MediaQueries {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = Group<MediaQuery>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for MediaQueries
where
  V: Into<Group<MediaQuery>>,
{
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let breakpoints = iter.into_iter().map(|value| value.into()).collect();
    Self(breakpoints)
  }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct MediaQuery {
  /// The name of the media query.
  #[builder(setter(into))]
  pub name: String,
  /// The query to use for the media query.
  #[builder(setter(into))]
  pub query: String,
  /// A markdown description of what this media query should be used for.
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
}

impl MediaQuery {
  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();

    if self.name != other.name {
      panic!("Cannot merge media queries with different names");
    }

    if let Some(description) = other.description {
      self.description = Some(description);
    }

    if other.priority < self.priority {
      self.priority = other.priority;
    }

    self.query = other.query;
  }
}
