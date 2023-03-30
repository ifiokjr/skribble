use derive_more::Deref;
use derive_more::DerefMut;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;

use super::PrioritizedString;

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize, Deref, DerefMut)]
pub struct NameSet(IndexSet<PrioritizedString>);

impl NameSet {
  pub fn sort_by_priority(&mut self) -> &Self {
    self.sort_by(|a, z| a.priority.cmp(&z.priority));
    self
  }

  pub fn merge(&mut self, other: impl Into<Self>) {
    self.extend(other.into());
    self.sort_by_priority();
  }
}

impl IntoIterator for NameSet {
  type IntoIter = indexmap::set::IntoIter<Self::Item>;
  type Item = PrioritizedString;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V: Into<PrioritizedString>> FromIterator<V> for NameSet {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let list = iter.into_iter().map(|v| v.into()).collect();

    Self(list)
  }
}

impl<I: Into<PrioritizedString>> From<Vec<I>> for NameSet {
  fn from(list: Vec<I>) -> Self {
    Self::from_iter(list)
  }
}

impl<I: Into<PrioritizedString>> From<IndexSet<I>> for NameSet {
  fn from(list: IndexSet<I>) -> Self {
    Self::from_iter(list)
  }
}
