use std::ops::Deref;
use std::ops::DerefMut;

use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// The priority of a an ordered item. A lower number is better. The default is
/// 150.
#[derive(Clone, Copy, Debug, Deserialize, Hash, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Priority(u8);

impl Priority {
  pub const DEFAULT: Self = Self(150);
  pub const HIGH: Self = Self(50);
  pub const LOW: Self = Self(200);
  pub const MEDIUM: Self = Self(100);
}

impl Default for Priority {
  fn default() -> Self {
    Self::DEFAULT
  }
}

impl<T: Into<u8>> From<T> for Priority {
  fn from(value: T) -> Self {
    Self(value.into())
  }
}

impl Deref for Priority {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Priority {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Prioritized<T> {
  #[builder(default, setter(into))]
  pub priority: Priority,
  #[builder(setter(into))]
  pub value: T,
}

impl<T: Into<String>> From<T> for Prioritized<String> {
  fn from(value: T) -> Self {
    Self {
      priority: Default::default(),
      value: value.into(),
    }
  }
}

impl<T> Deref for Prioritized<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl<T> DerefMut for Prioritized<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.value
  }
}

pub type PrioritizedString = Prioritized<String>;
