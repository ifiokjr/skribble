use std::cmp::Ordering;
use std::fmt::Write;
use std::hash::Hash;
use std::hash::Hasher;

use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::Arguments;
use super::ClassScore;
use super::ClassTransformer;
use crate::format_css_string;
use crate::indent_writer;
use crate::AnyEmptyResult;
use crate::AnyResult;
use crate::AtomType;
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
  /// The ordered list of transformers.
  #[builder(setter(into))]
  transformers: IndexSet<ClassTransformer>,
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
  score: ClassScore,
  /// The keyframes used in this class.
  #[builder(setter(into))]
  atom_type: Option<AtomType>,
  /// The css variables that are referenced by this class.
  #[builder(setter(into))]
  css_variables: IndexSet<String>,
  /// The css chunks that are referenced by this class.
  #[builder(setter(into))]
  css_chunk: Option<String>,
  /// The alias that is referenced by this class.
  #[builder(setter(into))]
  alias: Option<String>,
  /// The parent selector that is referenced by this class (applies only to
  /// named_classes)
  #[builder(setter(into))]
  parent_class_name: Option<String>,
}

impl Class {
  pub fn get_atom_type(&self) -> Option<AtomType> {
    self.atom_type
  }

  pub fn get_keyframe(&self) -> Option<&String> {
    self
      .get_atom_type()
      .filter(|ty| *ty == AtomType::Keyframes)
      .and_then(|_| self.get_value_name())
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

  pub fn get_transformers(&self) -> &IndexSet<ClassTransformer> {
    &self.transformers
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

  pub fn class_name(&self) -> AnyResult<String> {
    let mut writer = String::new();
    self.write_class_name(&mut writer)?;
    Ok(writer)
  }

  fn write_class_name(&self, writer: &mut dyn Write) -> AnyEmptyResult {
    if let Some(ref class) = self.parent_class_name {
      write!(writer, "{}", class)?;
      return Ok(());
    }

    let mut tokens = vec![];

    for media_query in self.media_queries.iter() {
      tokens.push(media_query.to_string());
    }

    for modifier in self.modifiers.iter() {
      tokens.push(modifier.to_string());
    }

    for transformer in self.transformers.iter() {
      tokens.push(transformer.to_string());
    }

    if let Some(ref named_class) = self.named_class {
      tokens.push(format!("${named_class}"));
    }

    if let Some(ref atom) = self.atom {
      tokens.push(atom.to_string());
    }

    if let Some(ref value_name) = self.value_name {
      tokens.push(format!("${value_name}"));
    }

    if let Some(ref alias) = self.alias {
      tokens.push(format!("${alias}"));
    }

    // Append an argument if it exists.
    if let Some(ref argument) = self.argument {
      tokens.push(format!("[{argument}]"));
    };

    write!(writer, "{}", tokens.join(":"))?;

    Ok(())
  }

  fn write_selector(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
    let selector = format!(".{}", format_css_string(self.class_name()?));
    let mut selectors = vec![selector];
    let mut class_modifiers = vec![];

    if let Some(modifier) = self
      .get_named_class()
      .and_then(|name| config.classes.get(name))
      .and_then(|class| class.modifier.as_ref())
    {
      class_modifiers.push(modifier);
    }

    if let Some(modifier) = self
      .get_atom()
      .and_then(|name| config.atoms.get(name))
      .and_then(|atom| atom.modifier.as_ref())
    {
      class_modifiers.push(modifier);
    }

    // Handle modifiers.
    for modifier in self.modifiers.iter() {
      let Some(modifier) = config.get_modifier(modifier) else {
        continue;
      };

      for name in modifier.values.iter() {
        class_modifiers.push(name);
      }
    }

    for class_modifier in class_modifiers {
      let mut new_selectors = vec![];

      for selector in selectors.iter() {
        let selector = class_modifier.replace('&', selector);
        new_selectors.push(selector);
      }

      selectors = new_selectors;
    }

    write!(writer, "{}", selectors.join(", "))?;

    Ok(())
  }

  fn write_css_properties(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
    if let Some(atom) = self.get_atom().and_then(|atom| config.atoms.get(atom)) {
      if let Some(value_set_name) = self.get_value_name() {
        atom.write_css_properties(writer, config, value_set_name, self.get_transformers())?;
      } else if let Some(argument) = self.get_argument() {
        atom.write_css_argument(writer, config, argument, self.get_transformers())?;
      }
    }

    if let Some(named_class) = self
      .get_named_class()
      .and_then(|name| config.classes.get(name))
    {
      named_class.write_css_properties(writer, config)?;
    }

    if let Some(argument) = self.get_argument() {
      argument.write_css(writer, config, self.get_transformers())?;
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
    self.atom_type.hash(state);
    self.parent_class_name.hash(state);
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
