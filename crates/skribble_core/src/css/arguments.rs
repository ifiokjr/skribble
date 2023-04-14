use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

use serde::Deserialize;
use serde::Serialize;

use crate::AnyEmptyResult;
use crate::Atom;
use crate::Placeholder;
use crate::RunnerConfig;

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

  pub fn get_value(&self) -> &str {
    match self {
      Arguments::V(value) => value,
      Arguments::KV(_, value) => value,
    }
  }

  pub fn write_css(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
    let Arguments::KV(ref key, ref value) = self else {
      return Ok(());
    };

    let property = Placeholder::normalize(key, config);
    let css_value = Placeholder::normalize(value, config);

    write!(writer, "{property}: {css_value};")?;

    Ok(())
  }

  pub fn write_css_atom(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    atom: &Atom,
  ) -> AnyEmptyResult {
    let Arguments::V(ref value) = self else {
      return Ok(());
    };

    let value = Placeholder::normalize(value, config);

    for (property, css_value) in atom.styles.iter() {
      let property = Placeholder::normalize(property, config);
      let css_value = css_value
        .as_ref()
        .map(|value| Placeholder::normalize(value, config))
        .unwrap_or_else(|| value.clone());

      writeln!(writer, "{property}: {css_value};")?;
    }

    Ok(())
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
