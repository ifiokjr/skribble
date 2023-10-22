use indexmap::IndexMap;
use indexmap::IndexSet;

use super::Arguments;
use super::Class;
use super::ClassScore;
use super::ClassTransformer;
use crate::AtomType;
use crate::RunnerConfig;

/// Skribble classes represent a css class.
#[derive(Clone, Debug)]
pub struct ClassFactory<'config> {
	/// The name of the aliased class.
	alias: Option<String>,
	/// This is the callable argument when the provided value is a callable
	/// expression.
	argument: Option<Arguments>,
	/// The name of the style provided. This must be provided for the
	/// `class_name` to be valid.
	atom: Option<String>,
	/// The finalized configuration which was used to create this class name.
	config: &'config RunnerConfig,
	/// The css chunks that is referenced by this class.
	css_chunk: Option<String>,
	/// Whether the atom of this class is a keyframe. The keyframe name should
	/// be taken from value_name.
	atom_type: Option<AtomType>,
	/// The layer to be used for this class. If left empty the default layer
	/// will be used.
	layer: Option<String>,
	/// The names of the media queries.
	media_queries: IndexMap<String, usize>,
	/// The ordered list of modifiers.
	modifiers: IndexMap<String, usize>,
	/// The ordered list of modifiers.
	transformers: IndexMap<ClassTransformer, (usize, usize)>,
	/// The name of the shorthand class.
	named_class: Option<String>,
	/// The score of this class. This is used to determine the order of the
	/// classes provided. A smaller number appears first.
	score: ClassScore,
	/// Whether this class is valid or not.
	valid: Option<bool>,
	/// The pre-configured value of the atom.
	value_name: Option<String>,
	/// The parent selector of this class. This only applies to named classes.
	parent_class_name: Option<String>,
}

impl<'config> ClassFactory<'config> {
	/// Create a class factory from the provided string.
	pub fn from_string(config: &'config RunnerConfig, string: impl AsRef<str>) -> Self {
		let string = string.as_ref().trim();
		let mut factory = Self::new(config);

		for token in string.split(':') {
			if token.starts_with('(') && token.ends_with(')') {
				let transformer = ClassTransformer::from(token);
				factory.add_transformer(&transformer);
				continue;
			}

			if token.starts_with('[') && token.ends_with(']') {
				if let Some(value) = token.get(1..token.len() - 1) {
					factory.add_argument(value.into());
				}

				continue;
			}

			if !token.starts_with('$') {
				factory.add_token(token);
				continue;
			}

			if let Some(value) = token.get(1..) {
				factory.add_token(value);
			}
		}

		factory
	}

	pub fn new(config: &'config RunnerConfig) -> Self {
		Self {
			config,
			alias: None,
			argument: None,
			atom: None,
			css_chunk: None,
			atom_type: None,
			layer: None,
			media_queries: IndexMap::new(),
			modifiers: IndexMap::new(),
			transformers: IndexMap::new(),
			named_class: None,
			score: ClassScore::default(),
			valid: None,
			value_name: None,
			parent_class_name: None,
		}
	}
}

impl<'config> ClassFactory<'config> {
	pub fn get_atom_type(&self) -> Option<AtomType> {
		self.atom_type
	}

	pub fn get_atom(&self) -> Option<&String> {
		self.atom.as_ref()
	}

	pub fn add_argument(&mut self, argument: Arguments) -> &Self {
		match argument {
			Arguments::V(_) => {
				if self.argument.is_some() || self.atom.is_none() || self.named_class.is_some() {
					self.valid = Some(false);
				} else {
					self.score.argument = argument.to_string().into();
					self.argument = Some(argument);
					self.valid = Some(true);
				}
			}
			Arguments::KV(..) => {
				if self.argument.is_some() || self.atom.is_some() || self.named_class.is_some() {
					self.valid = Some(false);
				} else {
					self.score.argument = argument.to_string().into();
					self.argument = Some(argument);
					self.valid = Some(true);
				}
			}
		}

		self
	}

