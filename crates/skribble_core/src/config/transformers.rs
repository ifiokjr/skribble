use std::fmt::Display;

use derive_more::Deref;
use derive_more::DerefMut;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::Group;
use super::Priority;
use super::StringList;
use super::StringMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TransformationScope {
  /// Apply the transformer to all values (like `!important`)
  All,
  /// Apply the transformer to all color atoms.
  Color,
  /// Apply the transformer to a selection of atoms.
  Atoms(StringList),
  /// Apply the transformer to a selection of value sets.
  ValueSets(StringList),
}

impl<T: Into<String>> From<T> for TransformationScope {
  fn from(value: T) -> Self {
    let value = value.into();

    match value.as_str() {
      "all" | "*" => Self::All,
      "color" => Self::Color,
      rest => Self::Atoms(StringList::from(vec![rest.to_string()])),
    }
  }
}

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TransformationRecipient {
  /// Only transform the values in the atom.
  Value,
  /// Transform all the properties for the impacted atom.
  #[default]
  Property,
}

impl<T: Into<String>> From<T> for TransformationRecipient {
  fn from(value: T) -> Self {
    let value = value.into();

    match value.as_str() {
      "value" => Self::Value,
      "property" => Self::Property,
      _ => Self::Property,
    }
  }
}

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CalcSymbol {
  /// Add the provided value to the current value.
  Add,
  /// Subtract the provided value from the current value.
  Subtract,
  /// Multiply the provided value with the current value.
  Multiply,
  /// Divide the current value by the provided value.
  Divide,
  /// Set the current value to the provided value.
  #[default]
  Set,
}

impl<T: Into<String>> From<T> for CalcSymbol {
  fn from(value: T) -> Self {
    let value = value.into();

    match value.as_str() {
      "+" => Self::Add,
      "-" => Self::Subtract,
      "*" => Self::Multiply,
      "/" => Self::Divide,
      "=" => Self::Set,
      _ => Self::Set,
    }
  }
}

impl Display for CalcSymbol {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let symbol = match self {
      Self::Add => "+",
      Self::Subtract => "-",
      Self::Multiply => "*",
      Self::Divide => "/",
      Self::Set => "=",
    };

    write!(f, "{}", symbol)
  }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ColorProperty {
  Hue(CalcSymbol),
  Saturation(CalcSymbol),
  Lightness(CalcSymbol),
  Alpha(CalcSymbol),
}

impl ColorProperty {
  pub fn set_symbol(&mut self, symbol: CalcSymbol) {
    *self = match self {
      Self::Hue(_) => Self::Hue(symbol),
      Self::Saturation(_) => Self::Saturation(symbol),
      Self::Lightness(_) => Self::Lightness(symbol),
      Self::Alpha(_) => Self::Alpha(symbol),
    };
  }
}

impl<T: Into<String>> From<T> for ColorProperty {
  fn from(value: T) -> Self {
    let value = value.into();

    match value.as_str() {
      "hue" => Self::Hue(Default::default()),
      "saturation" => Self::Saturation(Default::default()),
      "lightness" => Self::Lightness(Default::default()),
      "alpha" => Self::Alpha(Default::default()),
      value => {
        let Some((name, ch)) = value.strip_suffix(value).and_then(|v| v.split_once('(')) else {
          return Self::Alpha(Default::default());
        };

        let mut property: Self = name.into();
        property.set_symbol(ch.into());
        property
      }
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Transformation {
  /// Wrap the `&` with the provided string.
  Replacement(String),
  /// Replace the provided regex with the provided string.
  RegexReplacement { regex: String, replacement: String },
  /// Update the color using hsl color properties.
  Color(ColorProperty),
}

impl<T: Into<String>> From<T> for Transformation {
  fn from(value: T) -> Self {
    Self::Replacement(value.into())
  }
}

impl From<ColorProperty> for Transformation {
  fn from(value: ColorProperty) -> Self {
    Self::Color(value)
  }
}

/// Transformers are used to modify the values of an atom based on the value
/// sets / colors it receives.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Transformer {
  /// The name of the parent modifier will be used to reference
  #[builder(setter(into))]
  pub name: String,
  /// The description for this item
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// Wrap the `&` with the provided string.
  /// TODO replace with `Transformation` enum.
  #[builder(setter(into))]
  pub transformation: Transformation,
  /// The named values for the transformation.
  #[builder(default, setter(into, strip_option))]
  pub values: Option<StringMap>,
  /// The scope of atoms this transformer will impact.
  #[builder(setter(into))]
  pub scope: TransformationScope,
  /// The recipient of the transformation whether it's the value or the
  /// properties of an atom.
  #[builder(setter(into))]
  pub recipient: TransformationRecipient,
  /// The priority for this item.
  #[builder(default, setter(into))]
  pub priority: Priority,
}

impl Transformer {
  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();

    if self.name != other.name {
      panic!("Cannot merge modifiers with different names");
    }

    if let Some(description) = other.description {
      self.description = Some(description);
    }

    if other.priority < self.priority {
      self.priority = other.priority;
    }

    if let Some(values) = self.values.as_mut() {
      if let Some(other_values) = other.values {
        values.extend(other_values);
      }
    } else {
      self.values = other.values;
    }

    self.transformation = other.transformation;
    self.scope = other.scope;
  }
}

/// This is the setup for named modifiers.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Deref, DerefMut)]
pub struct Transformers(Vec<Group<Transformer>>);

impl Transformers {
  /// Extend an existing group or create a new one if it does not exist.
  pub fn extend_group(&mut self, group: impl Into<Group<Transformer>>) {
    let group = group.into();

    if let Some(existing_group) = self.0.iter_mut().find(|g| g.name == group.name) {
      existing_group.merge(group);
    } else {
      self.0.push(group);
    }
  }
}

impl From<Vec<Group<Transformer>>> for Transformers {
  fn from(modifiers: Vec<Group<Transformer>>) -> Self {
    Self(modifiers)
  }
}

impl IntoIterator for Transformers {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = Group<Transformer>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for Transformers
where
  V: Into<Group<Transformer>>,
{
  fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
    Self(iter.into_iter().map(|v| v.into()).collect())
  }
}
