use std::str::FromStr;

use colorsys::ColorAlpha;
use colorsys::Hsl;
use colorsys::Rgb;
use indexmap::IndexSet;
use lazy_static::lazy_static;
use regex::Regex;
use typed_builder::TypedBuilder;

use crate::constants::INDENTATION;
use crate::Palette;

lazy_static! {
  static ref ESCAPE_CSS_STRING_REGEX: Regex =
    Regex::new(r#"(#|&|~|=|>|'|:|"|!|;|,|\.|\*|\+|\||\[|\]|\(|\)|/|\^|\$)"#).unwrap();
  static ref CSS_VARIABLE_REGEX: Regex =
    Regex::new(r#"(?m)(?i)var\((--[a-zA-Z0-9_\-]+?)(?:,.*?)?\)"#).unwrap();
}

/// Retrieve the css variables from the provided css value.
pub fn get_css_variables_from_string(value: &str) -> IndexSet<String> {
  CSS_VARIABLE_REGEX
    .captures_iter(value)
    .map(|capture| capture[1].to_owned())
    .collect()
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

#[derive(Debug, Clone)]
pub enum IndentStyle {
  Tab,
  Spaces(u8),
}

impl Default for IndentStyle {
  fn default() -> Self {
    Self::Spaces(INDENTATION)
  }
}

/// Indent the string with the given amount of spaces.
pub fn indent(props: IndentProps) -> String {
  let IndentProps { content, style } = props;
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

/// Convert the color to a valid css value with the opacity set to the provided
/// css variable.
pub fn convert_css_value_to_color(
  original: impl AsRef<str>,
  palette: &Palette,
  opacity: impl AsRef<str>,
) -> String {
  // Create a copy of the string value to search against.
  let mut string_value = original.as_ref().to_string();

  if let Some(derived_value) = palette.get(&string_value) {
    string_value = derived_value.clone();
  }

  get_rgba_color_from_string(&string_value, opacity)
}

/// Will return the string unchanged if the color provided is not valid.
pub fn get_rgba_color_from_string(value: impl AsRef<str>, opacity: impl AsRef<str>) -> String {
  let mut string_value = value.as_ref().to_string();

  let rgb = if let Some(stripped) = string_value.strip_prefix('#') {
    Rgb::from_hex_str(stripped).ok()
  } else if string_value.starts_with("rgb") {
    Rgb::from_str(&string_value).ok()
  } else if string_value.starts_with("hsl") {
    if let Ok(hsl) = Hsl::from_str(&string_value) {
      Some(Rgb::from(hsl))
    } else {
      None
    }
  } else {
    None
  };

  let wrapped_opacity = wrap_css_variable(opacity);

  if let Some(rgb) = rgb {
    let alpha = if rgb.alpha() < 1.0 {
      format!("calc({} * {})", rgb.alpha(), wrapped_opacity)
    } else {
      wrapped_opacity
    };

    string_value = format!(
      "rgba({}, {}, {}, {})",
      rgb.red(),
      rgb.green(),
      rgb.blue(),
      alpha
    );
  }

  string_value
}

/// Wrap the opacity value in `var()` if not already done in the config.
pub fn wrap_css_variable(value: impl AsRef<str>) -> String {
  let value = value.as_ref();
  if value.starts_with("var(") && value.ends_with(')') {
    value.to_owned()
  } else {
    format!("var({value})")
  }
}
