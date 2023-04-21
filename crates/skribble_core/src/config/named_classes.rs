use std::fmt::Write;

use derive_more::Deref;
use derive_more::DerefMut;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::Priority;
use super::StringMap;
use crate::AnyEmptyResult;
use crate::Placeholder;
use crate::RunnerConfig;

/// The named classes with their own defined values.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Deref, DerefMut)]
pub struct NamedClasses(Vec<NamedClass>);

impl IntoIterator for NamedClasses {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = NamedClass;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for NamedClasses
where
  V: Into<NamedClass>,
{
  fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
    let classes = iter.into_iter().map(|value| value.into()).collect();

    Self(classes)
  }
}

/// A named class is a class with all it's values defined ahead of time.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct NamedClass {
  /// The name of the standalone class.
  #[builder(setter(into))]
  pub name: String,
  /// A markdown description of what this standalone class should be used for.
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The styles for the specific class.
  #[builder(setter(into))]
  pub styles: StringMap,
}

impl NamedClass {
  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();

    if self.name != other.name {
      panic!("Cannot merge named classes with different names");
    }

    if let Some(description) = other.description {
      self.description = Some(description);
    }

    if other.priority < self.priority {
      self.priority = other.priority;
    }

    self.styles.extend(other.styles);
  }

  pub fn write_css_properties(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
  ) -> AnyEmptyResult {
    for (property, css_value) in self.styles.iter() {
      let property = Placeholder::normalize(property, config);
      let css_value = Placeholder::normalize(css_value, config);
      writeln!(writer, "{}: {};", property, css_value)?;
    }

    Ok(())
  }

  pub fn collect_css_variables(&self, css_variables: &mut IndexSet<String>) {
    for (property, css_value) in self.styles.iter() {
      Placeholder::collect_css_variables(property, css_variables);
      Placeholder::collect_css_variables(css_value, css_variables);
    }
  }
}
