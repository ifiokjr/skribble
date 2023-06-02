use std::fmt::Write;

use derive_more::Deref;
use derive_more::DerefMut;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use skribble_color::palette::Hsla;
use skribble_color::HslaCss;
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
  #[builder(default, setter(into))]
  pub value: String,
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
    self.value = other.value;
    self.media_queries.extend(other.media_queries);
  }

  #[inline]
  pub fn get_variable(&self, options: &Options) -> String {
    let prefix = &options.variable_prefix;
    let replacement = format!("--{prefix}-");
    self.variable.as_str().replacen("--", &replacement, 1)
  }

  pub fn get_wrapped_variable(&self, options: &Options) -> String {
    let variable = self.get_variable(options);
    wrap_css_variable(
      variable,
      if self.value.is_empty() {
        None
      } else {
        Some(self.value.clone())
      },
    )
  }

  pub fn hsla_color_variable(&self, options: &Options) -> HslaColorVariable {
    HslaColorVariable::new(&self.variable, options)
  }

  pub fn write_property_rule(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    with_parts: bool,
  ) -> AnyEmptyResult {
    let options = config.options();
    let syntax = &self.syntax;
    let variable_name = self.get_variable(options);
    let mut value = Placeholder::normalize(&self.value, config);

    if value.is_empty() {
      return Ok(());
    }

    if self.is_color()  {
      value = options.color_format.get_color(value)?.to_string();
      let hsla = options.color_format.get_hsla(&value)?;

      if with_parts {
        let HslaColorVariable { h, s, l, a, .. } = self.hsla_color_variable(options);
        let hsla_css = HslaCss::new(&hsla);
        let hue = hsla_css.hue();
        let saturation = hsla_css.saturation();
        let lightness = hsla_css.lightness();
        let alpha = hsla_css.alpha();

        writeln!(writer, "@property {h} {{")?;
        let mut indented_writer = indent_writer();
        writeln!(indented_writer, "syntax: \"<number> | <angle>\";")?;
        writeln!(indented_writer, "inherits: true;")?;
        writeln!(indented_writer, "initial-value: {hue};")?;
        write!(writer, "{}", indented_writer.get_ref())?;
        writeln!(writer, "}}")?;

        writeln!(writer, "@property {s} {{")?;
        let mut indented_writer = indent_writer();
        writeln!(indented_writer, "syntax: \"<percentage>\";")?;
        writeln!(indented_writer, "inherits: true;")?;
        writeln!(indented_writer, "initial-value: {saturation};")?;
        write!(writer, "{}", indented_writer.get_ref())?;
        writeln!(writer, "}}")?;

        writeln!(writer, "@property {l} {{")?;
        let mut indented_writer = indent_writer();
        writeln!(indented_writer, "syntax: \"<percentage>\";")?;
        writeln!(indented_writer, "inherits: true;")?;
        writeln!(indented_writer, "initial-value: {lightness};")?;
        write!(writer, "{}", indented_writer.get_ref())?;
        writeln!(writer, "}}")?;

        writeln!(writer, "@property {a} {{")?;
        let mut indented_writer = indent_writer();
        writeln!(indented_writer, "syntax: \"<number> | <percentage>\";")?;
        writeln!(indented_writer, "inherits: true;")?;
        writeln!(indented_writer, "initial-value: {alpha};")?;
        write!(writer, "{}", indented_writer.get_ref())?;
        writeln!(writer, "}}")?;
      }
    }

    writeln!(writer, "@property {variable_name} {{")?;
    let mut indented_writer = indent_writer();
    writeln!(indented_writer, "syntax: \"{syntax}\";")?;
    writeln!(indented_writer, "inherits: true;")?;
    writeln!(indented_writer, "initial-value: {value};")?;
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
    if self.media_queries.is_empty() {
      let selector_name = ":root".to_string();
      self.extend_dictionary_for_selector(
        config,
        dictionary,
        &None,
        &selector_name,
        &self.value,
      )?;
    }

    for (query, selector_map) in self.media_queries.iter() {
      let query = if query.is_empty() {
        None
      } else {
        Some(Placeholder::normalize_media_query(query, config))
      };

      if query.is_none() {
        let selector_name = ":root".to_string();
        self.extend_dictionary_for_selector(
          config,
          dictionary,
          &query,
          &selector_name,
          &self.value,
        )?;
      }

      for (selector_name, variable_value) in selector_map.iter() {
        self.extend_dictionary_for_selector(
          config,
          dictionary,
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
            self.write_media_query_css(writer, config, variable_value)?;
          }
          None => {
            let mut writer = String::new();
            self.write_media_query_css(&mut writer, config, variable_value)?;
            map.insert(selector, writer);
          }
        }
      }
      None => {
        let mut map = StringMap::default();
        let mut writer = String::new();

        self.write_media_query_css(&mut writer, config, variable_value)?;
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
    variable_value: &String,
  ) -> AnyEmptyResult {
    let variable_name = &self.get_variable(config.options());

    if self.is_color() {
      let options = config.options();
      let hsla = options.color_format.get_hsla(&self.value)?;
      let HslaColorVariable { h, s, l, a, .. } = self.hsla_color_variable(options);
      let hsla_css = HslaCss::new(&hsla);
      let hue = hsla_css.hue();
      let saturation = hsla_css.saturation();
      let lightness = hsla_css.lightness();
      let alpha = hsla_css.alpha();
      let variable_value = options.color_format.get_color(variable_value)?;

      writeln!(writer, "{h}: {hue};")?;
      writeln!(writer, "{s}: {saturation};")?;
      writeln!(writer, "{l}: {lightness};")?;
      writeln!(writer, "{a}: {alpha};")?;
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

pub struct HslaColorVariable {
  color: Hsla,
  pub h: String,
  pub s: String,
  pub l: String,
  pub a: String,
}

impl HslaColorVariable {
  pub fn new(variable: impl AsRef<str>, options: &Options) -> Self {
    let variable = variable.as_ref().replacen("--", "", 1);
    let prefix = &options.variable_prefix;
    let color = Hsla::new(0.0, 0.0, 0.0, 0.0);

    Self {
      color,
      h: format!("--{prefix}-{variable}-hue"),
      s: format!("--{prefix}-{variable}-saturation"),
      l: format!("--{prefix}-{variable}-lightness"),
      a: format!("--{prefix}-{variable}-alpha"),
    }
  }

  pub fn h_wrapped(&self) -> String {
    wrap_css_variable(&self.h, None)
  }

  pub fn s_wrapped(&self) -> String {
    wrap_css_variable(&self.s, None)
  }

  pub fn l_wrapped(&self) -> String {
    wrap_css_variable(&self.l, None)
  }

  pub fn a_wrapped(&self) -> String {
    wrap_css_variable(&self.a, None)
  }

  pub fn hsla_css(&self) -> HslaCss<'_> {
    HslaCss::builder()
      .hsla(&self.color)
      .h(self.h_wrapped())
      .s(self.s_wrapped())
      .l(self.l_wrapped())
      .a(self.a_wrapped())
      .build()
  }

  pub fn wrapped_transparent(&self) -> String {
    let mut hsla_css = self.hsla_css();
    hsla_css.a = Some("0".into());
    hsla_css.to_string()
  }
}
