use indent_write::fmt::IndentWriter;
use lazy_static::lazy_static;
use regex::Regex;
use typed_builder::TypedBuilder;

use crate::constants::INDENTATION;

lazy_static! {
  static ref ESCAPE_CSS_STRING_REGEX: Regex =
    Regex::new(r#"(#|&|~|=|>|'|:|"|!|;|,|\.|\*|\+|\||\[|\]|\(|\)|/|\^|\$)"#).unwrap();
  static ref CSS_VARIABLE_REGEX: Regex =
    Regex::new(r#"(?m)(?i)var\((--[a-zA-Z0-9_\-]+?)(?:,.*?)?\)"#).unwrap();
}

/// Escape a css string.
pub fn escape_css_string(value: &str) -> String {
  ESCAPE_CSS_STRING_REGEX
    .replace_all(value, "\\$1")
    .to_string()
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct IndentProps {
  /// The content to be indented.
  #[builder(setter(into))]
  pub content: String,

  /// The indentation style to use.
  #[builder(
    default,
    setter(doc = "Set the indentation style for the indent function.")
  )]
  pub style: IndentStyle,
}

#[derive(Clone, Copy, Debug)]
pub enum IndentStyle {
  Tab,
  Spaces(u8),
}

impl Default for IndentStyle {
  fn default() -> Self {
    Self::Spaces(2)
  }
}

/// Indent the string with the given amount of spaces.
pub fn indent(content: impl AsRef<str>, style: IndentStyle) -> String {
  let content = content.as_ref();
  let lines = content.split('\n');
  let mut result = String::new();
  let empty_line_regex = Regex::new(r"^\s*$").unwrap();

  for line in lines {
    if empty_line_regex.is_match(line) {
      result.push('\n');
      continue;
    }

    let indentation = match style {
      IndentStyle::Tab => String::from("\t"),
      IndentStyle::Spaces(spaces) => " ".repeat(spaces.into()),
    };

    result.push_str(&format!("{indentation}{line}\n"));
  }

  result.trim_end().to_string()
}

pub fn wrap_indent(content: impl AsRef<str>, level: u8) -> String {
  let mut result = content.as_ref().to_string();
  let indent_style = IndentStyle::default();

  for _ in 1..=level {
    result = indent(result, indent_style);
  }

  result
}

pub fn indent_writer<'i>() -> IndentWriter<'i, String> {
  IndentWriter::new(INDENTATION, String::new())
}

pub fn wrap_css_variable(value: impl AsRef<str>, default: Option<String>) -> String {
  let value = value.as_ref();

  if let Some(default) = default {
    return format!("var({value}, {default})");
  } else {
    format!("var({value})")
  }
}
