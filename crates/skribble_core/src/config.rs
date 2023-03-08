use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;
use std::ops::DerefMut;

use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use typed_builder::TypedBuilder;

use crate::Error;
use crate::Result;

/// The style configuration which can also use the builder pattern.
#[derive(Serialize, Deserialize, TypedBuilder, Default, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StyleConfig {
  /// The general options.
  #[builder(default, setter(into))]
  pub options: Options,
  /// Setup the keyframes.
  #[builder(default, setter(into))]
  pub keyframes: Keyframes,
  /// CSS variables which can be reused throughout the configuration.
  pub variables: CssVariables,
  /// Setup the media queries.
  #[builder(default, setter(into))]
  pub media_queries: MediaQueries,
  /// Modifiers are used to nest styles within a selector.
  #[builder(default, setter(into))]
  pub parent_modifiers: ParentModifiers,
  /// Modifiers are used to nest styles within a selector.
  #[builder(default, setter(into))]
  pub modifiers: Modifiers,
  /// Set up the style rules which determine the styles that each atom name will
  /// correspond to.
  #[builder(default, setter(into))]
  pub rules: NamedRules,
  /// Named classes.
  #[builder(default, setter(into))]
  pub classes: NamedClasses,
  /// Hardcoded colors for the pallette.
  #[builder(default, setter(into))]
  pub palette: Palette,
  /// Support additional fields for plugins.
  #[serde(flatten, default)]
  #[builder(default, setter(into))]
  pub additional_fields: AdditionalFields,
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

/// Options to use in the configuration.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Options {
  /// This is the default format of colors rendered in css.
  #[serde(default)]
  pub color_format: ColorFormat,
  /// By default there is no variable prefix.
  #[serde(default = "default_variable_prefix")]
  pub variable_prefix: String,
  /// The character encoding used in the style sheet
  pub charset: Option<String>,
  /// This determines whether the new [`@property`](https://developer.mozilla.org/en-US/docs/Web/CSS/@property) syntax
  ///  is used for variables. Defaults to false.
  #[builder(default, setter(into))]
  pub use_registered_properties: bool,
}

impl Default for Options {
  fn default() -> Self {
    Self {
      color_format: Default::default(),
      variable_prefix: default_variable_prefix(),
      charset: Some("utf-8".to_string()),
      use_registered_properties: false,
    }
  }
}

fn default_variable_prefix() -> String {
  "sk".to_string()
}

/// ColorFormat is used to determine the default format of the colors.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ColorFormat {
  #[serde(rename = "rgb")]
  Rgb,
  #[serde(rename = "hsl")]
  #[default]
  Hsl,
}

impl AsRef<str> for ColorFormat {
  fn as_ref(&self) -> &str {
    match self {
      Self::Rgb => "rgb",
      Self::Hsl => "hsl",
    }
  }
}

impl<T: Into<String>> From<T> for ColorFormat {
  fn from(value: T) -> Self {
    match value.into().as_str() {
      "rgb" => Self::Rgb,
      "hsl" => Self::Hsl,
      _ => Self::Hsl,
    }
  }
}

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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Keyframes(Vec<Keyframe>);

impl<V, I> From<I> for Keyframes
where
  V: Into<Keyframe>,
  I: IntoIterator<Item = V>,
{
  fn from(iter: I) -> Self {
    Self(iter.into_iter().map(|value| value.into()).collect())
  }
}

impl<V> FromIterator<V> for Keyframes
where
  V: Into<Keyframe>,
{
  fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
    Self::from(iter)
  }
}

