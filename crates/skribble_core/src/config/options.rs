use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::ColorFormat;
use super::MergeRules;

/// Options to use in the configuration.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Options {
  /// The character encoding used in the style sheet
  #[serde(default = "default_charset")]
  #[builder(default = default_charset(), setter(into))]
  pub charset: String,
  /// The default layer to use when no layer is specified.
  #[serde(default = "default_layer")]
  #[builder(default = default_layer(), setter(into))]
  pub layer: String,
  /// This is the default format of colors rendered in css.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub color_format: ColorFormat,
  /// The rules to control how the user configuration is merged with the
  /// configuration extracted from plugins.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub merge_rules: MergeRules,
  /// This determines whether the new [`@property`](https://developer.mozilla.org/en-US/docs/Web/CSS/@property) syntax
  ///  is used for variables. Defaults to `false`.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub use_registered_properties: bool,
  /// Set the prefix that all css variables should use.
  #[serde(default = "default_variable_prefix")]
  #[builder(default = default_variable_prefix(), setter(into))]
  pub variable_prefix: String,
}

impl Default for Options {
  fn default() -> Self {
    Self::builder().build()
  }
}

fn default_variable_prefix() -> String {
  "sk".to_string()
}

fn default_charset() -> String {
  "utf-8".to_string()
}

fn default_layer() -> String {
  "default".to_string()
}
