use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;
use std::ops::DerefMut;

use derivative::Derivative;
use heck::ToKebabCase;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use typed_builder::TypedBuilder;

use crate::Error;
use crate::Plugin;
use crate::Result;

/// The style configuration which can also use the builder pattern.
#[derive(Derivative, Deserialize, Serialize, TypedBuilder)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct StyleConfig {
  /// The general options.
  #[builder(default, setter(into))]
  pub options: Options,
  /// Setup the keyframes.
  #[builder(default, setter(into))]
  pub keyframes: Keyframes,
  /// CSS variables which can be reused throughout the configuration.
  #[builder(default, setter(into))]
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
  pub atoms: Atoms,
  /// A list of classes with predefined styles.
  #[builder(default, setter(into))]
  pub classes: NamedClasses,
  /// Hardcoded colors for the pallette.
  #[builder(default, setter(into))]
  pub palette: Palette,
  /// The atoms which provide the values.
  #[builder(default, setter(into))]
  pub value_sets: ValueSets,
  /// Groups which are usually used to activate a set of css variables.
  #[builder(default, setter(into))]
  pub groups: VariableGroups,
  /// The plugins which can be used to add new functionality and extend the
  /// configuration.
  #[derivative(Debug = "ignore")]
  #[serde(skip)]
  #[builder(default, setter(into))]
  pub plugins: Plugins,
  /// Support additional fields for plugins.
  #[serde(flatten, default)]
  #[builder(default, setter(into))]
  pub additional_fields: AdditionalFields,
}

