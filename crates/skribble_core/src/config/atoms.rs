use std::fmt::Write;

use derive_more::Deref;
use derive_more::DerefMut;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::LinkedValues;
use super::OptionalStringMap;
use super::PrioritizedString;
use super::Priority;
use crate::AnyEmptyResult;
use crate::Arguments;
use crate::Placeholder;
use crate::RunnerConfig;

/// [`Atoms`] are class that take a single value. Each
/// style that is defined as null will be provided the value from the atom
/// style.
///
/// Atoms are defined as a style rule that receives one value from the user.
///
/// Each of the style rules above maps an atomic style name to a list of CSS
/// properties that it controls. The styles rules are later connected with
/// `Atoms` which are passed to each individual style rule.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct Atoms(Vec<Atom>);

impl From<Vec<Atom>> for Atoms {
  fn from(value: Vec<Atom>) -> Self {
    Self(value)
  }
}

impl IntoIterator for Atoms {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = Atom;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for Atoms
where
  V: Into<Atom>,
{
  fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
    let rules = iter.into_iter().map(|value| value.into()).collect();

    Self(rules)
  }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Atom {
  /// The name of the atom.
  #[builder(setter(into))]
  pub name: String,
  /// A markdown description of what this media query should be used for.
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The styles for the specific named rule. All values left as [None] will be
  /// filled with the value provided by the `atom`.
  #[builder(setter(into))]
  pub styles: OptionalStringMap,
  /// The names of the [`ValueSet`]s that will be used to generate the styles.
  #[builder(default, setter(into))]
  pub values: LinkedValues,
}

impl Atom {
  pub fn write_css_properties(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    name: impl AsRef<str>,
  ) -> AnyEmptyResult {
    self
      .values
      .write_css_properties(writer, config, self, name)?;
    Ok(())
  }

  pub fn write_css_argument(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    argument: &Arguments,
  ) -> AnyEmptyResult {
    self
      .values
      .write_css_argument(writer, config, self, argument)?;
    Ok(())
  }

  /// Add a value to the [`ValueSet`] that will be used to generate the builtin
  /// style variants.
  pub fn add_value_set<V: Into<PrioritizedString>>(&mut self, value: V) -> &Self {
    if let LinkedValues::Values(value_set) = &mut self.values {
      value_set.insert(value.into());
    }

    self
  }

  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();

    if self.name != other.name {
      panic!("Cannot merge atoms with different names");
    }

    if let Some(description) = other.description {
      self.description = Some(description);
    }

    if other.priority < self.priority {
      self.priority = other.priority;
    }

    self.styles.extend(other.styles);
    self.values.merge(other.values);
  }

  pub fn collect_css_variables(
    &self,
    config: &RunnerConfig,
    name: Option<&String>,
    css_variables: &mut IndexSet<String>,
  ) {
    if let Some(name) = name {
      self
        .values
        .collect_css_variables(config, name, css_variables);
    }

    for (key, value) in self.styles.iter() {
      Placeholder::collect_css_variables(key, css_variables);

      if let Some(ref content) = value {
        Placeholder::collect_css_variables(content, css_variables);
      }
    }
  }
}
