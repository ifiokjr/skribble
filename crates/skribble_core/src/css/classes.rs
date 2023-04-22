use std::cmp::Ordering;
use std::fmt::Write;

use derive_more::Deref;
use derive_more::DerefMut;
use indent_write::fmt::IndentWriter;
use indexmap::indexset;
use indexmap::IndexMap;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;

use super::Class;
use crate::indent_writer;
use crate::AnyEmptyResult;
use crate::ClassFactory;
use crate::RunnerConfig;
use crate::StringMap;
use crate::ToSkribbleCss;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Deref, DerefMut)]
pub struct Classes(IndexSet<Class>);

impl Classes {
  pub fn insert_factory(&mut self, class_factory: ClassFactory) {
    self.extend(class_factory.into_classes())
  }

  pub fn insert_factories(&mut self, class_factories: Vec<ClassFactory>) {
    for class_factory in class_factories {
      self.insert_factory(class_factory);
    }
  }

  pub fn merge(&mut self, other: impl Into<Self>) {
    self.extend(other.into().0);
    self.sort_by_class();
  }

  pub fn sort_by_class(&mut self) {
    self.sort_by(|a, z| a.cmp(z));
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

  fn write_css_variables(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
    let mut css_variables = indexset! {};

    for class in self.iter() {
      class.collect_css_variables(&mut css_variables);
    }

    for value in css_variables
      .iter()
      .map(|name| config.css_variables.get(name))
    {
      let Some( css_variable) = value else {
        continue;
      };

      css_variable.write_property_rule(writer, config, true)?;
    }

    // TODO think about how to handle the nested css variables in media queries

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
    let mut media_query_variables = IndexMap::<Option<String>, StringMap>::new();
    let mut media_query_text = IndexMap::<Option<String>, String>::new();
    let mut css_variables = indexset! {};
    let mut css = String::new();

    for class in self.iter().filter(|class| class.get_layer() == layer) {
      class.collect_css_variables(&mut css_variables);

      if let Some(chunk) = class
        .get_css_chunk()
        .and_then(|chunk| config.css_chunks.get(chunk))
      {
        chunk.write_skribble_css(&mut css, config)?;
      }
    }

    for value in css_variables
      .iter()
      .map(|name| config.css_variables.get(name))
    {
      let Some(css_variable) = value else {
        continue;
      };

      css_variable.extend_media_query_dictionary(config, &mut media_query_variables)?;
    }

    for (media_query, selector_map) in media_query_variables.iter() {
      match media_query_text.get_mut(media_query) {
        Some(content) => {
          write_css_variable_selector_map(content, selector_map)?;
        }
        None => {
          let mut content = String::new();
          write_css_variable_selector_map(&mut content, selector_map)?;
          media_query_text.insert(media_query.clone(), content);
        }
      }
    }

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
      match media_query_text.get_mut(media_query) {
        Some(content) => {
          self.write_media_query_css(content, config, classes)?;
        }
        None => {
          let mut content = String::new();
          self.write_media_query_css(&mut content, config, classes)?;
          media_query_text.insert(media_query.clone(), content);
        }
      }
    }

    media_query_text.sort_by(|a, _, z, _| {
      let Some(a) = a else {
        return Ordering::Less;
      };

      let Some(z) = z else {
        return Ordering::Greater;
      };

      let a_pos = config
        .get_media_queries()
        .iter()
        .position(|query| &query.query == a);
      let z_pos = config
        .get_media_queries()
        .iter()
        .position(|query| &query.query == z);

      a_pos.cmp(&z_pos)
    });

    write!(writer, "{}", css)?;
    for (media_query, content) in media_query_text.iter() {
      if let Some(media_query) = media_query {
        writeln!(writer, "@media {media_query} {{")?;
        let mut child_writer = indent_writer();
        write!(&mut child_writer, "{}", content)?;
        write!(writer, "{}", child_writer.get_ref())?;
        writeln!(writer, "}}")?;
      } else {
        write!(writer, "{}", content)?;
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

fn write_css_variable_selector_map(
  writer: &mut dyn Write,
  selector_map: &StringMap,
) -> AnyEmptyResult {
  for (selector, properties) in selector_map.iter() {
    writeln!(writer, "{} {{", selector)?;
    let mut indented_writer = indent_writer();
    write!(indented_writer, "{}", properties)?;
    write!(writer, "{}", indented_writer.get_ref())?;
    writeln!(writer, "}}")?;
  }

  Ok(())
}

impl ToSkribbleCss for Classes {
  fn write_skribble_css(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult {
    let options = config.options();

    writeln!(writer, "/* Generated by Skribble */")?;
    writeln!(writer, "@charset \"{}\";", options.charset)?;
    self.write_layers_header(writer, config)?;
    self.write_keyframes(writer, config)?;
    self.write_css_variables(writer, config)?;

    for layer in config.layers.iter() {
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
      let content = indented.get_ref();

      if content.trim().is_empty() {
        continue;
      }

      writeln!(writer, "@layer {layer} {{")?;
      write!(writer, "{}", content)?;
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
