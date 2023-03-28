use std::cmp::Ordering;
use std::fmt::Write;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use heck::ToKebabCase;
use indent_write::fmt::IndentWriter;
use indexmap::indexset;
use indexmap::IndexMap;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::indent_writer;
use crate::AnyEmptyResult;
use crate::AnyResult;
use crate::Arguments;
use crate::ClassFactory;
use crate::ClassSize;
use crate::Placeholder;
use crate::RunnerConfig;
use crate::ToSkribbleCss;

/// These represent an atomic class and should be
#[derive(Clone, Debug, Default, Deserialize, Serialize, TypedBuilder, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
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
  /// The name of the shorthand class.
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

  /// Get the string representation of the selector for this `SkribbleClass`.
  ///
  /// - Convert `["sm", "focus", "text", "red"]` -> `"sm\:text-red:focus"`
  /// - Convert `tokens: ["sm", "p"], argument: "100px"` -> `"sm\:p-\[100px\]"`
  pub fn selector(&self, config: &RunnerConfig) -> AnyResult<String> {
    let mut writer = String::new();
    self.write_selector(&mut writer, config)?;
    Ok(writer)
  }

  fn write_selector(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
    let mut tokens = vec![];

    for media_query in self.media_queries.iter() {
      tokens.push(media_query.to_kebab_case());
    }

    for modifier in self.modifiers.iter() {
      tokens.push(modifier.to_kebab_case());
    }

    if let Some(ref named_class) = self.named_class {
      let name = named_class.to_kebab_case();
      tokens.push(format!("\\${name}"));
    }

    if let Some(ref atom) = self.atom {
      tokens.push(atom.to_kebab_case());
    }

    if let Some(ref value_name) = self.value_name {
      let name = value_name.to_kebab_case();
      tokens.push(format!("\\${name}"));
    }

    let mut selector = format!(".{}", tokens.join("\\:"));

    // Append an argument if it exists.
    if let Some(ref argument) = self.argument {
      let prefix = if tokens.is_empty() { "" } else { "-" };
      let argument = argument.to_string();
      selector = format!("{selector}{prefix}[{argument}]");
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
      }
    }

    Ok(())
  }
}

impl ToSkribbleCss for Class {
  fn write_skribble_css(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
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

    for mq in self.media_queries.iter() {
      mq.hash(state);
    }

    for mq in self.modifiers.iter() {
      mq.hash(state);
    }

    self.atom.hash(state);
    self.value_name.hash(state);
    self.named_class.hash(state);
    self.argument.hash(state);
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Classes(IndexSet<Class>);

impl Classes {
  pub fn insert_factory(&mut self, class_factory: ClassFactory) -> bool {
    self.insert_optional(class_factory.into_class())
  }

  pub fn insert_factories(&mut self, class_factories: Vec<ClassFactory>) {
    for class_factory in class_factories {
      self.insert_factory(class_factory);
    }
  }

  pub fn insert_optional(&mut self, class: Option<Class>) -> bool {
    match class {
      Some(class) => self.insert(class),
      _ => false,
    }
  }

  pub fn extend_optional(&mut self, classes: Option<Classes>) {
    if let Some(classes) = classes {
      self.extend(classes);
    }
  }

  pub fn merge(&mut self, other: impl Into<Self>) {
    self.extend(other.into().0);
  }

  pub fn sort_by_class(&mut self) {
    self.sort_by(|a, b| a.cmp(b));
  }

  fn write_keyframes(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
    let mut set = indexset! {};
    for class in self.iter() {
      let Some(keyframe) = class.get_keyframe() else {
        continue;
      };

      if set.get(keyframe).is_some() {
        continue;
      }

      set.insert(keyframe);

      let keyframe = config
        .keyframes
        .get(keyframe)
        .ok_or_else(|| format!("Keyframe {} not found", keyframe))?;

      keyframe.write_skribble_css(writer, config)?;
    }

    Ok(())
  }

  fn write_layers_header(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
    let length = config.layers.len();

    if length == 0 {
      return Ok(());
    }

    write!(writer, "@layer ")?;

    for (index, layer) in config.layers.iter().enumerate() {
      write!(writer, "{layer}")?;

      if index + 1 < length {
        write!(writer, ", ")?;
      }
    }

    writeln!(writer, ";")?;

    Ok(())
  }

  fn write_layer_css(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    layer: Option<&String>,
  ) -> AnyEmptyResult {
    let mut media_query_classes = IndexMap::<Option<String>, Vec<&Class>>::new();

    for class in self.iter().filter(|class| class.get_layer() == layer) {
      let key = class.join_media_query(config);
      match media_query_classes.get_mut(&key) {
        Some(existing) => {
          existing.push(class);
        }
        None => {
          media_query_classes.insert(key, vec![class]);
        }
      }
    }

    for (media_query, classes) in media_query_classes.iter() {
      if let Some(media_query) = media_query {
        writeln!(writer, "@media {media_query} {{")?;
        let mut child_writer = indent_writer();
        self.write_media_query_css(&mut child_writer, config, classes)?;
        write!(writer, "{}", child_writer.get_ref())?;
        writeln!(writer, "}}")?;
      } else {
        self.write_media_query_css(writer, config, classes)?;
      }
    }

    Ok(())
  }

  fn write_media_query_css(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    classes: &Vec<&Class>,
  ) -> AnyEmptyResult {
    for class in classes {
      class.write_skribble_css(writer, config)?;
    }
    Ok(())
  }
}

impl ToSkribbleCss for Classes {
  fn write_skribble_css(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
    let options = config.options();

    writeln!(writer, "/* Generated by Skribble */")?;
    writeln!(writer, "@charset \"{}\";", options.charset)?;
    self.write_layers_header(writer, config)?;
    self.write_keyframes(writer, config)?;

    for layer in config.layers.iter() {
      writeln!(writer, "@layer {layer} {{")?;
      let mut indented = IndentWriter::new("  ", String::new());

      self.write_layer_css(
        &mut indented,
        config,
        if layer == &options.default_layer {
          None
        } else {
          Some(layer)
        },
      )?;
      write!(writer, "{}", indented.get_ref())?;
      writeln!(writer, "}}")?;
    }

    Ok(())
  }
}

impl From<Vec<Class>> for Classes {
  fn from(classes: Vec<Class>) -> Self {
    Self(classes.into_iter().collect())
  }
}

impl From<IndexSet<Class>> for Classes {
  fn from(classes: IndexSet<Class>) -> Self {
    Self(classes.into_iter().collect())
  }
}

impl IntoIterator for Classes {
  type IntoIter = <IndexSet<Class> as IntoIterator>::IntoIter;
  type Item = Class;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl FromIterator<Class> for Classes {
  fn from_iter<T: IntoIterator<Item = Class>>(iter: T) -> Self {
    Self(iter.into_iter().collect())
  }
}

impl Deref for Classes {
  type Target = IndexSet<Class>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Classes {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
