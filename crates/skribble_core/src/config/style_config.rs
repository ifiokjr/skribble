use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;
use std::ops::Deref;
use std::ops::DerefMut;

use derivative::Derivative;
use heck::ToKebabCase;
use indexmap::indexset;
use indexmap::IndexMap;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::Group;
use super::Keyframes;
use super::NestedStringMap;
use super::OptionalStringMap;
use super::Options;
use super::PrioritizedString;
use super::Priority;
use super::StringList;
use super::StringMap;
use crate::AnyEmptyResult;
use crate::Color;
use crate::Error;
use crate::Placeholder;
use crate::Plugin;
use crate::PluginConfig;
use crate::PluginData;
use crate::Prioritized;
use crate::Result;
use crate::RunnerConfig;

/// The style configuration which can also use the builder pattern.
#[derive(Derivative, Deserialize, Serialize, TypedBuilder)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct StyleConfig {
  /// The general options.
  #[builder(default, setter(into))]
  pub options: Options,
  /// The css layers.
  #[builder(default, setter(into))]
  pub layers: Layers,
  /// Setup the keyframes.
  #[builder(default, setter(into))]
  pub keyframes: Keyframes,
  /// CSS variables which can be reused throughout the configuration.
  #[builder(default, setter(into))]
  pub variables: CssVariables,
  /// Setup the media queries.
  #[builder(default, setter(into))]
  pub media_queries: MediaQueries,
  /// Modifiers are used to nest styles within a selector. They can be parents
  /// modifiers or child modifiers.
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
}

impl Default for StyleConfig {
  fn default() -> Self {
    Self::builder().build()
  }
}

impl StyleConfig {
  pub fn from_json(json: impl AsRef<str>) -> Result<Self> {
    let config: Self = serde_json::from_str(json.as_ref()).map_err(Error::InvalidConfig)?;
    Ok(config)
  }

  pub(crate) fn into_wrapped_config(self) -> (Options, PluginConfig, Plugins) {
    let Self {
      atoms,
      classes,
      groups,
      keyframes,
      layers,
      media_queries,
      modifiers,
      options,
      palette,
      plugins,
      value_sets,
      variables,
    } = self;

    (
      options,
      PluginConfig {
        atoms,
        classes,
        groups,
        keyframes,
        layers,
        media_queries,
        modifiers,
        palette,
        value_sets,
        variables,
      },
      plugins,
    )
  }

  pub fn to_json(&self) -> Result<String> {
    serde_json::to_string(self).map_err(Error::CouldNotSerializeConfig)
  }

  pub fn to_pretty_json(&self) -> Result<String> {
    serde_json::to_string_pretty(self).map_err(Error::CouldNotSerializeConfig)
  }
}

/// Media queries can should be defined as a map of names to their css queries.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MediaQueries(Vec<Group<MediaQuery>>);

impl MediaQueries {
  /// Extend an existing group or create a new one if it does not exist.
  pub fn extend_group(&mut self, group: impl Into<Group<MediaQuery>>) {
    let group = group.into();

    if let Some(existing_group) = self.0.iter_mut().find(|g| g.name == group.name) {
      existing_group.merge(group);
    } else {
      self.0.push(group);
    }
  }
}

impl From<Vec<Group<MediaQuery>>> for MediaQueries {
  fn from(breakpoints: Vec<Group<MediaQuery>>) -> Self {
    Self(breakpoints)
  }
}

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

impl MediaQuery {
  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();

    if self.name != other.name {
      panic!("Cannot merge media queries with different names");
    }

    if let Some(description) = other.description {
      self.description = Some(description);
    }

    if other.priority < self.priority {
      self.priority = other.priority;
    }

    self.query = other.query;
  }
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
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum LinkedValues {
  /// The atom will be linked to colors and the settings determine how the link
  /// is made.
  Color(ColorSettings),
  /// The [`ValueSet`] names that will be used to populate the names that can be
  /// used.
  Values(NameSet),
  /// The atom will be linked to all the `keyframes` that are available. This is
  /// used to generate the `animate` class name.
  Keyframes,
}