impl Default for StyleConfig {
  fn default() -> Self {
    Self::builder().build()
  }
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
  pub parent_modifiers: MergeRule,
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
  #[serde(flatten, default)]
  #[builder(default, setter(into))]
  pub other: IndexMap<String, MergeRule>,
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

/// Options to use in the configuration.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Options {
  /// The character encoding used in the style sheet
  #[serde(default = "default_charset")]
  #[builder(default = default_charset(), setter(into))]
  pub charset: String,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Keyframes(Vec<Keyframe>);

impl IntoIterator for Keyframes {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = Keyframe;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for Keyframes
where
  V: Into<Keyframe>,
{
  fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
    Self(iter.into_iter().map(|value| value.into()).collect())
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
  #[builder(setter(into))]
  pub name: String,
  /// The description of the keyframe. This will be used in the vscode
  /// extension.
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The rules for the specific keyframe.
  #[serde(flatten, default)]
  #[builder(default, setter(into))]
  pub rules: StringMap,
}

/// This is a more usable version of Index<String, String> which allows for
/// easier construction and fully supports serde with renaming built in.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StringMap(IndexMap<String, String>);

impl IntoIterator for StringMap {
  type IntoIter = indexmap::map::IntoIter<String, String>;
  type Item = (String, String);

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<K, V> FromIterator<(K, V)> for StringMap
where
  K: Into<String>,
  V: Into<String>,
{
  fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
    let rules = iter
      .into_iter()
      .map(|(key, value)| (key.into(), value.into()))
      .collect();

    Self(rules)
  }
}

impl Deref for StringMap {
  type Target = IndexMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for StringMap {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// This is a more usable version of Vec<String> which allows for
/// easier construction and fully supports serde with renaming built in.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StringList(Vec<String>);

impl<V: Into<String>> From<Vec<V>> for StringList {
  fn from(value: Vec<V>) -> Self {
    Self::from_iter(value)
  }
}

impl IntoIterator for StringList {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = String;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for StringList
where
  V: Into<String>,
{
  fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
    let rules = iter.into_iter().map(|value| value.into()).collect();
    Self(rules)
  }
}

impl Deref for StringList {
  type Target = Vec<String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for StringList {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionalStringMap(IndexMap<String, Option<String>>);

impl IntoIterator for OptionalStringMap {
  type IntoIter = indexmap::map::IntoIter<String, Option<String>>;
  type Item = (String, Option<String>);

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<S> FromIterator<(S, Option<S>)> for OptionalStringMap
where
  S: Into<String>,
{
  fn from_iter<T: IntoIterator<Item = (S, Option<S>)>>(iter: T) -> Self {
    let rules = iter
      .into_iter()
      .map(|(key, value)| (key.into(), value.map(|v| v.into())))
      .collect();

    Self(rules)
  }
}

impl Deref for OptionalStringMap {
  type Target = IndexMap<String, Option<String>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for OptionalStringMap {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// Media queries can should be defined as a map of names to their css queries.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MediaQueries(Vec<Group<MediaQuery>>);

impl IntoIterator for MediaQueries {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = Group<MediaQuery>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for MediaQueries
where
  V: Into<Group<MediaQuery>>,
{
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let breakpoints = iter.into_iter().map(|value| value.into()).collect();
    Self(breakpoints)
  }
}

impl Deref for MediaQueries {
  type Target = Vec<Group<MediaQuery>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for MediaQueries {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct MediaQuery {
  /// The name of the media query.
  #[builder(setter(into))]
  pub name: String,
  /// The query to use for the media query.
  #[builder(setter(into))]
  pub query: String,
  /// A markdown description of what this media query should be used for.
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
}

/// [`Atoms`] are class that take a single value. Each
/// style that is defined as null will be provided the value from the atom
/// style.
///
/// Atoms are defined as a style rule that receives one value from the user.
///
/// Each of the style rules above maps an atomic style name to a list of CSS
/// properties that it controls. The styles rules are later connected with
/// `Atoms` which are passed to each individual style rule.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Atoms(Vec<Atom>);

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

impl Deref for Atoms {
  type Target = Vec<Atom>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Atoms {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
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
  /// Support additional fields for plugins to add extra functionality.
  #[serde(flatten, default)]
  #[builder(default, setter(into))]
  additional_fields: AdditionalFields,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum LinkedValues {
  /// The atom will be linked to colors and the settings determine how the link
  /// is made.
  Color(ColorSettings),
  /// The [`ValueSet`] names that will be used to populate the names that can be
  /// used.
  Values(ValueSetNames),
}

impl Default for LinkedValues {
  fn default() -> Self {
    Self::Values(ValueSetNames::default())
  }
}

impl<V: Into<ValueSetNames>> From<V> for LinkedValues {
  fn from(value: V) -> Self {
    Self::Values(value.into())
  }
}

impl From<ColorSettings> for LinkedValues {
  fn from(value: ColorSettings) -> Self {
    Self::Color(value)
  }
}

/// The named classes with their own defined values.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

impl Deref for NamedClasses {
  type Target = Vec<NamedClass>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for NamedClasses {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// A named class is a class with all it's values defined ahead of time.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct NamedClass {
  /// The name of the media query.
  #[builder(setter(into))]
  pub name: String,
  /// A markdown description of what this media query should be used for.
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The styles for the specific class.
  #[builder(setter(into))]
  pub styles: StringMap,
}

/// Create CSS variables from a list of atoms.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CssVariables(Vec<CssVariable>);

impl IntoIterator for CssVariables {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = CssVariable;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
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
    let css_variables = iter.into_iter().map(|v| v.into()).collect();

    Self(css_variables)
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

pub type CssVariableSelectors = StringMap;
pub type NestedCssVariableSelectors = IndexMap<String, CssVariableSelectors>;

/// This can be used to define colors and other CSS variables.
///
/// For colors you should set the `syntax` to [PropertySyntaxValue::Color].
///
/// All CSS variables are made available in the produced code.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CssVariable {
  /// A required name which is used to reference the variable.
  #[builder(setter(into))]
  pub name: String,
  /// A description of the CSS variable and what it is used for.
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The variable name. This should always start with `--`.
  #[builder(setter(into))]
  pub variable: String,
  /// The [syntax](https://developer.mozilla.org/en-US/docs/Web/CSS/@property/syntax) of the CSS variable.
  #[builder(default, setter(into))]
  pub syntax: PropertySyntax,
  /// The initial value of the CSS variable. This is required if the
  /// [PropertySyntax] is set to anything other than [PropertySyntaxValue::Any].
  #[builder(default, setter(into, strip_option))]
  pub value: Option<String>,
  /// Define the value of the CSS variables different parent selector contexts.
  /// Only parent modifiers defined in the configuration are allowed.
  #[builder(default, setter(into, strip_option))]
  pub parent_modifiers: Option<CssVariableSelectors>,
  /// Define the value of the CSS variables within different modifier contexts
  /// For example a variable can be a certain value when :hovered, :active and
  /// other inline pseudo states.
  #[builder(default, setter(into, strip_option))]
  pub modifiers: Option<CssVariableSelectors>,
  /// Define the value of the CSS variable under different nested media query
  /// situations.
  #[builder(default, setter(into, strip_option))]
  pub media_queries: Option<NestedCssVariableSelectors>,
}

impl CssVariable {
  #[inline]
  pub fn get_name(&self) -> &str {
    &self.name
  }

  #[inline]
  pub fn get_variable(&self) -> String {
    format!("var({})", self.variable)
  }

  #[inline]
  pub fn get_variable_with_fallback(&self, fallback: &str) -> String {
    format!("var({}, {})", self.name, fallback)
  }

  /// Check whether this instance of [CssVariable] is a color.
  #[inline]
  pub fn is_color(&self) -> bool {
    self.syntax.is_color()
  }
}

impl<T: Into<String>> From<T> for CssVariable {
  #[inline]
  fn from(name: T) -> Self {
    let name: String = name.into();
    let variable: String = format!("--{}", name.to_kebab_case());
    CssVariable::builder().name(name).variable(variable).build()
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

impl<V: Into<PropertySyntaxValue>> From<V> for PropertySyntax {
  fn from(value: V) -> Self {
    PropertySyntax::Value(value.into())
  }
}

impl Display for PropertySyntax {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      PropertySyntax::Value(value) => write!(f, "{value}"),
      PropertySyntax::List(values) => {
        let values = values
          .iter()
          .map(|v| v.to_string())
          .collect::<Vec<String>>()
          .join(" | ");

        write!(f, "{values}")
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
pub type Palette = StringMap;

/// This is the setup for the parent modifiers.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ParentModifiers(Vec<Modifier>);

impl IntoIterator for ParentModifiers {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = Modifier;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for ParentModifiers
where
  V: Into<Modifier>,
{
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let parent_modifiers = iter.into_iter().map(|value| value.into()).collect();

    Self(parent_modifiers)
  }
}

impl Deref for ParentModifiers {
  type Target = Vec<Modifier>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for ParentModifiers {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Modifier {
  /// The name of the parent modifier will be used to reference
  #[builder(setter(into))]
  pub name: String,
  /// The values.
  #[builder(setter(into))]
  pub values: StringList,
  /// The description for this item
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority for this item.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// Support additional fields for plugins.
  #[serde(flatten, default)]
  #[builder(default, setter(into))]
  additional_fields: AdditionalFields,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, TypedBuilder)]
pub struct Group<T> {
  #[builder(setter(into))]
  pub name: String,
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The items in this group.
  #[builder(setter(into))]
  pub items: Vec<T>,
}

impl<T> IntoIterator for Group<T> {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = T;

  fn into_iter(self) -> Self::IntoIter {
    self.items.into_iter()
  }
}

impl<T> Deref for Group<T> {
  type Target = Vec<T>;

  fn deref(&self) -> &Self::Target {
    &self.items
  }
}

impl<T> DerefMut for Group<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.items
  }
}

/// This is the setup for named modifiers.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Modifiers(Vec<Group<Modifier>>);

impl IntoIterator for Modifiers {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = Group<Modifier>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for Modifiers
where
  V: Into<Group<Modifier>>,
{
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let modifiers = iter.into_iter().map(|value| value.into()).collect();

    Self(modifiers)
  }
}

impl Deref for Modifiers {
  type Target = Vec<Group<Modifier>>;

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

impl IntoIterator for AdditionalFields {
  type IntoIter = indexmap::map::IntoIter<String, Value>;
  type Item = (String, Value);

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<K, V> FromIterator<(K, V)> for AdditionalFields
where
  K: Into<String>,
  V: Into<Value>,
{
  fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
    let additional_fields = iter
      .into_iter()
      .map(|(k, v)| (k.into(), v.into()))
      .collect();

    Self(additional_fields)
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

/// A set of values that referenced by .

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ValueSets(Vec<ValueSet>);

impl IntoIterator for ValueSets {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = ValueSet;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V> FromIterator<V> for ValueSets
where
  V: Into<ValueSet>,
{
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let atoms = iter.into_iter().map(|v| v.into()).collect();

    Self(atoms)
  }
}

impl Deref for ValueSets {
  type Target = Vec<ValueSet>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for ValueSets {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// The value atom.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ValueSet {
  #[builder(setter(into))]
  pub name: String,
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The values for the atom.
  pub values: CssValues,
  /// Support additional fields for plugins.
  #[serde(flatten, default)]
  #[builder(default, setter(into))]
  additional_fields: AdditionalFields,
}

/// Values for the value atom.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

impl Deref for CssValues {
  type Target = IndexMap<String, CssValue>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for CssValues {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// The value of an individual value atom.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CssValue {
  /// A singular value. Use this with named rules.
  Value(String),
  /// Provide an object with the values.
  Object(StringMap),
}

impl<T: Into<String>> From<T> for CssValue {
  fn from(value: T) -> Self {
    Self::Value(value.into())
  }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VariableGroups(Vec<VariableGroup>);

impl IntoIterator for VariableGroups {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = VariableGroup;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V: Into<VariableGroup>> FromIterator<V> for VariableGroups {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let groups = iter.into_iter().map(|v| v.into()).collect();

    Self(groups)
  }
}

impl Deref for VariableGroups {
  type Target = Vec<VariableGroup>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for VariableGroups {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, TypedBuilder)]
pub struct VariableGroup {
  /// The name of the group.
  #[builder(setter(into))]
  pub name: String,
  /// The description of the group.
  #[builder(default, setter(into, strip_option))]
  pub description: Option<String>,
  /// The priority of this items.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The styles for the specific class.
  #[builder(setter(into))]
  pub styles: StringMap,
}

/// A map of string values.
#[derive(Default)]
pub struct Plugins(Vec<PluginContainer>);

impl IntoIterator for Plugins {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = PluginContainer;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V: Into<PluginContainer>> FromIterator<V> for Plugins {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let plugins = iter.into_iter().map(|v| v.into()).collect();

    Self(plugins)
  }
}

#[derive(Serialize, TypedBuilder)]
pub struct PluginContainer {
  /// Get the default priority of this plugin which will be used to determine
  /// the order in which plugins are loaded. This can be overridden by the
  /// user.
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The plugin.
  #[serde(skip)]
  #[builder(setter(into))]
  pub plugin: Box<dyn Plugin>,
}

impl<P: Plugin + 'static> From<P> for PluginContainer {
  fn from(plugin: P) -> Self {
    Self {
      priority: Default::default(),
      plugin: Box::new(plugin),
    }
  }
}

impl Deref for PluginContainer {
  type Target = Box<dyn Plugin>;

  fn deref(&self) -> &Self::Target {
    &self.plugin
  }
}

impl DerefMut for PluginContainer {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.plugin
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Prioritized<T> {
  #[builder(default, setter(into))]
  pub priority: Priority,
  #[builder(setter(into))]
  pub value: T,
}

impl<T> From<T> for Prioritized<T> {
  fn from(value: T) -> Self {
    Self {
      priority: Default::default(),
      value,
    }
  }
}

impl<T> Deref for Prioritized<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl<T> DerefMut for Prioritized<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.value
  }
}

/// The color atom.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ColorSettings {
  #[builder(setter(into))]
  /// The name of the CSS Variable which is used to set the color opacity.
  pub opacity: String,
  /// When true the built in palette will also be available as values for the
  /// colors. If false only the colors defined in the `variables` will be
  /// available.
  #[builder(default, setter(into))]
  pub palette: bool,
  /// Support additional fields for plugins to add extra functionality.
  #[serde(flatten, default)]
  #[builder(default, setter(into))]
  additional_fields: AdditionalFields,
}

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
pub struct ValueSetNames(Vec<Prioritized<String>>);

impl IntoIterator for ValueSetNames {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = Prioritized<String>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V: Into<Prioritized<String>>> FromIterator<V> for ValueSetNames {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let values = iter.into_iter().map(|v| v.into()).collect();

    Self(values)
  }
}

impl Deref for ValueSetNames {
  type Target = Vec<Prioritized<String>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for ValueSetNames {
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
