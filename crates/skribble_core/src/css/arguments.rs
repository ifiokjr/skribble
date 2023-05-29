use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;

use crate::apply_transformers;
use crate::AnyEmptyResult;
use crate::Atom;
use crate::ClassTransformer;
use crate::Placeholder;
use crate::RunnerConfig;
use crate::TransformationRecipient;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Arguments {
  /// A value argument.
  V(String),
  /// A key value argument.
  KV(String, String),
}

impl Arguments {
  pub const DELIMITER: char = '=';

  pub fn v(value: impl AsRef<str>) -> Self {
    let value = value.as_ref().to_string();
    Self::V(value)
  }

  pub fn kv(key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
    let key = key.as_ref().trim().to_string();
    let value = value.as_ref().trim().to_string();

    Self::KV(key, value)
  }

  pub fn is_kv(&self) -> bool {
    match self {
      Arguments::V(_) => false,
      Arguments::KV(..) => true,
    }
  }

  pub fn get_value(&self) -> &str {
    match self {
      Arguments::V(value) => value,
      Arguments::KV(_, value) => value,
    }
  }

  pub fn write_css(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    transformers: &IndexSet<ClassTransformer>,
  ) -> AnyEmptyResult {
    let Arguments::KV(ref key, ref value) = self else {
      return Ok(());
    };

    let property = Placeholder::normalize(key, config);
    let css_value = {
      let normalized_value = Placeholder::normalize(value, config);
      let css_value = apply_transformers(
        normalized_value,
        transformers,
        config,
        TransformationRecipient::Value,
      );
      apply_transformers(
        css_value,
        transformers,
        config,
        TransformationRecipient::Property,
      )
    };

    writeln!(writer, "{property}: {css_value};")?;

    Ok(())
  }

  pub fn write_css_atom(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    atom: &Atom,
    transformers: &IndexSet<ClassTransformer>,
  ) -> AnyEmptyResult {
    let Arguments::V(ref value) = self else {
      return Ok(());
    };

    let value = {
      let normalized_value = Placeholder::normalize(value, config);
      apply_transformers(
        normalized_value,
        transformers,
        config,
        TransformationRecipient::Value,
      )
    };

    for (property, css_value) in atom.styles.iter() {
      let property = Placeholder::normalize(property, config);
      let css_value = {
        let css_value = css_value
          .as_ref()
          .map(|value| Placeholder::normalize(value, config))
          .unwrap_or_else(|| value.clone());
        apply_transformers(
          css_value,
          transformers,
          config,
          TransformationRecipient::Property,
        )
      };

      writeln!(writer, "{property}: {css_value};")?;
    }

    Ok(())
  }
}

impl Display for Arguments {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Arguments::V(value) => write!(f, "{}", value),
      Arguments::KV(key, value) => write!(f, "{}{}{}", key, Arguments::DELIMITER, value),
    }
  }
}

impl<S: AsRef<str>> From<S> for Arguments {
  fn from(value: S) -> Self {
    match value.as_ref().split_once(Arguments::DELIMITER) {
      Some((key, value)) => Self::kv(key, value),
      None => Self::v(value),
    }
  }
}
