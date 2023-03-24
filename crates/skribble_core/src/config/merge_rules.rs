use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// The rules to follow when merging the provided configuration with the derived
/// configuration from plugins.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct MergeRules {
  #[builder(default, setter(into))]
  pub keyframes: MergeRule,
  #[builder(default, setter(into))]
  pub variables: MergeRule,
  #[builder(default, setter(into))]
  pub media_queries: MergeRule,
  #[builder(default, setter(into))]
  pub modifiers: MergeRule,
  #[builder(default, setter(into))]
  pub rules: MergeRule,
  #[builder(default, setter(into))]
  pub classes: MergeRule,
  #[builder(default, setter(into))]
  pub palette: MergeRule,
  #[builder(default, setter(into))]
  pub atoms: MergeRule,
  #[builder(default, setter(into))]
  pub groups: MergeRule,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MergeRule {
  /// The configuration provided by the user will be merged with the
  /// configuration from the plugins. Any clashing keys will be overwritten by
  /// the user configuration.
  #[default]
  Append,
  /// The configuration provided by the user will be applied first and the
  /// configuration from the plugins will be applied second. Any clashing keys
  /// will be overwritten by the plugin configuration.
  Prepend,
  /// Only the configuration from the user will be used.
  Replace,
  /// Only the configuration from the plugins will be used.
  Ignore,
  /// The configuration will be reset to the default value and neither the user
  /// configuration nor the plugin configuration will be used (not recommended -
  /// mainly here for completeness)..
  Reset,
}

impl Display for MergeRule {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let value = match self {
      MergeRule::Append => "append",
      MergeRule::Prepend => "prepend",
      MergeRule::Replace => "replace",
      MergeRule::Ignore => "ignore",
      MergeRule::Reset => "reset",
    };
    write!(f, "{value}")
  }
}

impl<V: Into<String>> From<V> for MergeRule {
  fn from(value: V) -> Self {
    match value.into().as_str() {
      "append" => MergeRule::Append,
      "prepend" => MergeRule::Prepend,
      "replace" => MergeRule::Replace,
      "ignore" => MergeRule::Ignore,
      "reset" => MergeRule::Reset,
      _ => MergeRule::Append,
    }
  }
}
