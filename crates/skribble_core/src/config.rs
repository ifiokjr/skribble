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

  /// Set up the style rules which determine the styles that each atom name will
  /// correspond to.
  pub named_rules: NamedRules,
  /// Shorthand properties.
  ///
  /// ```json
  /// {
  ///   "group": [],
  ///   "container": [
  ///     ["width", "100%"],
  ///     ["max-width", "var(--container-max-width)"]
  ///   ]
  /// }
  /// ```
  pub shortcuts: NamedRules,
  /// Setup the breakpoints.
  pub breakpoints: Breakpoints,
  pub keyframes: Keyframes,
  pub palette: Palette,
}

impl StyleConfig {
  pub fn from_json(json: &str) -> Result<Self> {
    let config: Self =
      serde_json::from_str(json).map_err(|source| Error::InvalidConfig { source })?;
    Ok(config)
  }
}

/// `NamedRules` connect all the atomic names to their atomic styles.
///
/// ```json
/// {
///   "namedRules": {
///     "p": ["padding"],
///     "py": ["padding-top", "padding-bottom"],
///     "px": ["padding-right", "padding-left"],
///     "pt": ["padding-top"],
///     "pr": ["padding-right"],
///     "pb": ["padding-bottom"],
///     "pl": ["padding-left"],
///     "pbl": ["padding-block"],
///     "pbls": ["padding-block-start"],
///     "pble": ["padding-block-end"],
///     "pin": ["padding-inline"],
///     "pins": ["padding-inline-start"],
///     "pine": ["padding-inline-end"]
///   }
/// }
/// ```
///
/// Each of the style rules above maps an atomic style name to a list of CSS
/// properties that it controls. The styles rules are later connected with
/// `Atoms` which are passed to each individual style rule.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NamedRules(IndexMap<String, Vec<StyleRule>>);

impl NamedRules {
  pub fn add_rule(&mut self, name: impl Into<String>, rule: Vec<StyleRule>) {
    self.0.insert(name.into(), rule);
  }

  pub fn update_rule(&mut self, name: impl Into<String>, rule: Vec<StyleRule>) {
    let name: String = name.into();

    match self.0.get_mut(&name) {
      Some(rules) => rules.extend(rule),
      None => self.add_rule(name, rule),
    };
  }

  pub fn add_rules(&mut self, rules: IndexMap<String, Vec<StyleRule>>) {
    self.0.extend(rules);
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd)]
#[serde(untagged)]
pub enum StyleRule {
  /// The rule has a value.
  WithValue(String, CssValue),
  Name(String),
}

impl StyleRule {
  pub fn get_style_declaration(&self, css_value: Option<CssValue>) -> String {
    match self {
      StyleRule::WithValue(name, value) => format!("{}: {}", name, value.get_string()),
      StyleRule::Name(name) => {
        let value = if let Some(v) = css_value {
          v.get_string()
        } else {
          "".to_string()
        };

        format!("{name}: {value}")
      }
    }
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

/// The breakpoints are a shorthand way of creating media queries based on the
/// min-width.
///
/// ```json
/// {
///   "breakpoints": {
///     "sm": "640px",
///     "md": "768px",
///     "lg": "1024px",
///     "xl": "1280px",
///     "xxl": "1536px"
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Breakpoints(IndexMap<String, CssValue>);

impl Breakpoints {
  /// Add a breakpoint to the configuration.
  pub fn insert(&mut self, name: impl Into<String>, value: impl Into<CssValue>) {
    self.0.insert(name.into(), value.into());
  }

  /// Add multiple breakpoints to the configuration.
  pub fn insert_multiple<Name, Value, I>(&mut self, breakpoints: I)
  where
    Name: Into<String>,
    Value: Into<CssValue>,
    I: IntoIterator<Item = (Name, Value)>,
  {
    let iterable = breakpoints.into_iter();
    let reserve = if self.0.is_empty() {
      iterable.size_hint().0
    } else {
      (iterable.size_hint().0 + 1) / 2
    };
    self.0.reserve(reserve);

    iterable.for_each(move |(k, v)| {
      self.insert(k, v);
    });
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
    let config: StyleConfig = serde_json::from_str(include_str!("default.json")).unwrap();
    let json = serde_json::to_string(&config).unwrap();
    assert_eq!(config, serde_json::from_str(&json).unwrap());
  }

  #[test]
  fn default_config() {
    insta::assert_json_snapshot!(StyleConfig::default());
  }
}