impl LinkedValues {
  pub fn get_names_from_config(&self, config: &RunnerConfig) -> IndexSet<String> {
    match self {
      Self::Color(ref color_settings) => {
        let mut names = indexset! {};

        for (name, variable) in config.css_variables.iter() {
          if variable.is_color() {
            names.insert(name.to_owned());
          }
        }

        if !color_settings.ignore_palette {
          names.extend(config.palette.keys().cloned());
        }

        names
      }
      Self::Values(ref value_set) => {
        let mut names = indexset! {};
        for value in value_set.iter() {
          if let Some(set) = config.value_sets.get(&value.value) {
            names.extend(set.values.keys().cloned());
          }
        }

        names
      }
      Self::Keyframes => config.keyframes.keys().cloned().collect(),
    }
  }

  pub fn merge(&mut self, other: Self) {
    match self {
      Self::Color(color_settings) => {
        if let Self::Color(other_color_settings) = other {
          color_settings.merge(other_color_settings);
        }
      }
      Self::Values(value_set) => {
        if let Self::Values(other_value_set) = other {
          value_set.merge(other_value_set);
        }
      }
      Self::Keyframes => {
        *self = other;
      }
    }
  }

  pub fn write_css_properties(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    atom: &Atom,
    name: impl AsRef<str>,
  ) -> AnyEmptyResult {
    match self {
      Self::Values(ref set) => {
        for Prioritized { value: key, .. } in set.iter() {
          if let Some(css_value) = config
            .value_sets
            .get(key)
            .and_then(|value_set| value_set.values.get(name.as_ref()))
          {
            css_value.write_css(writer, config, atom)?;
            break;
          }
        }
      }
      Self::Color(ref settings) => {
        settings.write_css(writer, config, atom, name)?;
      }
      Self::Keyframes => {}
    }

    Ok(())
  }
}

impl Default for LinkedValues {
  fn default() -> Self {
    Self::Values(NameSet::default())
  }
}

impl<V: Into<NameSet>> From<V> for LinkedValues {
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
pub type NestedCssVariableSelectors = NestedStringMap;

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
  /// The variable token. This should always start with `--`.
  #[builder(setter(into))]
  pub variable: String,
  /// The [syntax](https://developer.mozilla.org/en-US/docs/Web/CSS/@property/syntax) of the CSS variable.
  #[builder(default, setter(into))]
  pub syntax: PropertySyntax,
  /// The initial value of the CSS variable. This is required if the
  /// [PropertySyntax] is set to anything other than [PropertySyntaxValue::Any].
  #[builder(default, setter(into, strip_option))]
  pub value: Option<String>,
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
  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();

    if self.name != other.name {
      panic!("Cannot merge CSS variables with different names");
    }

    if let Some(description) = other.description {
      self.description = Some(description);
    }

    self.variable = other.variable;
    self.syntax = other.syntax;

    if let Some(value) = other.value {
      self.value = Some(value);
    }

    if let Some(modifiers) = other.modifiers {
      match self.modifiers {
        Some(ref mut original_modifiers) => {
          original_modifiers.extend(modifiers);
        }

        None => self.modifiers = Some(modifiers),
      };
    }

    if let Some(media_queries) = other.media_queries {
      match self.media_queries {
        Some(ref mut original_media_queries) => {
          original_media_queries.extend(media_queries);
        }
        None => self.media_queries = Some(media_queries),
      };
    }
  }

  #[inline]
  pub fn get_variable(&self, prefix: impl AsRef<str>) -> String {
    let prefix = prefix.as_ref();
    let replacement = format!("--{prefix}-");
    self.variable.as_str().replacen("--", &replacement, 1)
  }

  pub fn get_opacity_variable(&self, prefix: impl AsRef<str>) -> String {
    let prefix = prefix.as_ref();
    let replacement = format!("--{prefix}-opacity-");
    self.variable.as_str().replacen("--", &replacement, 1)
  }

  pub fn get_default_opacity(&self) -> f32 {
    self
      .value
      .as_ref()
      .and_then(|value| value.parse::<Color>().ok())
      .map(|color| color.alpha())
      .unwrap_or(1.0)
  }

  pub fn get_property_rule(&self, config: &RunnerConfig) -> Result<String> {
    let mut property_rule = vec![];
    let options = config.options();
    let prefix = &options.variable_prefix;
    let syntax = &self.syntax;
    let _color_format = &options.color_format;
    let variable_name = self.get_variable(prefix);
    let initial_value = if self.is_color() {
      let opacity_variable = self.get_opacity_variable(prefix);
      let alpha = self.get_default_opacity();
      property_rule.push(format!(
        "@property {opacity_variable} {{\n  syntax: \"<number>\";\n  inherits: false;\n  \
         initial-value: {alpha};\n}}"
      ));
      options
        .color_format
        .get_color_value_with_opacity(config, self)?
    } else {
      let default_initial_value = "/* */".into();
      Placeholder::normalize(
        self.value.as_ref().unwrap_or(&default_initial_value),
        config,
      )
    };

    property_rule.push(format!(
      "@property {variable_name} {{\n  syntax: {syntax};\n  inherits: false;\n  initial-value: \
       {initial_value};\n}}"
    ));

    Ok(property_rule.join("\n"))
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
}

