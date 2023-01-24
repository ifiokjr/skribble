use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Options to use in the configuration.
#[derive(Default, Serialize, Deserialize, TypedBuilder, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Options {
  #[serde(default)]
  pub color_format: ColorFormat,

  /// By default there is no variable prefix.
  #[serde(default)]
  pub variable_prefix: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub enum ColorFormat {
  #[serde(rename = "rgb")]
  Rgb,
  #[serde(rename = "hsl")]
  #[default]
  Hsl,
}

/// The min width can either be a string or a number. If a number it will be
/// interpreted as a pixel measurement.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd)]
#[serde(untagged)]
pub enum CssValue {
  Number(f32),
  String(String),
}

impl<T: AsRef<str>> From<T> for CssValue {
  fn from(value: T) -> Self {
    CssValue::String(value.as_ref().to_string())
  }
}

impl CssValue {
  pub fn get_string(&self) -> String {
    match self {
      CssValue::Number(value) => value.to_string(),
      CssValue::String(value) => value.clone(),
    }
  }
}
