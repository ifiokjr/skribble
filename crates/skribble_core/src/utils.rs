use std::path::Path;


use globset::Glob;
use globset::GlobSet;
use globset::GlobSetBuilder;
use lazy_static::lazy_static;
use regex::Regex;
use typed_builder::TypedBuilder;
use walkdir::DirEntry;
use walkdir::WalkDir;

use crate::constants::INDENTATION;
use crate::AnyResult;


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
    Self::Spaces(INDENTATION)
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

/// Wrap the opacity value in `var()` if not already done in the config.
pub fn wrap_css_variable(value: impl AsRef<str>) -> String {
  let value = value.as_ref();
  if value.starts_with("var(") && value.ends_with(')') {
    value.to_owned()
  } else {
    format!("var({value})")
  }
}

fn is_match(entry: &DirEntry, include_set: &GlobSet, exclude_set: &GlobSet) -> bool {
  entry
    .path()
    .to_str()
    .map(|file_name| !exclude_set.is_match(file_name) && include_set.is_match(file_name))
    .unwrap_or(false)
}

/// Find all files in the given directory that match the given glob rules.
pub(crate) fn walk_directory(
  path: impl AsRef<Path>,
  glob_rules: &Vec<String>,
) -> AnyResult<Vec<DirEntry>> {
  let mut include_builder = GlobSetBuilder::new();
  let mut exclude_builder = GlobSetBuilder::new();

  for rule in glob_rules {
    if rule.starts_with('!') {
      let glob = Glob::new(&rule[1..])?;
      exclude_builder.add(glob);
      continue;
    }

    let glob = Glob::new(rule)?;
    include_builder.add(glob);
  }

  let include_set = include_builder.build()?;
  let exclude_set = exclude_builder.build()?;

  let entries = WalkDir::new(path)
    .into_iter()
    .filter_map(Result::ok)
    .filter(|entry| entry.file_type().is_file())
    .filter(|entry| is_match(entry, &include_set, &exclude_set))
    .collect::<Vec<_>>();

  Ok(entries)
}
