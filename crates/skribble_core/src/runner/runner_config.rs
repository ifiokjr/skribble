use std::sync::Arc;

use indexmap::IndexMap;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::Alias;
use crate::Atom;
use crate::AtomType;
use crate::CssChunk;
use crate::CssVariable;
use crate::Error;
use crate::Keyframe;
use crate::MediaQuery;
use crate::Modifier;
use crate::NamedClass;
use crate::Options;
use crate::Result;
use crate::StringMap;
use crate::Transformer;
use crate::ValueSet;

/// The configuration after all plugins have been run.
#[derive(Clone, Debug, Default, Deserialize, Serialize, TypedBuilder)]
pub struct RunnerConfig {
	#[serde(skip)]
	_options: Arc<Options>,
	pub aliases: IndexMap<String, Alias>,
	pub atoms: IndexMap<String, Atom>,
	pub classes: IndexMap<String, NamedClass>,
	pub css_chunks: IndexMap<String, CssChunk>,
	pub css_variables: IndexMap<String, CssVariable>,
	pub keyframes: IndexMap<String, Keyframe>,
	pub layers: IndexSet<String>,
	pub media_queries: IndexMap<String, IndexMap<String, MediaQuery>>,
	pub modifiers: IndexMap<String, IndexMap<String, Modifier>>,
	pub transformers: IndexMap<String, IndexMap<String, Transformer>>,
	#[builder(default)]
	pub names: IndexMap<String, IndexSet<String>>,
	pub palette: StringMap,
	pub value_sets: IndexMap<String, ValueSet>,
}

impl RunnerConfig {
	pub fn get_alias_index(&self, name: impl AsRef<str>) -> Option<usize> {
		self.names
			.get("aliases")
			.and_then(|map| map.get_index_of(name.as_ref()))
	}

	pub fn get_atom_index(&self, name: impl AsRef<str>) -> Option<usize> {
		self.names
			.get("atoms")
			.and_then(|map| map.get_index_of(name.as_ref()))
	}

	pub fn get_atom_type(&self, name: impl AsRef<str>) -> Option<AtomType> {
		self.atoms.get(name.as_ref()).map(|atom| atom.get_type())
	}

	pub fn get_atom_values_index(
		&self,
		atom_name: impl AsRef<str>,
		value_name: impl AsRef<str>,
	) -> Option<usize> {
		let lookup_name = get_atom_name_lookup_name(atom_name);
		self.names
			.get(&lookup_name)
			.and_then(|map| map.get_index_of(value_name.as_ref()))
	}

	pub fn get_css_chunk_index(&self, name: impl AsRef<str>) -> Option<usize> {
		self.names
			.get("css_chunks")
			.and_then(|map| map.get_index_of(name.as_ref()))
	}

	pub fn get_media_queries(&self) -> Vec<&MediaQuery> {
		self.media_queries
			.values()
			.flat_map(|map| map.values())
			.collect()
	}

	pub fn get_media_query(&self, name: impl AsRef<str>) -> Option<&MediaQuery> {
		self.get_media_queries()
			.into_iter()
			.find(|&media_query| media_query.name == name.as_ref())
	}

	pub fn get_media_query_index(&self, name: impl AsRef<str>) -> Option<usize> {
		self.names
			.get("media_queries")
			.and_then(|map| map.get_index_of(name.as_ref()))
	}

	pub fn get_modifier(&self, name: impl AsRef<str>) -> Option<&Modifier> {
		self.get_modifiers()
			.into_iter()
			.find(|&modifier| modifier.name == name.as_ref())
	}

	pub fn get_modifier_index(&self, name: impl AsRef<str>) -> Option<usize> {
		self.names
			.get("modifiers")
			.and_then(|map| map.get_index_of(name.as_ref()))
	}

	pub fn get_modifiers(&self) -> Vec<&Modifier> {
		self.modifiers
			.values()
			.flat_map(|map| map.values())
			.collect()
	}

	pub fn get_transformer(&self, name: impl AsRef<str>) -> Option<&Transformer> {
		self.get_transformers()
			.into_iter()
			.find(|&transformer| transformer.name == name.as_ref())
	}

	pub fn get_transformer_index(&self, name: impl AsRef<str>) -> Option<usize> {
		self.names
			.get("transformers")
			.and_then(|map| map.get_index_of(name.as_ref()))
	}

	pub fn get_transformers(&self) -> Vec<&Transformer> {
		self.transformers
			.values()
			.flat_map(|map| map.values())
			.collect()
	}

	pub fn get_named_class_index(&self, name: impl AsRef<str>) -> Option<usize> {
		self.names
			.get("classes")
			.and_then(|map| map.get_index_of(name.as_ref()))
	}

	pub fn has_atom(&self, name: impl AsRef<str>) -> bool {
		let name = name.as_ref().to_string();
		self.names
			.get("atoms")
			.as_ref()
			.map(|map| map.contains(&name))
			.unwrap_or(false)
	}

	pub fn has_class(&self, name: impl AsRef<str>) -> bool {
		let name = name.as_ref().to_string();
		self.names
			.get("classes")
			.as_ref()
			.map(|map| map.contains(&name))
			.unwrap_or(false)
	}

	pub fn has_css_variable(&self, name: impl AsRef<str>) -> bool {
		let name = name.as_ref().to_string();
		self.names
			.get("css_variables")
			.as_ref()
			.map(|map| map.contains(&name))
			.unwrap_or(false)
	}

	pub fn has_keyframe(&self, name: impl AsRef<str>) -> bool {
		let name = name.as_ref().to_string();
		self.names
			.get("keyframes")
			.as_ref()
			.map(|map| map.contains(&name))
			.unwrap_or(false)
	}

	pub fn has_media_query(&self, name: impl AsRef<str>) -> bool {
		let name = name.as_ref().to_string();
		self.names
			.get("media_queries")
			.as_ref()
			.map(|map| map.contains(&name))
			.unwrap_or(false)
	}

	pub fn has_modifier(&self, name: impl AsRef<str>) -> bool {
		let name = name.as_ref().to_string();
		self.names
			.get("modifiers")
			.as_ref()
			.map(|map| map.contains(&name))
			.unwrap_or(false)
	}

	/// Load the options
	pub fn options(&self) -> &Options {
		&self._options
	}

	pub fn to_json(&self) -> Result<String> {
		serde_json::to_string_pretty(self).map_err(Error::CouldNotSerializeConfig)
	}
}

pub(crate) fn get_atom_name_lookup_name(atom_name: impl AsRef<str>) -> String {
	format!("atom:{}", atom_name.as_ref())
}