impl Modifier {
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

    self.values.extend(other.values);
  }
}

/// This is the setup for named modifiers.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Modifiers(Vec<Group<Modifier>>);

impl Modifiers {
  /// Extend an existing group or create a new one if it does not exist.
  pub fn extend_group(&mut self, group: impl Into<Group<Modifier>>) {
    let group = group.into();

    if let Some(existing_group) = self.0.iter_mut().find(|g| g.name == group.name) {
      existing_group.merge(group);
    } else {
      self.0.push(group);
    }
  }
}

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

/// A set of values that referenced by .

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ValueSets(Vec<ValueSet>);

impl From<Vec<ValueSet>> for ValueSets {
  fn from(value: Vec<ValueSet>) -> Self {
    Self(value)
  }
}

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
  /// The values for this set.
  #[builder(setter(into))]
  pub values: CssValues,
}

impl ValueSet {
  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();

    if self.name != other.name {
      panic!("Cannot merge groups with different names");
    }

    if let Some(description) = other.description {
      self.description = Some(description);
    }

    if other.priority < self.priority {
      self.priority = other.priority;
    }

    self.values.extend(other.values);
  }
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

impl<K: Into<String>, V: Into<CssValue>> From<IndexMap<K, V>> for CssValues {
  fn from(values: IndexMap<K, V>) -> Self {
    Self::from_iter(values)
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

impl CssValue {
  pub fn write_css(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    atom: &Atom,
  ) -> AnyEmptyResult {
    match self {
      Self::Value(value) => {
        let value = Placeholder::normalize(value, config);

        for (property, css_value) in atom.styles.iter() {
          let property = Placeholder::normalize(property, config);
          let css_value = css_value
            .as_ref()
            .map(|value| Placeholder::normalize(value, config))
            .unwrap_or_else(|| value.clone());

          writeln!(writer, "{property}: {css_value};")?;
        }
      }
      Self::Object(map) => {
        for (property, css_value) in map.iter() {
          let property = Placeholder::normalize(property, config);
          let css_value = Placeholder::normalize(css_value, config);
          writeln!(writer, "{property}: {css_value};")?;
        }
      }
    }

    Ok(())
  }
}

impl From<&str> for CssValue {
  fn from(value: &str) -> Self {
    Self::Value(value.into())
  }
}

impl From<String> for CssValue {
  fn from(value: String) -> Self {
    Self::Value(value)
  }
}

impl<V: Into<StringMap>> From<V> for CssValue {
  fn from(map: V) -> Self {
    Self::Object(map.into())
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

impl VariableGroup {
  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();

    if self.name != other.name {
      panic!("Cannot merge groups with different names");
    }

    if let Some(description) = other.description {
      self.description = Some(description);
    }

    if other.priority < self.priority {
      self.priority = other.priority;
    }

    self.styles.extend(other.styles);
  }
}

/// A map of string values.
#[derive(Default)]
pub struct Plugins(Vec<PluginContainer>);

pub(crate) type BoxedPlugin = Box<dyn Plugin>;
pub(crate) struct WrappedPlugin {
  plugin: BoxedPlugin,
  data: PluginData,
}

impl WrappedPlugin {
  pub fn data(&self) -> &PluginData {
    &self.data
  }
}

impl Deref for WrappedPlugin {
  type Target = BoxedPlugin;

  fn deref(&self) -> &Self::Target {
    &self.plugin
  }
}

impl DerefMut for WrappedPlugin {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.plugin
  }
}

impl Plugins {
  /// Sort the plugins by priority and deduplicate them.
  pub(crate) fn sort_by_priority(&mut self) {
    self.0.sort_by(|a, z| a.priority.cmp(&z.priority));
  }

  /// Remove the the container plugins.
  pub(crate) fn extract_plugins(self) -> Vec<WrappedPlugin> {
    let mut plugins = vec![];

    for container in self.into_iter() {
      plugins.push(container.extract_plugin());
    }

    plugins
  }
}

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

impl From<Vec<PluginContainer>> for Plugins {
  fn from(plugins: Vec<PluginContainer>) -> Self {
    Self::from_iter(plugins)
  }
}

impl Deref for Plugins {
  type Target = Vec<PluginContainer>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Plugins {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Serialize, TypedBuilder)]
pub struct PluginContainer {
  /// Get the default priority of this plugin which will be used to determine
  /// the order in which plugins are loaded. This can be overridden by the
  /// user.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub priority: Priority,
  /// The plugin.
  #[serde(skip)]
  #[builder(setter(transform = |p: impl Plugin + 'static| Box::new(p) as Box<dyn Plugin>))]
  plugin: BoxedPlugin,
}

impl PluginContainer {
  /// Get the plugin.
  pub(crate) fn extract_plugin(self) -> WrappedPlugin {
    WrappedPlugin {
      data: self.plugin.get_data(),
      plugin: self.plugin,
    }
  }
}

impl<P: Plugin + 'static> From<P> for PluginContainer {
  fn from(plugin: P) -> Self {
    Self {
      priority: Default::default(),
      plugin: Box::new(plugin),
    }
  }
}

/// The color atom.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ColorSettings {
  #[builder(setter(into))]
  /// The name of the CSS Variable which is used to set the color opacity.
  pub opacity: String,
  /// When set to true the color palette will not be available for the atom
  /// property which is using colors.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub ignore_palette: bool,
}

