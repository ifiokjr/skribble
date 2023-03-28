use lazy_static::lazy_static;
use regex::Captures;
use regex::Regex;

use crate::RunnerConfig;

pub const INDENTATION: &str = "  ";
pub const ROOT_SELECTOR: &str = ":root";

lazy_static! {
  static ref CSS_VARIABLE_REGEX: Regex =
    Regex::new(format!("(?m){}", Placeholder::variable("(?P<name>\\w[\\w\\d]+)")).as_str())
      .unwrap();
  static ref MEDIA_QUERY_REGEX: Regex =
    Regex::new(format!("(?m){}", Placeholder::media_query("(?P<name>\\w[\\w\\d]+)")).as_str())
      .unwrap();
  static ref MODIFIER_REGEX: Regex =
    Regex::new(format!("(?m){}", Placeholder::modifier("(?P<name>\\w[\\w\\d]+)")).as_str())
      .unwrap();
  static ref PALETTE_REGEX: Regex =
    Regex::new(format!("(?m){}", Placeholder::palette("(?P<name>\\w[\\w\\d]+)")).as_str()).unwrap();
  static ref VALUE: Regex = Regex::new(format!("(?m){}", Placeholder::value()).as_str()).unwrap();
}

pub struct Placeholder;

impl Placeholder {
  pub const CSS_VARIABLE: &str = "CSS_VARIABLE";
  pub const MEDIA_QUERY: &str = "MEDIA_QUERY";
  pub const MODIFIER: &str = "MODIFIER";
  pub const PALETTE: &str = "PALETTE";
  pub const VALUE: &str = "VALUE";

  pub fn create(namespace: &str, name: impl AsRef<str>) -> String {
    let name = name.as_ref();
    format!("__:{namespace}::{name}:__")
  }

  /// Generate a placeholder for the variable by using the name. This inserts
  /// some text which will be replaced by the actual variable name when the code
  /// is generated.
  pub fn variable(name: impl AsRef<str>) -> String {
    Self::create(Self::CSS_VARIABLE, name)
  }

  pub fn normalize(content: impl AsRef<str>, config: &RunnerConfig) -> String {
    let content = CSS_VARIABLE_REGEX.replace_all(content.as_ref(), |caps: &Captures| {
      // value for an invalid match
      let invalid_regex = format!(
        "--{}-invalid-css-variable",
        config.options().variable_prefix
      );

      // get the name from the capture group
      let Some(name) = caps.name("name") else {
        return invalid_regex;
      };

      let name = name.as_str();

      let Some(name) = config.css_variables.get(name) else {
        return invalid_regex;
      };

      name.get_variable(&config.options().variable_prefix)
    });

    let content = PALETTE_REGEX.replace_all(&content, |caps: &Captures| {
      // value for an invalid match
      let default_value = "#000000".into();

      // get the name from the capture group
      let Some(name) = caps.name("name") else {
        return default_value;
      };

      let name = name.as_str();

      let Some(value) = config.palette.get(name) else {
        return default_value;
      };

      value.to_owned()
    });

    let content = MODIFIER_REGEX.replace_all(&content, |caps: &Captures| {
      let Some(name) = caps.name("name") else {
        return String::new();
      };

      let name = name.as_str();

      let Some(group) = config.modifiers.get(name) else {
        return String::new();
      };

      let Some(modifier) = group.get(name) else {
        return String::new();
      };

      modifier.values.join(", ")
    });

    content.to_string()
  }

  /// Replaces all the value placeholders with the given value.
  pub fn normalize_with_value(
    content: impl AsRef<str>,
    value: impl AsRef<str>,
    config: &RunnerConfig,
  ) -> String {
    let content = VALUE.replace_all(content.as_ref(), value.as_ref());

    Self::normalize(content, config)
  }

  /// Generate a placeholder for the palette color by using the name. This
  /// inserts some text which will be replaced by the actual palette color
  /// when the code is generated.
  pub fn palette(name: impl AsRef<str>) -> String {
    Self::create(Self::PALETTE, name)
  }

  pub fn media_query(name: impl AsRef<str>) -> String {
    Self::create(Self::MEDIA_QUERY, name)
  }

  pub fn modifier(name: impl AsRef<str>) -> String {
    Self::create(Self::MODIFIER, name)
  }

  pub fn value() -> String {
    Self::create(Self::VALUE, "0")
  }
}
