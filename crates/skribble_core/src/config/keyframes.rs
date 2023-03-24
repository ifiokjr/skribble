use std::ops::Deref;
use std::ops::DerefMut;

use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::NestedStringMap;
use super::Priority;

/// This setups up the animation keyframes for the configuration. The names can
/// be reference in the atoms.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Keyframes(Vec<Keyframe>);

impl IntoIterator for Keyframes {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = Keyframe;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for Keyframes
where
  V: Into<Keyframe>,
{
  fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
    Self(iter.into_iter().map(|value| value.into()).collect())
  }
}

impl Deref for Keyframes {
  type Target = Vec<Keyframe>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Keyframes {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Keyframe {
  /// The name of the keyframe.
  #[builder(setter(into))]
  pub name: String,
  /// The description of the keyframe. This will be used in the vscode
  /// extension.
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The rules for the specific keyframe.
  #[serde(flatten, default)]
  #[builder(default, setter(into))]
  pub rules: NestedStringMap,
}

impl Keyframe {
  pub fn merge(&mut self, other: impl Into<Keyframe>) {
    let other = other.into();

    if self.name != other.name {
      panic!("Cannot merge keyframes with different names");
    }

    if let Some(description) = other.description {
      self.description = Some(description);
    }

    if self.priority > other.priority {
      self.priority = other.priority;
    }

    self.rules.extend(other.rules);
  }
}
