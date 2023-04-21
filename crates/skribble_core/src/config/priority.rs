use derive_more::Deref;
use derive_more::DerefMut;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// The priority of a an ordered item. A lower number is will be given a higher
/// priority. The default is 150.
#[derive(
  Clone, Copy, Debug, Deserialize, Hash, Eq, Ord, PartialEq, PartialOrd, Serialize, Deref, DerefMut,
)]
pub struct Priority(u8);

impl Priority {
  pub const DEFAULT: Self = Self(150);
  pub const HIGH: Self = Self(50);
  pub const LOW: Self = Self(200);
  pub const LOWER: Self = Self(250);
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

#[derive(
  Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, TypedBuilder, Deref, DerefMut,
)]
#[serde(rename_all = "camelCase")]
pub struct Prioritized<T> {
  #[builder(default, setter(into))]
  pub priority: Priority,
  #[builder(setter(into))]
  #[deref]
  #[deref_mut]
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

pub type PrioritizedString = Prioritized<String>;
