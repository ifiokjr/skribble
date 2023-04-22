use std::cmp::Ordering;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ClassSize {
  pub layer: usize,
  pub css_chunk: usize,
  pub media_queries: Vec<usize>,
  pub modifiers: Vec<usize>,
  pub atom: usize,
  pub value_name: usize,
  pub named_class: usize,
  pub alias: usize,
  pub argument: Vec<u8>,
}

impl PartialOrd for ClassSize {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for ClassSize {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .layer
      .cmp(&other.layer)
      .then(self.css_chunk.cmp(&other.css_chunk))
      .then(self.media_queries.cmp(&other.media_queries))
      .then(self.modifiers.cmp(&other.modifiers))
      .then(self.atom.cmp(&other.atom))
      .then(self.value_name.cmp(&other.value_name))
      .then(self.named_class.cmp(&other.named_class))
      .then(self.alias.cmp(&other.alias))
      .then(self.argument.cmp(&other.argument))
  }
}
