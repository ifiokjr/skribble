use std::cmp::Ordering;
use std::fmt::Write;
use std::hash::Hash;
use std::hash::Hasher;

use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::format_css_string;
use crate::indent_writer;
use crate::AnyEmptyResult;
use crate::AnyResult;
use crate::Arguments;
use crate::ClassSize;
use crate::RunnerConfig;
use crate::ToSkribbleCss;

/// These represent an atomic class and should be
#[derive(Clone, Debug, Default, Deserialize, Serialize, TypedBuilder, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[readonly::make]
pub struct Class {
  /// The layer for this class.
  #[builder(setter(into))]
  layer: Option<String>,
  /// The names of the media queries.
  #[builder(setter(into))]
  media_queries: IndexSet<String>,
  /// The ordered list of modifiers.
  #[builder(setter(into))]
  modifiers: IndexSet<String>,
  /// The name of the style provided. This must be provided for the `class_name`
  /// to be valid.
  #[builder(setter(into))]
  atom: Option<String>,
  /// The pre-configured value of the atom.
  #[builder(setter(into))]
  value_name: Option<String>,
  /// The name of the standalone class.
  #[builder(setter(into))]
  named_class: Option<String>,
  /// This is the callable argument when the provided value is a callable
  /// expression.
  #[builder(setter(into))]
  argument: Option<Arguments>,
  /// Used to compare to classes
  #[builder(setter(into))]
  score: ClassSize,
  /// The keyframes used in this class.
  #[builder(setter(into))]
  keyframe: bool,
  /// The css variables that are referenced by this class.
  #[builder(setter(into))]
  css_variables: IndexSet<String>,
  /// The css chunks that are referenced by this class.
  #[builder(setter(into))]
  css_chunk: Option<String>,
  /// The alias that is referenced by this class.
  #[builder(setter(into))]
  alias: Option<String>,
}

impl Class {
  pub fn get_keyframe(&self) -> Option<&String> {
    if self.keyframe {
      self.value_name.as_ref()
    } else {
      None
    }
  }

  pub fn get_layer(&self) -> Option<&String> {
    self.layer.as_ref()
  }

  pub fn get_media_queries(&self) -> &IndexSet<String> {
    &self.media_queries
  }

  pub fn join_media_query(&self, config: &RunnerConfig) -> Option<String> {
    if self.media_queries.is_empty() {
      return None;
    }

    let queries = self
      .media_queries
      .iter()
      .filter_map(|name| config.get_media_query(name))
      .map(|media_query| media_query.query.clone())
      .collect::<Vec<String>>()
      .join(" and ");

    Some(queries)
  }

  pub fn get_modifiers(&self) -> &IndexSet<String> {
    &self.modifiers
  }

  pub fn get_atom(&self) -> Option<&String> {
    self.atom.as_ref()
  }

  pub fn get_value_name(&self) -> Option<&String> {
    self.value_name.as_ref()
  }

  pub fn get_named_class(&self) -> Option<&String> {
    self.named_class.as_ref()
  }

  pub fn get_argument(&self) -> Option<&Arguments> {
    self.argument.as_ref()
  }

  pub fn get_css_chunk(&self) -> Option<&String> {
    self.css_chunk.as_ref()
  }

  pub fn get_alias(&self) -> Option<&String> {
    self.alias.as_ref()
  }

  pub fn collect_css_variables(&self, css_variables: &mut IndexSet<String>) {
    css_variables.extend(self.css_variables.iter().cloned());
  }

  /// Get the string representation of the selector for this `SkribbleClass`.
  ///
  /// - Convert `["sm", "focus", "text", "red"]` -> `".sm\:text-red:focus"`
  /// - Convert `tokens: ["sm", "p"], argument: "100px"` -> `".sm\:p-\[100px\]"`
  pub fn selector(&self, config: &RunnerConfig) -> AnyResult<String> {
    let mut writer = String::new();
    self.write_selector(&mut writer, config)?;
    Ok(writer)
  }

  fn write_selector(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
    let mut tokens = vec![];

    for media_query in self.media_queries.iter() {
      tokens.push(format_css_string(media_query));
    }

    for modifier in self.modifiers.iter() {
      tokens.push(format_css_string(modifier));
    }

    if let Some(ref named_class) = self.named_class {
      let name = format_css_string(named_class);
      tokens.push(format!("\\${name}"));
    }

    if let Some(ref atom) = self.atom {
      tokens.push(format_css_string(atom));
    }

    if let Some(ref value_name) = self.value_name {
      let name = format_css_string(value_name);
      tokens.push(format!("\\${name}"));
    }

    let mut selector = format!(".{}", tokens.join("\\:"));

    // Append an argument if it exists.
    if let Some(ref argument) = self.argument {
      let prefix = if tokens.is_empty() { "" } else { "\\:" };
      let argument = format_css_string(argument.to_string());
      selector = format!("{selector}{prefix}\\[{argument}\\]");
    };

    let mut selectors = vec![selector];

    // Handle modifiers.
    for modifier in self.modifiers.iter() {
      if let Some(modifiers) = config.modifiers.get(modifier) {
        let mut new_selectors = vec![];

        for modifier in modifiers.keys() {
          for selector in &selectors {
            new_selectors.push(modifier.replace('&', selector));
          }
        }

        if !new_selectors.is_empty() {
          selectors = new_selectors;
        }
      }
    }

    write!(writer, "{}", selectors.join(", "))?;

    Ok(())
  }

  fn write_css_properties(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
    if let Some(atom) = self.get_atom().and_then(|atom| config.atoms.get(atom)) {
      if let Some(value_set_name) = self.get_value_name() {
        atom.write_css_properties(writer, config, value_set_name)?;
      } else if let Some(argument) = self.get_argument() {
        atom.write_css_argument(writer, config, argument)?;
      }
    }

    if let Some(named_class) = self
      .get_named_class()
      .and_then(|name| config.classes.get(name))
    {
      named_class.write_css_properties(writer, config)?;
    }

    if let Some(argument) = self.get_argument() {
      argument.write_css(writer, config)?;
    }

    Ok(())
  }
}

impl ToSkribbleCss for Class {
  fn write_skribble_css(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
    if let Some(css_chunk) = self
      .css_chunk
      .as_ref()
      .and_then(|name| config.css_chunks.get(name))
    {
      css_chunk.write_skribble_css(writer, config)?;
      return Ok(());
    }

    self.write_selector(writer, config)?;
    writeln!(writer, " {{")?;
    let mut indented = indent_writer();
    self.write_css_properties(&mut indented, config)?;
    write!(writer, "{}", indented.get_ref())?;
    writeln!(writer, "}}")?;

    Ok(())
  }
}

impl Hash for Class {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.layer.hash(state);
    self.css_chunk.hash(state);

    for mq in self.media_queries.iter() {
      mq.hash(state);
    }

    for mq in self.modifiers.iter() {
      mq.hash(state);
    }

    for variable in self.css_variables.iter() {
      variable.hash(state);
    }

    self.atom.hash(state);
    self.value_name.hash(state);
    self.named_class.hash(state);
    self.argument.hash(state);
    self.keyframe.hash(state);
  }
}

impl PartialOrd for Class {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Class {
  fn cmp(&self, other: &Self) -> Ordering {
    self.score.cmp(&other.score)
  }
}
