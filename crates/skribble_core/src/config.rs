use std::ops::Deref;
use std::ops::DerefMut;

use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::Error;
use crate::Result;

/// The style configuration which can also use the builder pattern.
#[derive(Serialize, Deserialize, TypedBuilder, Default, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StyleConfig {
  /// The general options.
  pub options: Options,
  /// Setup the media queries.
  pub media_queries: MediaQueries,
  /// Set up the style rules which determine the styles that each atom name will
  /// correspond to.
  pub rules: NamedRules,
  /// Named classes.
  pub classes: NamedClasses,
  /// Setup the keyframes.
  pub keyframes: Keyframes,
  pub palette: Palette,
}

impl StyleConfig {
  pub fn from_json(json: impl AsRef<str>) -> Result<Self> {
    let config: Self =
      serde_json::from_str(json.as_ref()).map_err(|source| Error::InvalidConfig { source })?;
    Ok(config)
  }

  pub fn to_json(&self) -> Result<String> {
    serde_json::to_string(self).map_err(|source| Error::CouldNotSerializeConfig { source })
  }

  pub fn to_pretty_json(&self) -> Result<String> {
    serde_json::to_string_pretty(self).map_err(|source| Error::CouldNotSerializeConfig { source })
  }
}

/// `NamedRules` connect all the atomic names to their atomic styles. Each style
/// that is defined as null will be provided the value from the atom style.
///
/// Atoms are defined as a style rule that receives one value from the user.
///
/// ```json
/// {
///   "rules": {
///     "p": { "padding": null },
///     "py": { "padding-top": null, "padding-bottom": null },
///     "px": { "padding-right": null, "padding-left": null },
///     "pt": { "padding-top": null },
///     "pr": { "padding-right": null },
///     "pb": { "padding-bottom": null },
///     "pl": { "padding-left": null },
///     "pbl": { "padding-block": null },
///     "pbls": { "padding-block-start": null },
///     "pble": { "padding-block-end": null },
///     "pin": { "padding-inline": null },
///     "pins": { "padding-inline-start": null },
///     "pine": { "padding-inline-end": null }
///   }
/// }
/// ```
///
/// Each of the style rules above maps an atomic style name to a list of CSS
/// properties that it controls. The styles rules are later connected with
/// `Atoms` which are passed to each individual style rule.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NamedRules(IndexMap<String, IndexMap<String, Option<CssValue>>>);

impl<K, C, V, I> From<I> for NamedRules
where
  K: Into<String>,
  C: Into<CssValue>,
  V: IntoIterator<Item = (K, Option<C>)>,
  I: IntoIterator<Item = (K, V)>,
{
  fn from(value: I) -> Self {
    let mut rules = IndexMap::new();

    for (name, values) in value {
      let name = name.into();
      let values = values
        .into_iter()
        .map(|(name, value)| (name.into(), value.map(|v| v.into())))
        .collect();

      rules.insert(name, values);
    }

    Self(rules)
  }
}

/// The named classes with their own defined values.
///
/// ```json
/// {
///   "group": {}, // Empty class
///   "container": {
///     "width": "100%",
///     "max-width": "var(--container-max-width)"
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NamedClasses(IndexMap<String, IndexMap<String, CssValue>>);

impl<K, C, V, I> From<I> for NamedClasses
where
  K: Into<String>,
  C: Into<CssValue>,
  V: IntoIterator<Item = (K, C)>,
  I: IntoIterator<Item = (K, V)>,
{
  fn from(value: I) -> Self {
    let mut classes = IndexMap::new();

    for (name, values) in value {
      let name = name.into();
      let values = values
        .into_iter()
        .map(|(name, value)| (name.into(), value.into()))
        .collect();

      classes.insert(name, values);
    }

    Self(classes)
  }
}

/// Options to use in the configuration.
#[derive(Serialize, Deserialize, TypedBuilder, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Options {
  #[serde(default)]
  pub color_format: ColorFormat,

  /// By default there is no variable prefix.
  #[serde(default = "default_variable_prefix")]
  pub variable_prefix: String,
}

impl Default for Options {
  fn default() -> Self {
    Self {
      color_format: Default::default(),
      variable_prefix: default_variable_prefix(),
    }
  }
}

fn default_variable_prefix() -> String {
  "sk".to_string()
}

