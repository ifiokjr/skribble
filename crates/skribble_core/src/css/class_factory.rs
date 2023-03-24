use std::sync::Arc;

use heck::ToKebabCase;
use indexmap::IndexSet;

use crate::Arguments;
use crate::Class;
use crate::ClassSize;
use crate::RunnerConfig;

/// Skribble classes represent a css class.
#[derive(Clone, Debug, Default)]
pub struct ClassFactory {
  /// The layer to be used for this class. If left empty the default layer will
  /// be used.
  layer: Option<String>,
  /// The names of the media queries.
  media_queries: IndexSet<String>,
  /// The ordered list of modifiers.
  modifiers: IndexSet<String>,
  /// The name of the style provided. This must be provided for the `class_name`
  /// to be valid.
  atom: Option<String>,
  /// The pre-configured value of the atom.
  value_name: Option<String>,
  /// The name of the shorthand class.
  named_class: Option<String>,
  /// This is the callable argument when the provided value is a callable
  /// expression.
  argument: Option<Arguments>,
  /// The finalized configuration which was used to create this class name.
  config: Arc<RunnerConfig>,
  /// Whether this class is valid or not.
  valid: Option<bool>,
  /// The score of this class. This is used to determine the order of the
  /// classes provided. A smaller number appears first.
  score: ClassSize,
  /// Whether the atom of this class is a keyframe. The keyframe name should be
  /// taken from value_name.
  keyframe: bool,
}

impl ClassFactory {
  /// Create a new class from this factory. It will return none if the class is
  /// not valid.
  pub fn into_class(self) -> Option<Class> {
    if !self.is_valid() {
      return None;
    }

    let class = Class::builder()
      .selector(self.selector())
      .media_queries(self.media_queries)
      .modifiers(self.modifiers)
      .score(self.score)
      .layer(self.layer)
      .atom(self.atom)
      .value_name(self.value_name)
      .named_class(self.named_class)
      .argument(self.argument)
      .keyframe(self.keyframe)
      .build();

    Some(class)
  }

  /// Get the string representation of the selector for this `SkribbleClass`.
  ///
  /// - Convert `["sm", "focus", "text", "red"]` -> `"sm\:text-red:focus"`
  /// - Convert `tokens: ["sm", "p"], argument: "100px"` -> `"sm\:p-\[100px\]"`
  pub fn selector(&self) -> String {
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
      if let Some(modifiers) = self.config.modifiers.get(modifier) {
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

    selectors.join(", ")
  }

  /// Checks whether the class has been invalidated.
  pub fn is_invalid(&self) -> bool {
    match self.valid {
      Some(valid) => !valid,
      None => false,
    }
  }

  /// Checks whether the class has been validated.
  pub fn is_valid(&self) -> bool {
    self.valid.unwrap_or(false)
  }

  /// Once a class name is validated or invalidated then it is locked and cannot
  /// be changed.
  pub fn is_locked(&self) -> bool {
    self.valid.is_some()
  }

  pub fn add_token(&mut self, token: impl AsRef<str>) -> &Self {
    if self.is_locked() {
      return self;
    }

    // value_name.
    if let Some(index) = self
      .atom
      .as_ref()
      .and_then(|atom_name| self.config.get_atom_values_index(atom_name, &token))
    {
      if self.value_name.is_some() {
        self.valid = Some(false);
      } else {
        self.value_name = Some(token.as_ref().to_string());
        self.score.value_name = index.checked_add(1).unwrap_or(index);
      }
    }
    // layer.
    else if let Some(index) = self.config.layers.get_index_of(token.as_ref()) {
      if self.layer.is_some() {
        self.valid = Some(false);
      } else {
        self.layer = Some(token.as_ref().to_string());
        self.score.layer = index.checked_add(1).unwrap_or(index);
      }
    }
    // media_query.
    else if let Some(index) = self.config.get_media_query_index(&token) {
      if self.media_queries.contains(token.as_ref()) {
        self.valid = Some(false);
      } else {
        self.media_queries.insert(token.as_ref().to_string());
        self
          .score
          .media_queries
          .push(index.checked_add(1).unwrap_or(index));
      }
    }
    // modifier.
    else if let Some(index) = self.config.get_modifier_index(&token) {
      if self.modifiers.contains(token.as_ref()) {
        self.valid = Some(false);
      } else {
        self.modifiers.insert(token.as_ref().to_string());
        self
          .score
          .modifiers
          .push(index.checked_add(1).unwrap_or(index));
      }
    }
    // atom.
    else if let Some(index) = self.config.get_atom_index(&token) {
      if self.atom.is_some() {
        self.valid = Some(false);
      } else {
        self.atom = Some(token.as_ref().to_string());
        self.score.atom = index.checked_add(1).unwrap_or(index);
        self.keyframe = self.config.get_atom_is_keyframe(&token);
      }
    }
    // named_class.
    else if let Some(index) = self.config.get_named_class_index(&token) {
      if self.named_class.is_some() {
        self.valid = Some(false);
      } else {
        self.named_class = Some(token.as_ref().to_string());
        self.score.named_class = index.checked_add(1).unwrap_or(index);
      }
    }
    // invalid value received.
    else {
      self.valid = Some(false);
    }

    self
  }
}