impl Deref for Keyframes {
  type Target = Vec<Keyframe>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Keyframes {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Keyframe {
  /// The name of the keyframe.
  pub name: String,
  /// The description of the keyframe. This will be used in the vscode
  /// extension.
  #[builder(default, setter(into))]
  pub description: Option<String>,
  /// The rules for the specific keyframe.
  #[serde(flatten, default)]
  #[builder(default, setter(into))]
  pub rules: KeyframeRules,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KeyframeRules(IndexMap<String, String>);

impl<K, V, I> From<I> for KeyframeRules
where
  K: Into<String>,
  V: Into<String>,
  I: IntoIterator<Item = (K, V)>,
{
  fn from(iter: I) -> Self {
    let rules = iter
      .into_iter()
      .map(|(key, value)| (key.into(), value.into()))
      .collect();

    Self(rules)
  }
}

impl<K, V> FromIterator<(K, V)> for KeyframeRules
where
  K: Into<String>,
  V: Into<String>,
{
  fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
    Self::from(iter)
  }
}

impl Deref for KeyframeRules {
  type Target = IndexMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for KeyframeRules {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
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

impl<K, V, I> From<I> for MediaQueries
where
  K: Into<String>,
  V: Into<String>,
  I: IntoIterator<Item = (K, V)>,
{
  fn from(value: I) -> Self {
    let mut breakpoints = IndexMap::new();

    for (name, value) in value {
      breakpoints.insert(name.into(), value.into());
    }

    Self(breakpoints)
  }
}

impl<K, V> FromIterator<(K, V)> for MediaQueries
where
  K: Into<String>,
  V: Into<String>,
{
  fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
    Self::from(iter)
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
pub struct NamedRules(IndexMap<String, IndexMap<String, Option<String>>>);

impl<K, C, V, I> From<I> for NamedRules
where
  K: Into<String>,
  C: Into<String>,
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

impl<K, C, V> FromIterator<(K, V)> for NamedRules
where
  K: Into<String>,
  C: Into<String>,
  V: IntoIterator<Item = (K, Option<C>)>,
{
  fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
    Self::from(iter)
  }
}

impl Deref for NamedRules {
  type Target = IndexMap<String, IndexMap<String, Option<String>>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for NamedRules {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
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
pub struct NamedClasses(IndexMap<String, IndexMap<String, String>>);

impl<K, C, V, I> From<I> for NamedClasses
where
  K: Into<String>,
  C: Into<String>,
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

impl<K, C, V> FromIterator<(K, V)> for NamedClasses
where
  K: Into<String>,
  C: Into<String>,
  V: IntoIterator<Item = (K, C)>,
{
  fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
    Self::from(iter)
  }
}

impl Deref for NamedClasses {
  type Target = IndexMap<String, IndexMap<String, String>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for NamedClasses {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// Create CSS variables from a list of atoms.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CssVariables(Vec<CssVariable>);

impl<V, I> From<I> for CssVariables
where
  V: Into<CssVariable>,
  I: IntoIterator<Item = V>,
{
  fn from(value: I) -> Self {
    Self(value.into_iter().map(|v| v.into()).collect())
  }
}

impl<V> FromIterator<V> for CssVariables
where
  V: Into<CssVariable>,
{
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    Self::from(iter)
  }
}

impl Deref for CssVariables {
  type Target = Vec<CssVariable>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for CssVariables {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

pub type CssVariableSelectors = IndexMap<String, String>;
pub type NestedCssVariableSelectors = IndexMap<String, CssVariableSelectors>;

/// This can be used to define colors and other CSS variables.
///
/// For colors you should set the `syntax` to [PropertySyntaxValue::Color].
///
/// All CSS variables are made available in the produced code.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CssVariable {
  /// A required name. This should always start with `--`.
  #[builder(setter(into))]
  pub name: String,
  /// A description of the CSS variable and what it is used for.
  #[builder(default, setter(strip_option, into))]
  pub description: Option<String>,
  /// The [syntax](https://developer.mozilla.org/en-US/docs/Web/CSS/@property/syntax) of the CSS variable.
  #[builder(default, setter(into))]
  pub syntax: PropertySyntax,
  /// The initial value of the CSS variable. This is required if the
  /// [PropertySyntax] is set to anything other than [PropertySyntaxValue::Any].
  #[builder(default, setter(into, strip_option))]
  pub initial_value: Option<String>,
  /// Define the value of the CSS variables for each selector.
  #[builder(default, setter(strip_option, into))]
  pub selectors: Option<CssVariableSelectors>,
  /// Define the value of the CSS variable for the nested media query.
  #[builder(default, setter(strip_option, into))]
  pub media_queries: Option<NestedCssVariableSelectors>,
}

impl CssVariable {
  #[inline]
  pub fn get_name(&self) -> &str {
    &self.name
  }

  #[inline]
  pub fn get_variable(&self) -> String {
    format!("var({})", self.name)
  }

  #[inline]
  pub fn get_variable_with_fallback(&self, fallback: &str) -> String {
    format!("var({}, {})", self.name, fallback)
  }
}

impl<T: Into<String>> From<T> for CssVariable {
  #[inline]
  fn from(name: T) -> Self {
    CssVariable::builder().name(name).build()
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PropertySyntax {
  Value(PropertySyntaxValue),
  List(Vec<PropertySyntaxValue>),
}

impl PropertySyntax {
  #[inline]
  pub fn from_string<T: Into<String>>(value: T) -> Self {
    PropertySyntax::Value(PropertySyntaxValue::from(value))
  }

  #[inline]
  pub fn from_iterator<V: Into<String>, I: IntoIterator<Item = V>>(iter: I) -> Self {
    let property = iter
      .into_iter()
      .map(|v| PropertySyntaxValue::from(v))
      .collect();

    PropertySyntax::List(property)
  }

  #[inline]
  pub fn is_color(&self) -> bool {
    match self {
      PropertySyntax::Value(value) => *value == PropertySyntaxValue::Color,
      PropertySyntax::List(_) => false,
    }
  }
}

impl Default for PropertySyntax {
  fn default() -> Self {
    PropertySyntax::Value(PropertySyntaxValue::Any)
  }
}

impl<T: Into<String>> From<T> for PropertySyntax {
  fn from(value: T) -> Self {
    Self::from_string(value)
  }
}

impl Display for PropertySyntax {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      PropertySyntax::Value(value) => write!(f, "{}", value),
      PropertySyntax::List(values) => {
        let values = values
          .iter()
          .map(|v| v.to_string())
          .collect::<Vec<String>>()
          .join(" | ");

        write!(f, "{}", values)
      }
    }
  }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PropertySyntaxValue {
  /// Any valid <length> values.
  #[serde(rename = "<length>")]
  Length,
  #[serde(rename = "<number>")]
  Number,
  /// Any valid <percentage> values.
  #[serde(rename = "<percentage>")]
  Percentage,
  /// Any valid <length-percentage> values.
  #[serde(rename = "<length-percentage>")]
  LengthPercentage,
  /// Any valid <color> values.
  #[serde(rename = "<color>")]
  Color,
  /// Any valid <image> values.
  #[serde(rename = "<image>")]
  Image,
  /// Any valid url() values.
  #[serde(rename = "<url>")]
  Url,
  /// Any valid <integer> values.
  #[serde(rename = "<integer>")]
  Integer,
  /// Any valid <angle> values.
  #[serde(rename = "<angle>")]
  Angle,
  /// Any valid <time> values.
  #[serde(rename = "<time>")]
  Time,
  /// Any valid <resolution> values.
  #[serde(rename = "<resolution>")]
  Resolution,
  /// Any valid <transform-function> values.
  #[serde(rename = "<transform-function>")]
  TransformFunction,
  /// Any valid <custom-ident> values.
  #[serde(rename = "<custom-ident>")]
  CustomIdent,
  /// A list of valid <transform-function> values.
  #[serde(rename = "<transform-list>")]
  TransformList,
  /// Any valid token
  #[serde(rename = "*")]
  #[default]
  Any,
  /// Accepts this value as custom idents
  String(String),
}

impl<T: Into<String>> From<T> for PropertySyntaxValue {
  fn from(value: T) -> Self {
    let value = value.into();

    match value.as_str() {
      "<length>" => PropertySyntaxValue::Length,
      "<number>" => PropertySyntaxValue::Number,
      "<percentage>" => PropertySyntaxValue::Percentage,
      "<length-percentage>" => PropertySyntaxValue::LengthPercentage,
      "<color>" => PropertySyntaxValue::Color,
      "<image>" => PropertySyntaxValue::Image,
      "<url>" => PropertySyntaxValue::Url,
      "<integer>" => PropertySyntaxValue::Integer,
      "<angle>" => PropertySyntaxValue::Angle,
      "<time>" => PropertySyntaxValue::Time,
      "<resolution>" => PropertySyntaxValue::Resolution,
      "<transform-function>" => PropertySyntaxValue::TransformFunction,
      "<custom-ident>" => PropertySyntaxValue::CustomIdent,
      "<transform-list>" => PropertySyntaxValue::TransformList,
      "*" => PropertySyntaxValue::Any,
      _ => PropertySyntaxValue::String(value),
    }
  }
}

impl AsRef<str> for PropertySyntaxValue {
  fn as_ref(&self) -> &str {
    match self {
      PropertySyntaxValue::Length => "<length>",
      PropertySyntaxValue::Number => "<number>",
      PropertySyntaxValue::Percentage => "<percentage>",
      PropertySyntaxValue::LengthPercentage => "<length-percentage>",
      PropertySyntaxValue::Color => "<color>",
      PropertySyntaxValue::Image => "<image>",
      PropertySyntaxValue::Url => "<url>",
      PropertySyntaxValue::Integer => "<integer>",
      PropertySyntaxValue::Angle => "<angle>",
      PropertySyntaxValue::Time => "<time>",
      PropertySyntaxValue::Resolution => "<resolution>",
      PropertySyntaxValue::TransformFunction => "<transform-function>",
      PropertySyntaxValue::CustomIdent => "<custom-ident>",
      PropertySyntaxValue::TransformList => "<transform-list>",
      PropertySyntaxValue::Any => "*",
      PropertySyntaxValue::String(value) => value,
    }
  }
}

impl Display for PropertySyntaxValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_ref())
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Palette(IndexMap<String, String>);

impl<K, V, I> From<I> for Palette
where
  K: Into<String>,
  V: Into<String>,
  I: IntoIterator<Item = (K, V)>,
{
  fn from(value: I) -> Self {
    let palette = value
      .into_iter()
      .map(|(name, value)| (name.into(), value.into()))
      .collect();

    Self(palette)
  }
}

impl<K, V> FromIterator<(K, V)> for Palette
where
  K: Into<String>,
  V: Into<String>,
{
  fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
    Self::from(iter)
  }
}

impl Deref for Palette {
  type Target = IndexMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Palette {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// This is the setup for the parent modifiers.
///
/// ```json
/// {
///   "parentModifiers": {
///     "light": [".light &"],
///     "dark": [".dark &"],
///     "rtl": ["[dir=rtl] &"],
///     "groupHover": [
///       ".\\$group:hover &",
///       ".group:hover &",
///       "[role='group']:hover &"
///     ],
///     "groupFocus": [
///       ".\\$group:focus &",
///       ".group:focus &",
///       "[role='group']:focus &"
///     ],
///     "groupActive": [
///       ".\\$group:active &",
///       ".group:active &",
///       "[role='group']:active &"
///     ],
///     "groupVisited": [
///       ".\\$group:visited &",
///       ".group:visited &",
///       "[role='group']:visited &"
///     ]
///   }
/// }
/// ```
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ParentModifiers(IndexMap<String, Vec<String>>);

impl<K, C, V, I> From<I> for ParentModifiers
where
  K: Into<String>,
  C: Into<String>,
  V: IntoIterator<Item = C>,
  I: IntoIterator<Item = (K, V)>,
{
  fn from(iter: I) -> Self {
    let parent_modifiers = iter
      .into_iter()
      .map(|(k, v)| (k.into(), v.into_iter().map(|v| v.into()).collect()))
      .collect();

    Self(parent_modifiers)
  }
}

impl<K, C, V> FromIterator<(K, V)> for ParentModifiers
where
  K: Into<String>,
  C: Into<String>,
  V: IntoIterator<Item = C>,
{
  fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
    Self::from(iter)
  }
}

impl Deref for ParentModifiers {
  type Target = IndexMap<String, Vec<String>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for ParentModifiers {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// This is the setup for named modifiers.
///
/// ```json
/// [
///   { "hover": ["&:hover"] },
///   { "active": ["&:active"] },
///   { "focus": ["&:focus"] },
///   { "focusWithin": ["&:focus-within"] },
///   { "focusVisible": ["&:focus-visible"] },
///   {
///     "disabled": ["&[disabled]", "&[aria-disabled=true]", "&:disabled"],
///     "notDisabled": ["&[aria-disabled=false]", "&:disabled"],
///     "enabled": ["&:enabled"]
///   },
///   { "empty": ["&:empty"] },
/// ]
/// ```
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Modifiers(Vec<IndexMap<String, Vec<String>>>);

impl<K, IK, IV, V, I> From<I> for Modifiers
where
  K: Into<String>,
  IK: Into<String>,
  IV: IntoIterator<Item = IK>,
  V: IntoIterator<Item = (K, IV)>,
  I: IntoIterator<Item = V>,
{
  fn from(iter: I) -> Self {
    let modifiers = iter
      .into_iter()
      .map(|v| {
        v.into_iter()
          .map(|(k, iv)| (k.into(), iv.into_iter().map(|ik| ik.into()).collect()))
          .collect()
      })
      .collect();

    Self(modifiers)
  }
}

impl Deref for Modifiers {
  type Target = Vec<IndexMap<String, Vec<String>>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Modifiers {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AdditionalFields(IndexMap<String, Value>);

impl<K, V, I> From<I> for AdditionalFields
where
  K: Into<String>,
  V: Into<Value>,
  I: IntoIterator<Item = (K, V)>,
{
  fn from(iter: I) -> Self {
    let additional_fields = iter
      .into_iter()
      .map(|(k, v)| (k.into(), v.into()))
      .collect();

    Self(additional_fields)
  }
}

impl<K, V> FromIterator<(K, V)> for AdditionalFields
where
  K: Into<String>,
  V: Into<Value>,
{
  fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
    Self::from(iter)
  }
}

impl Deref for AdditionalFields {
  type Target = IndexMap<String, Value>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for AdditionalFields {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// The priority of a an ordered item. A lower number is better. The default is
/// 150.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Priority(u8);

impl Priority {
  pub const DEFAULT: Self = Self(150);
  pub const HIGH: Self = Self(50);
  pub const LOW: Self = Self(200);
  pub const MEDIUM: Self = Self(100);
}

impl Default for Priority {
  fn default() -> Self {
    Self::DEFAULT
  }
}

impl<T: Into<u8>> From<T> for Priority {
  fn from(value: T) -> Self {
    Self(value.into())
  }
}

impl Deref for Priority {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Priority {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_config_can_serialize() {
    let config: StyleConfig = Default::default();
    let json = config.to_json().unwrap();
    StyleConfig::from_json(json).unwrap();
  }

  #[test]
  fn default_config() {
    insta::assert_json_snapshot!(StyleConfig::default());
  }

  #[test]
  fn supports_additional_fields() {
    let json = include_str!("default.json");
    let config = StyleConfig::from_json(json).unwrap();
    insta::assert_json_snapshot!(config);
  }
}