impl ColorSettings {
  pub fn write_css(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    atom: &Atom,
    color_name: impl AsRef<str>,
  ) -> AnyEmptyResult {
    let prefix = &config.options().variable_prefix;

    for (name, variable) in config.css_variables.iter() {
      if !variable.is_color() || color_name.as_ref() != name {
        continue;
      }

      let variable_name = variable.get_variable(prefix);
      let opacity_variable = Placeholder::normalize(variable.get_opacity_variable(prefix), config);
      let default_opacity = variable.get_default_opacity();
      writeln!(writer, "{opacity_variable}: {default_opacity};")?;

      for (property, css_value) in atom.styles.iter() {
        let property = Placeholder::normalize(property, config);
        let css_value = css_value
          .as_ref()
          .map(|value| Placeholder::normalize_with_value(value, &variable_name, config))
          .unwrap_or_else(|| variable_name.clone());

        writeln!(writer, "{}: {};", property, css_value)?;
      }

      break;
    }

    Ok(())
  }

  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();
    self.opacity = other.opacity;
    self.ignore_palette = other.ignore_palette;
  }
}

impl<S: Into<String>> From<S> for ColorSettings {
  fn from(opacity: S) -> Self {
    Self::builder().opacity(opacity).build()
  }
}

pub type Layers = NameSet;

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
pub struct NameSet(IndexSet<PrioritizedString>);

impl NameSet {
  pub fn sort_by_priority(&mut self) -> &Self {
    self.sort_by(|a, z| a.priority.cmp(&z.priority));
    self
  }

  pub fn merge(&mut self, other: impl Into<Self>) {
    self.extend(other.into());
    self.sort_by_priority();
  }
}

impl IntoIterator for NameSet {
  type IntoIter = indexmap::set::IntoIter<Self::Item>;
  type Item = PrioritizedString;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<V: Into<PrioritizedString>> FromIterator<V> for NameSet {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = V>,
  {
    let list = iter.into_iter().map(|v| v.into()).collect();

    Self(list)
  }
}

impl<I: Into<PrioritizedString>> From<Vec<I>> for NameSet {
  fn from(list: Vec<I>) -> Self {
    Self::from_iter(list)
  }
}

impl<I: Into<PrioritizedString>> From<IndexSet<I>> for NameSet {
  fn from(list: IndexSet<I>) -> Self {
    Self::from_iter(list)
  }
}

impl Deref for NameSet {
  type Target = IndexSet<PrioritizedString>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for NameSet {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[cfg(test)]
mod tests {}
