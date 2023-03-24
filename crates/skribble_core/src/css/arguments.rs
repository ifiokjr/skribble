use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

use crate::escape_css_string;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Arguments {
  /// A value argument.
  V(String),
  /// A key value argument.
  KV(String, String),
}

impl Arguments {
  pub fn v(value: impl AsRef<str>) -> Self {
    let value = value.as_ref();
    Self::V(escape_css_string(value.trim()))
  }

  pub fn kv(key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
    let key = key.as_ref().trim();
    let value = value.as_ref().trim();

    Self::KV(escape_css_string(key), escape_css_string(value))
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
