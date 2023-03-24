use std::ops::Deref;
use std::ops::DerefMut;

use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;

/// This is a more usable version of Index<String, String> which allows for
/// easier construction and fully supports serde with renaming built in.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StringMap(IndexMap<String, String>);

impl IntoIterator for StringMap {
  type IntoIter = indexmap::map::IntoIter<String, String>;
  type Item = (String, String);

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<K, V> FromIterator<(K, V)> for StringMap
where
  K: Into<String>,
  V: Into<String>,
{
  fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
    let map = iter
      .into_iter()
      .map(|(key, value)| (key.into(), value.into()))
      .collect();

    Self(map)
  }
}

impl<K: Into<String>, V: Into<String>> From<IndexMap<K, V>> for StringMap {
  fn from(value: IndexMap<K, V>) -> Self {
    Self::from_iter(value)
  }
}

impl Deref for StringMap {
  type Target = IndexMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for StringMap {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NestedStringMap(IndexMap<String, StringMap>);

impl IntoIterator for NestedStringMap {
  type IntoIter = indexmap::map::IntoIter<String, StringMap>;
  type Item = (String, StringMap);

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<K: Into<String>, V: Into<StringMap>> FromIterator<(K, V)> for NestedStringMap {
  fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
    let map = iter
      .into_iter()
      .map(|(k, v)| (k.into(), v.into()))
      .collect();

    Self(map)
  }
}

impl<K: Into<String>, V: Into<StringMap>> From<IndexMap<K, V>> for NestedStringMap {
  fn from(value: IndexMap<K, V>) -> Self {
    Self::from_iter(value)
  }
}

impl Deref for NestedStringMap {
  type Target = IndexMap<String, StringMap>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for NestedStringMap {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// This is a more usable version of Vec<String> which allows for
/// easier construction and fully supports serde with renaming built in.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StringList(Vec<String>);

impl<V: Into<String>> From<Vec<V>> for StringList {
  fn from(value: Vec<V>) -> Self {
    Self::from_iter(value)
  }
}

impl IntoIterator for StringList {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = String;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for StringList
where
  V: Into<String>,
{
  fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
    let rules = iter.into_iter().map(|value| value.into()).collect();
    Self(rules)
  }
}

impl Deref for StringList {
  type Target = Vec<String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for StringList {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionalStringMap(IndexMap<String, Option<String>>);

impl IntoIterator for OptionalStringMap {
  type IntoIter = indexmap::map::IntoIter<String, Option<String>>;
  type Item = (String, Option<String>);

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<K, V> FromIterator<(K, Option<V>)> for OptionalStringMap
where
  K: Into<String>,
  V: Into<String>,
{
  fn from_iter<T: IntoIterator<Item = (K, Option<V>)>>(iter: T) -> Self {
    let rules = iter
      .into_iter()
      .map(|(key, value)| (key.into(), value.map(|v| v.into())))
      .collect();

    Self(rules)
  }
}

impl<K, V> From<IndexMap<K, Option<V>>> for OptionalStringMap
where
  K: Into<String>,
  V: Into<String>,
{
  fn from(value: IndexMap<K, Option<V>>) -> Self {
    Self::from_iter(value)
  }
}

impl Deref for OptionalStringMap {
  type Target = IndexMap<String, Option<String>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for OptionalStringMap {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