/// ColorFormat is used to determine the default format of the colors.
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

/// Media queries can should be defined as a map of names to their css queries.
///
/// ```json
/// {
///   "mediaQueries": {
///     "sm": "(min-width: 640px)",
///     "md": "(min-width: 768px)",
///     "lg": "(min-width: 1024px)",
///     "xl": "(min-width: 1280px)",
///     "xxl": "(min-width: 1536px)",
///     "portrait": "(orientation: portrait)",
///     "combined": "(min-width: 30em) and (orientation: landscape)"
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MediaQueries(IndexMap<String, String>);

impl MediaQueries {
  pub fn breakpoint(
    &mut self,
    name: impl Into<String>,
    value: impl Into<String>,
  ) -> Option<String> {
    let query = format!("(min-width: {})", value.into());
    self.0.insert(name.into(), query)
  }
}

impl<K: Into<String>, V: Into<String>, I: IntoIterator<Item = (K, V)>> From<I> for MediaQueries {
  fn from(value: I) -> Self {
    let mut breakpoints = IndexMap::new();

    for (name, value) in value {
      breakpoints.insert(name.into(), value.into());
    }

    Self(breakpoints)
  }
}

impl Deref for MediaQueries {
  type Target = IndexMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for MediaQueries {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// Create a palette for the configuration.
///
/// ```json
/// {
///   "palette": {
///     "inherit": "inherit",
///     "current": "currentColor",
///     "transparent": "transparent",
///     "black": "#000",
///     "white": "#fff",
///     "slate50": "#f8fafc",
///     "slate100": "#f1f5f9",
///     "slate200": "#e2e8f0",
///     "slate300": "#cbd5e1",
///     "slate400": "#94a3b8",
///     "slate500": "#64748b",
///     "slate600": "#475569",
///     "slate700": "#334155",
///     "slate800": "#1e293b",
///     "slate900": "#0f172a",
///     "gray50": "#f9fafb",
///     "gray100": "#f3f4f6",
///     "gray200": "#e5e7eb",
///     "gray300": "#d1d5db",
///     "gray400": "#9ca3af",
///     "gray500": "#6b7280",
///     "gray600": "#4b5563",
///     "gray700": "#374151",
///     "gray800": "#1f2937",
///     "gray900": "#111827"
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Palette(IndexMap<String, String>);

type Frames = IndexMap<String, IndexMap<String, CssValue>>;
/// This setups up the animation keyframes for the configuration. The names can
/// be reference in the atoms.
///
/// ```json
/// {
///   "keyframes": {
///     "enter": {
///       "from": {
///         "opacity": "var(--enter-opacity, 1)",
///         "transform": "translate3d(var(--enter-translate-x, 0), var(--enter-translate-y, 0), 0) scale3d(var(--enter-scale, 1), var(--enter-scale, 1), var(--enter-scale, 1)) rotate(var(--enter-rotate, 0))"
///       }
///     },
///     "exit": {
///       "to": {
///         "opacity": "var(--exit-opacity, 1)",
///         "transform": "translate3d(var(--exit-translate-x, 0), var(--exit-translate-y, 0), 0) scale3d(var(--exit-scale, 1), var(--exit-scale, 1), var(--exit-scale, 1)) rotate(var(--exit-rotate, 0))"
///       }
///     },
///     "spin": {
///       "from": { "transform": "rotate(0deg)" },
///       "to": { "transform": "rotate(360deg)" }
///     },
///     "ping": { "75%, 100%": { "transform": "scale(2)", "opacity": "0" } },
///     "pulse": { "0%, 100%": { "opacity": "1" }, "50%": { "opacity": "0.5" } },
///     "bounce": {
///       "0%, 100%": {
///         "transform": "translateY(-25%)",
///         "animationTimingFunction": "cubic-bezier(0.8, 0, 1, 1)"
///       },
///       "50%": {
///         "transform": "translateY(0)",
///         "animationTimingFunction": "cubic-bezier(0, 0, 0.2, 1)"
///       }
///     }
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Keyframes(Frames);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_config_can_serialize() {
    let config: StyleConfig = Default::default();
    let json = config.to_json().unwrap();
    assert_eq!(config, serde_json::from_str(&json).unwrap());
  }

  #[test]
  fn default_config() {
    insta::assert_json_snapshot!(StyleConfig::default());
  }
}
