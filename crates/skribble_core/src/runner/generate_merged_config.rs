use std::sync::Arc;

use indexmap::indexset;
use indexmap::IndexMap;
use indexmap::IndexSet;

use super::get_atom_name_lookup_name;
use super::RunnerConfig;
use crate::default_layers;
use crate::Atom;
use crate::CssChunk;
use crate::CssVariable;
use crate::Error;
use crate::Keyframe;
use crate::MediaQuery;
use crate::Modifier;
use crate::NamedClass;
use crate::Options;
use crate::PluginConfig;
use crate::Result;
use crate::StringMap;
use crate::ValueSet;

pub(crate) fn generate_merged_config(
  mut plugin_config: PluginConfig,
  options: Arc<Options>,
  config: &PluginConfig,
) -> Result<RunnerConfig> {
  // mutate
  plugin_config.keyframes.extend(config.keyframes.clone());
  plugin_config.variables.extend(config.variables.clone());
  plugin_config
    .media_queries
    .extend(config.media_queries.clone());
  plugin_config.modifiers.extend(config.modifiers.clone());
  plugin_config.value_sets.extend(config.value_sets.clone());
  plugin_config.atoms.extend(config.atoms.clone());
  plugin_config.classes.extend(config.classes.clone());
  let mut initial_layers = plugin_config.layers.clone();
  initial_layers.merge(default_layers());
  plugin_config.layers.extend(initial_layers);

  let mut layers = indexset! {};
  let mut keyframes = IndexMap::<String, Keyframe>::new();
  let mut css_chunks = IndexMap::<String, CssChunk>::new();
  let mut css_variables = IndexMap::<String, CssVariable>::new();
  let mut media_queries = IndexMap::<String, IndexMap<String, MediaQuery>>::new();
  let mut modifiers = IndexMap::<String, IndexMap<String, Modifier>>::new();
  let mut atoms = IndexMap::<String, Atom>::new();
  let mut classes = IndexMap::<String, NamedClass>::new();
  let mut palette = StringMap::default();
  let mut value_sets = IndexMap::<String, ValueSet>::new();

  // layers
  let default_layer = &options.default_layer;
  plugin_config.layers.sort_from_lowest_priority();
  layers.extend(plugin_config.layers.into_iter().map(|layer| layer.value));

  if !layers.contains(default_layer) {
    return Err(Error::InvalidDefaultLayer(default_layer.into()));
  }

  // css_chunks
  plugin_config.css_chunks.extend(config.css_chunks.clone());
  for css_chunk in plugin_config.css_chunks.into_iter() {
    let key = &css_chunk.name;

    match css_chunks.get_mut(key) {
      Some(existing) => {
        existing.merge(css_chunk);
      }
      None => {
        css_chunks.insert(key.clone(), css_chunk);
      }
    }
  }

  // keyframes
  plugin_config.keyframes.extend(config.keyframes.clone());
  for keyframe in plugin_config.keyframes.into_iter() {
    let key = &keyframe.name;

    match keyframes.get_mut(key) {
      Some(existing) => {
        existing.merge(keyframe);
      }
      None => {
        keyframes.insert(key.clone(), keyframe);
      }
    }
  }

  // css_variables
  for css_variable in plugin_config.variables.into_iter() {
    let key = &css_variable.name;

    match css_variables.get_mut(key) {
      Some(existing) => {
        existing.merge(css_variable);
      }
      None => {
        css_variables.insert(key.clone(), css_variable);
      }
    }
  }

  // media_queries
  let mut wrapped_media_queries = plugin_config.media_queries;
  wrapped_media_queries.sort_by(|a, z| z.priority.cmp(&a.priority));

  for media_query_group in wrapped_media_queries.into_iter() {
    let group_name = media_query_group.name.clone();
    let mut group = IndexMap::<String, MediaQuery>::new();

    for media_query in media_query_group.into_iter() {
      let key = &media_query.name;
      match group.get_mut(key) {
        Some(existing) => {
          existing.merge(media_query);
        }
        None => {
          group.insert(key.clone(), media_query);
        }
      }
    }

    group.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));

    match media_queries.get_mut(&group_name) {
      Some(existing) => {
        existing.extend(group);
      }
      None => {
        media_queries.insert(group_name, group);
      }
    }
  }

  // modifiers
  let mut wrapped_modifiers = plugin_config.modifiers;
  wrapped_modifiers.sort_by(|a, z| z.priority.cmp(&a.priority));

  for modifier_group in wrapped_modifiers.into_iter() {
    let group_name = modifier_group.name.clone();
    let mut group = IndexMap::<String, Modifier>::new();

    for modifier in modifier_group.into_iter() {
      let key = &modifier.name;
      match group.get_mut(key) {
        Some(existing) => {
          existing.merge(modifier);
        }
        None => {
          group.insert(key.clone(), modifier);
        }
      }
    }

    group.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));

    match modifiers.get_mut(&group_name) {
      Some(existing) => {
        existing.extend(group);
      }
      None => {
        modifiers.insert(group_name.clone(), group);
      }
    }
  }

  // css_variables
  for atom in plugin_config.atoms.into_iter() {
    let key = &atom.name;

    match atoms.get_mut(key) {
      Some(existing) => {
        existing.merge(atom);
      }
      None => {
        atoms.insert(key.clone(), atom);
      }
    }
  }

  // classes
  for class in plugin_config.classes.into_iter() {
    let key = &class.name;

    match classes.get_mut(key) {
      Some(existing) => {
        existing.merge(class);
      }
      None => {
        classes.insert(key.clone(), class);
      }
    }
  }

  // palette
  palette.extend(plugin_config.palette);
  palette.extend(config.palette.clone());

  // value_sets
  for value_set in plugin_config.value_sets.into_iter() {
    let key = &value_set.name;

    match value_sets.get_mut(key) {
      Some(existing) => {
        existing.merge(value_set);
      }
      None => {
        value_sets.insert(key.clone(), value_set);
      }
    }
  }

  // sort by priority
  keyframes.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
  css_variables.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
  atoms.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
  classes.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
  value_sets.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));

  let mut names = IndexMap::<String, IndexSet<String>>::default();
  let keyframe_names = keyframes.keys().cloned().collect();
  let css_variable_names = css_variables.keys().cloned().collect();
  let atom_names = atoms.keys().cloned().collect();
  let class_names = classes.keys().cloned().collect();
  let css_chunk_names = css_chunks.keys().cloned().collect();
  let media_query_names = media_queries
    .iter()
    .flat_map(|(_, query)| query.keys().cloned())
    .collect();
  let modifier_names = modifiers
    .iter()
    .flat_map(|(_, query)| query.keys().cloned())
    .collect();

  names.insert("keyframes".into(), keyframe_names);
  names.insert("css_variables".into(), css_variable_names);
  names.insert("atoms".into(), atom_names);
  names.insert("classes".into(), class_names);
  names.insert("css_chunks".into(), css_chunk_names);
  names.insert("media_queries".into(), media_query_names);
  names.insert("modifiers".into(), modifier_names);

  let mut merged_config = RunnerConfig::builder()
    .layers(layers)
    .keyframes(keyframes)
    .css_variables(css_variables)
    .media_queries(media_queries)
    .modifiers(modifiers)
    .atoms(atoms)
    .classes(classes)
    .palette(palette)
    .value_sets(value_sets)
    .names(names)
    ._options(options)
    .css_chunks(css_chunks)
    .build();

  for (name, atom) in merged_config.atoms.iter() {
    let name_atom_name = get_atom_name_lookup_name(name);
    let atom_names = atom.values.get_names_from_config(&merged_config);
    merged_config.names.insert(name_atom_name, atom_names);
  }

  Ok(merged_config)
}
