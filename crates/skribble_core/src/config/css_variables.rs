use std::fmt::Write;

use derive_more::Deref;
use derive_more::DerefMut;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use skribble_color::Color;
use typed_builder::TypedBuilder;

use super::NestedStringMap;
use super::Options;
use super::Priority;
use super::PropertySyntax;
use super::StringMap;
use crate::format_css_string;
use crate::indent_writer;
use crate::wrap_css_variable;
use crate::AnyEmptyResult;
use crate::Placeholder;
use crate::RunnerConfig;

/// Create CSS variables from a list of atoms.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct CssVariables(Vec<CssVariable>);

impl<T: Into<CssVariable>> From<Vec<T>> for CssVariables {
  fn from(variables: Vec<T>) -> Self {
    Self::from_iter(variables)
  }
}

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
  /// Define the value of the CSS variable under different nested media query
  /// situations.
  ///
  /// CSS Variable are not dynamic when nested. For example the variable
  /// `--color: hsl(0, 0%, 0%, var(--color-opacity))` will not change when the
  /// variable for `--color-opacity` changes. Bear this in mind when creating
  /// these variables.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub media_queries: NestedCssVariableSelectors,
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

    self.media_queries.extend(other.media_queries);
  }

  #[inline]
  pub fn get_variable(&self, options: &Options) -> String {
    let prefix = &options.variable_prefix;
    let replacement = format!("--{prefix}-");
    self.variable.as_str().replacen("--", &replacement, 1)
  }

  pub fn get_wrapped_variable(&self, options: &Options, default: Option<String>) -> String {
    let variable = self.get_variable(options);
    wrap_css_variable(variable, default)
  }

  pub fn get_opacity_variable(&self, options: &Options) -> String {
    let prefix = &options.variable_prefix;
    let opacity_prefix = &options.opacity_prefix;
    let replacement = format!("--{prefix}-{opacity_prefix}-");
    self.variable.as_str().replacen("--", &replacement, 1)
  }

  pub fn get_wrapped_opacity_variable(&self, options: &Options, default: Option<String>) -> String {
    let variable = self.get_opacity_variable(options);
    wrap_css_variable(variable, default)
  }

  pub fn get_color_variable(&self, options: &Options) -> String {
    let prefix = &options.variable_prefix;
    let color_prefix = &options.color_prefix;
    let replacement = format!("--{prefix}-{color_prefix}-");
    self.variable.as_str().replacen("--", &replacement, 1)
  }

  pub fn get_wrapped_color_variable(&self, options: &Options, default: Option<String>) -> String {
    let variable = self.get_color_variable(options);
    wrap_css_variable(variable, default)
  }

  pub fn get_default_opacity(&self, value: Option<&String>) -> f32 {
    value
      .or(self.value.as_ref())
      .and_then(|value| value.parse::<Color>().ok())
      .map(|color| color.alpha())
      .unwrap_or(1.0)
  }

  pub fn write_property_rule(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    with_opacity: bool,
  ) -> AnyEmptyResult {
    let options = config.options();
    let syntax = &self.syntax;
    let _color_format = &options.color_format;
    let variable_name = self.get_variable(options);
    let inherits = !self.media_queries.is_empty() || self.value.is_some();
    let initial_value = if self.is_color() {
      let normalized_color = options
        .color_format
        .get_normalized_color(config, self, None)?
        .to_string();
      if with_opacity {
        let opacity_variable = self.get_opacity_variable(options);
        let alpha = self.get_default_opacity(None);

        writeln!(writer, "@property {opacity_variable} {{")?;
        let mut indented_writer = indent_writer();
        writeln!(indented_writer, "syntax: \"<number>\";")?;
        writeln!(indented_writer, "inherits: {inherits};")?;
        writeln!(indented_writer, "initial-value: {alpha};")?;
        write!(writer, "{}", indented_writer.get_ref())?;
        writeln!(writer, "}}")?;

        let color_variable = self.get_color_variable(options);
        let color_parts = options.color_format.get_inner_color(&normalized_color)?;

        writeln!(writer, "@property {color_variable} {{")?;
        let mut indented_writer = indent_writer();
        writeln!(indented_writer, "syntax: \"*\";")?;
        writeln!(indented_writer, "inherits: {inherits};")?;
        writeln!(indented_writer, "initial-value: {color_parts};")?;
        write!(writer, "{}", indented_writer.get_ref())?;
        writeln!(writer, "}}")?;
      }

      normalized_color
    } else {
      let default_initial_value = "/* */".into();
      Placeholder::normalize(
        self.value.as_ref().unwrap_or(&default_initial_value),
        config,
      )
    };

    writeln!(writer, "@property {variable_name} {{")?;
    let mut indented_writer = indent_writer();
    writeln!(indented_writer, "syntax: \"{syntax}\";")?;
    writeln!(indented_writer, "inherits: {inherits};")?;
    writeln!(indented_writer, "initial-value: {initial_value};")?;
    write!(writer, "{}", indented_writer.get_ref())?;
    writeln!(writer, "}}")?;

    Ok(())
  }

  /// Check whether this instance of [CssVariable] is a color.
  #[inline]
  pub fn is_color(&self) -> bool {
    self.syntax.is_color()
  }

  pub fn extend_media_query_dictionary(
    &self,
    config: &RunnerConfig,
    dictionary: &mut IndexMap<Option<String>, StringMap>,
  ) -> AnyEmptyResult {
    let options = config.options();
    let variable_name = self.get_variable(options);

    if self.media_queries.is_empty() {
      if let Some(ref initial_value) = self.value {
        let selector_name = ":root".to_string();
        self.extend_dictionary_for_selector(
          config,
          dictionary,
          &variable_name,
          &None,
          &selector_name,
          initial_value,
        )?;
      };
    }

    for (query, selector_map) in self.media_queries.iter() {
      let query = if query.is_empty() {
        None
      } else {
        Some(Placeholder::normalize_media_query(query, config))
      };

      if query.is_none() {
        if let Some(ref initial_value) = self.value {
          let selector_name = ":root".to_string();
          self.extend_dictionary_for_selector(
            config,
            dictionary,
            &variable_name,
            &query,
            &selector_name,
            initial_value,
          )?;
        };
      }

      for (selector_name, variable_value) in selector_map.iter() {
        self.extend_dictionary_for_selector(
          config,
          dictionary,
          &variable_name,
          &query,
          selector_name,
          variable_value,
        )?;
      }
    }

    Ok(())
  }

  fn extend_dictionary_for_selector(
    &self,
    config: &RunnerConfig,
    dictionary: &mut IndexMap<Option<String>, StringMap>,
    variable_name: &String,
    query: &Option<String>,
    selector_name: &String,
    variable_value: &String,
  ) -> AnyEmptyResult {
    let selector = if selector_name.is_empty() {
      ":root".into()
    } else {
      Placeholder::normalize(selector_name, config)
    };
    match dictionary.get_mut(query) {
      Some(map) => {
        match map.get_mut(&selector) {
          Some(writer) => {
            self.write_media_query_css(writer, config, variable_name, variable_value)?;
          }
          None => {
            let mut writer = String::new();
            self.write_media_query_css(&mut writer, config, variable_name, variable_value)?;
            map.insert(selector, writer);
          }
        }
      }
      None => {
        let mut map = StringMap::default();
        let mut writer = String::new();

        self.write_media_query_css(&mut writer, config, variable_name, variable_value)?;
        map.insert(selector, writer);
        dictionary.insert(query.clone(), map);
      }
    };
    Ok(())
  }

  fn write_media_query_css(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    variable_name: &String,
    variable_value: &String,
  ) -> AnyEmptyResult {
    if self.is_color() {
      let options = config.options();
      let opacity_variable = self.get_opacity_variable(options);
      let opacity_value = self.get_default_opacity(Some(variable_value));
      let color_variable = self.get_color_variable(options);
      let inner_color_value = options.color_format.get_inner_color(
        options
          .color_format
          .get_normalized_color(config, self, Some(variable_value))?
          .to_string(),
      )?;
      let variable_value = options
        .color_format
        .get_color_with_parts_and_opacity(self, options);
      writeln!(writer, "{opacity_variable}: {opacity_value};")?;
      writeln!(writer, "{color_variable}: {inner_color_value};")?;
      writeln!(writer, "{variable_name}: {variable_value};")?;
    } else {
      writeln!(writer, "{variable_name}: {variable_value};")?;
    }
    Ok(())
  }
}

impl<T: Into<String>> From<T> for CssVariable {
  #[inline]
  fn from(name: T) -> Self {
    let name: String = name.into();
    let variable: String = format!("--{}", format_css_string(&name));
    CssVariable::builder().name(name).variable(variable).build()
  }
}
