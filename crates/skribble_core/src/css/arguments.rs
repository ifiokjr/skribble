use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Arguments {
  /// A value argument.
  V(String),
  /// A key value argument.
  KV(String, String),
}

impl Arguments {
  pub fn v(value: impl AsRef<str>) -> Self {
    let value = value.as_ref().to_string();
    Self::V(value)
  }

  pub fn kv(key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
    let key = key.as_ref().trim().to_string();
    let value = value.as_ref().trim().to_string();

    Self::KV(key, value)
  }

  pub fn get_value(&self) -> String {
    match self {
      Arguments::V(value) => value.to_string(),
      Arguments::KV(_, value) => value.to_string(),
    }
  }
}

impl Display for Arguments {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Arguments::V(value) => write!(f, "{}", value),
      Arguments::KV(key, value) => write!(f, "{}:{}", key, value),
    }
  }
}

impl<S: AsRef<str>> From<S> for Arguments {
  fn from(value: S) -> Self {
    match value.as_ref().split_once(':') {
      Some((key, value)) => Self::kv(key, value),
      None => Self::v(value),
    }
  }
}
