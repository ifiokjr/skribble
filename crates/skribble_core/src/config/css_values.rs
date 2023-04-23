use std::fmt::Write;

use derive_more::Deref;
use derive_more::DerefMut;
use indexmap::IndexMap;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;

use super::Atom;
use super::StringMap;
use crate::AnyEmptyResult;
use crate::Placeholder;
use crate::RunnerConfig;

/// The value of an individual value atom.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CssValue {
  /// A singular value. Use this with named rules.
  Value(String),
  /// Provide an object with the values.
  Object(StringMap),
}

impl CssValue {
  pub fn write_css(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    atom: &Atom,
  ) -> AnyEmptyResult {
    match self {
      Self::Value(value) => {
        let value = Placeholder::normalize(value, config);

        for (property, css_value) in atom.styles.iter() {
          let property = Placeholder::normalize(property, config);
          let css_value = css_value
            .as_ref()
            .map(|content| Placeholder::normalize_value(content, &value, config))
            .unwrap_or_else(|| value.clone());

          writeln!(writer, "{property}: {css_value};")?;
        }
      }
      Self::Object(map) => {
        for (property, css_value) in map.iter() {
          let property = Placeholder::normalize(property, config);
          let css_value = Placeholder::normalize(css_value, config);
          writeln!(writer, "{property}: {css_value};")?;
        }
      }
    }

    Ok(())
  }

  pub fn collect_css_variables(&self, css_variables: &mut IndexSet<String>) {
    match self {
      Self::Value(value) => {
        Placeholder::collect_css_variables(value, css_variables);
      }
      Self::Object(map) => {
        for value in map.values() {
          Placeholder::collect_css_variables(value, css_variables);
        }
      }
    };
  }
}

impl From<&str> for CssValue {
  fn from(value: &str) -> Self {
    Self::Value(value.into())
  }
}

impl From<String> for CssValue {
  fn from(value: String) -> Self {
    Self::Value(value)
  }
}

impl<V: Into<StringMap>> From<V> for CssValue {
  fn from(map: V) -> Self {
    Self::Object(map.into())
  }
}

/// Values for the value atom.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Deref, DerefMut)]
pub struct CssValues(IndexMap<String, CssValue>);

impl IntoIterator for CssValues {
  type IntoIter = indexmap::map::IntoIter<String, CssValue>;
  type Item = (String, CssValue);

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<K, V> FromIterator<(K, V)> for CssValues
where
  K: Into<String>,
  V: Into<CssValue>,
{
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = (K, V)>,
  {
    let values = iter
      .into_iter()
      .map(|(k, v)| (k.into(), v.into()))
      .collect();

    Self(values)
  }
}

impl<K: Into<String>, V: Into<CssValue>> From<IndexMap<K, V>> for CssValues {
  fn from(values: IndexMap<K, V>) -> Self {
    Self::from_iter(values)
  }
}