	pub fn add_css_chunk(&mut self, token: impl AsRef<str>) -> &Self {
		if self.is_locked() {
			return self;
		}

		let Some(index) = self.config.get_css_chunk_index(&token) else {
			self.valid = Some(false);
			return self;
		};

		let Some(css_chunk) = self.config.css_chunks.get(token.as_ref()) else {
			self.valid = Some(false);
			return self;
		};

		if self.css_chunk.is_some()
			|| self.atom.is_some()
			|| self.named_class.is_some()
			|| !self.media_queries.is_empty()
			|| !self.modifiers.is_empty()
		{
			self.valid = Some(false);
		} else {
			self.css_chunk = Some(token.as_ref().to_string());
			self.layer = Some(css_chunk.layer.clone());
			self.score.css_chunk = index.checked_add(1).unwrap_or(index);
			self.valid = Some(true);
		}

		self
	}

	pub fn add_token(&mut self, token: impl AsRef<str>) -> &mut Self {
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
				self.valid = Some(true);
			}
		}
		// media_query
		else if self.add_media_query_token(&token) || self.add_modifier_token(&token) {
			// Prevent further branches being run
		}
		// atom.
		else if let Some(index) = self.config.get_atom_index(&token) {
			if self.atom.is_some() {
				self.valid = Some(false);
			} else {
				self.atom = Some(token.as_ref().to_string());
				self.score.atom = index.checked_add(1).unwrap_or(index);
				self.atom_type = self.config.get_atom_type(&token);
			}
		}
		// named_class.
		else if let Some((index, named_class)) = self
			.config
			.get_named_class_index(&token)
			.zip(self.config.classes.get(token.as_ref()))
		{
			if self.named_class.is_some() {
				self.valid = Some(false);
			} else {
				self.named_class = Some(token.as_ref().to_string());
				self.score.named_class = index.checked_add(1).unwrap_or(index);

				if let Some(ref name) = named_class.layer {
					self.layer = Some(name.clone());
				};

				self.valid = Some(true);
			}
		} else if let Some(index) = self.config.get_alias_index(&token) {
			if self.alias.is_some() {
				self.valid = Some(false);
			} else {
				self.alias = Some(token.as_ref().to_string());
				self.score.alias = index.checked_add(1).unwrap_or(index);
				self.valid = Some(true);
			}
		}
		// invalid value received.
		else {
			self.valid = Some(false);
		}

		self
	}

	fn add_modifier_token(&mut self, token: impl AsRef<str>) -> bool {
		if let Some(index) = self.config.get_modifier_index(&token) {
			if self.modifiers.contains_key(token.as_ref()) {
				self.valid = Some(false);
			} else {
				self.modifiers.insert(token.as_ref().to_string(), index);
				self.score
					.modifiers
					.push(index.checked_add(1).unwrap_or(index));
				self.score.modifiers.sort();
				self.modifiers
					.sort_by(|_, a_index, _, z_index| a_index.cmp(z_index));
			}

			true
		} else {
			false
		}
	}

	pub fn add_transformer(&mut self, transformer: &ClassTransformer) -> bool {
		if let Some(index) = self.config.get_transformer_index(&transformer.name) {
			if self.transformers.contains_key(transformer) {
				self.valid = Some(false);
			} else {
				let value_index = self
					.config
					.get_transformer(&transformer.name)
					.and_then(|transformer| transformer.values.as_ref())
					.zip(transformer.get_reference_value())
					.and_then(|(values, key)| values.get_index_of(key))
					.and_then(|index| index.checked_add(1))
					.unwrap_or(0);
				self.transformers
					.insert(transformer.clone(), (index, value_index));
				self.score.transformers.push((index, value_index));
				self.score.transformers.sort();
				self.transformers
					.sort_by(|_, a_index, _, z_index| a_index.cmp(z_index));
			}

			true
		} else {
			false
		}
	}

	fn add_media_query_token(&mut self, token: impl AsRef<str>) -> bool {
		if let Some(index) = self.config.get_media_query_index(&token) {
			if self.media_queries.contains_key(token.as_ref()) {
				self.valid = Some(false);
			} else {
				self.media_queries.insert(token.as_ref().to_string(), index);
				self.score
					.media_queries
					.push(index.checked_add(1).unwrap_or(index));
				self.score.media_queries.sort();
				self.media_queries
					.sort_by(|_, a_index, _, z_index| a_index.cmp(z_index));
			}

			true
		} else {
			false
		}
	}

	/// Create a new class from this factory. It will return none if the class
	/// is not valid.
	fn into_class(self) -> Option<Class> {
		if !self.is_valid() {
			return None;
		}

		let mut css_variables = IndexSet::new();

		if let Some(atom) = self
			.atom
			.as_ref()
			.and_then(|name| self.config.atoms.get(name))
		{
			atom.collect_css_variables(self.config, self.value_name.as_ref(), &mut css_variables)
		}

		if let Some(named_class) = self
			.named_class
			.as_ref()
			.and_then(|name| self.config.classes.get(name))
		{
			named_class.collect_css_variables(&mut css_variables)
		}

		let media_queries: IndexSet<String> =
			self.media_queries.into_iter().map(|(key, _)| key).collect();
		let modifiers: IndexSet<String> = self.modifiers.into_iter().map(|(key, _)| key).collect();
		let transformers: IndexSet<ClassTransformer> =
			self.transformers.into_iter().map(|(key, _)| key).collect();

		let class = Class::builder()
			.alias(self.alias)
			.argument(self.argument)
			.atom(self.atom)
			.css_chunk(self.css_chunk)
			.css_variables(css_variables)
			.atom_type(self.atom_type)
			.layer(self.layer)
			.media_queries(media_queries)
			.modifiers(modifiers)
			.transformers(transformers)
			.named_class(self.named_class)
			.score(self.score)
			.value_name(self.value_name)
			.parent_class_name(self.parent_class_name)
			.build();

		Some(class)
	}

	pub fn into_classes(self) -> Vec<Class> {
		let mut classes = vec![];
		let config = self.config;

		if let Some(alias) = self
			.alias
			.as_ref()
			.and_then(|alias| config.aliases.get(alias))
		{
			if !alias.combined {
				for name in alias.classes.iter() {
					let mut factory = Self::from_string(config, name);

					for token in self.media_queries.keys() {
						factory.add_media_query_token(token);
					}

					for token in self.modifiers.keys() {
						factory.add_modifier_token(token);
					}

					for transformer in self.transformers.keys() {
						factory.add_transformer(transformer);
					}

					let Some(class) = factory.into_class() else {
						continue;
					};

					classes.push(class);
				}
			} else if let Some(class) = self.into_class() {
				// TODO this should only be done if `alias.combine == true`
				// Currently this is a noop because combining the class will take a lot more
				// work.
				classes.push(class);
			}
		} else if let Some(class) = self.into_class().take() {
			if let Some((atom, selector)) = class
				.get_atom()
				.and_then(|atom_name| config.atoms.get(atom_name))
				.zip(class.class_name().ok())
			{
				for named_class in atom.children.iter() {
					let mut factory = Self::new(config);

					for token in class.get_media_queries() {
						factory.add_media_query_token(token);
					}

					for token in class.get_modifiers() {
						factory.add_modifier_token(token);
					}

					for transformer in class.get_transformers() {
						factory.add_transformer(transformer);
					}

					factory.add_token(named_class);
					factory.parent_class_name = Some(selector.clone());

					let Some(child_class) = factory.into_class() else {
						continue;
					};

					classes.push(child_class);
				}
			}

			classes.push(class);
		}

		classes
	}

	/// Checks whether the class has been invalidated.
	pub fn is_invalid(&self) -> bool {
		match self.valid {
			Some(valid) => !valid,
			None => false,
		}
	}

	/// Once a class name is validated or invalidated then it is locked and
	/// cannot be changed.
	pub fn is_locked(&self) -> bool {
		self.valid.is_some()
	}

	/// Checks whether the class has been validated.
	pub fn is_valid(&self) -> bool {
		self.valid.unwrap_or(false)
	}
}
