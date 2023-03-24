use std::sync::Arc;
use std::sync::Mutex;

use indexmap::IndexMap;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::Atom;
use crate::BoxedPlugin;
use crate::CssVariable;
use crate::Error;
use crate::GeneratedFiles;
use crate::Keyframe;
use crate::LinkedValues;
use crate::MediaQuery;
use crate::Modifier;
use crate::NamedClass;
use crate::Options;
use crate::Result;
use crate::StringMap;
use crate::StyleConfig;
use crate::ValueSet;
use crate::VariableGroup;
use crate::WrappedPluginConfig;

pub struct SkribbleRunner {
  options: Arc<Options>,
  config: Arc<WrappedPluginConfig>,
  plugins: Arc<Mutex<Vec<BoxedPlugin>>>,
  merged_config: Option<MergedConfig>,
}

impl SkribbleRunner {
  pub fn new(config: StyleConfig) -> Self {
    let (options, wrapped_config, mut plugins) = config.into_wrapped_config();
    let options = Arc::new(options);
    let config = Arc::new(wrapped_config);

    // Extract the plugins from the config and sort them by priority.
    plugins.sort_by_priority();
    let plugins = Arc::new(Mutex::new(plugins.extract_plugins()));

    Self {
      options,
      config,
      plugins,
      merged_config: None,
    }
  }

  /// Run the plugins to mutate the config and get the transformed config which
  /// is used.
  pub fn run(&mut self) -> Result<()> {
    self.provide_options_to_plugins()?;
    let config_from_plugins = self.generate_wrapped_config()?;
    self.merge(config_from_plugins);

    // TODO ignoring options around how the config should be extended for now.

    Ok(())
  }

  /// Provide options to the plugins.
  fn provide_options_to_plugins(&mut self) -> Result<()> {
    let options = self.options.as_ref();
    let mut plugins = self.plugins.lock().unwrap();

    for boxed_plugin in plugins.iter_mut() {
      let plugin = boxed_plugin.as_mut();
      plugin.read_options(options).map_err(|e| {
        Error::PluginReadConfigError {
          id: plugin.get_id(),
          source: e,
        }
      })?;
    }

    Ok(())
  }

  /// Run the generate functions on all plugins with the provided merged
  /// configuration.
  pub fn generate(&self) -> Result<GeneratedFiles> {
    let Some(ref config) = self.merged_config else {
      return Err(Error::RunnerNotSetup);
    };

    let plugins = self.plugins.lock().unwrap();
    let mut generated_files = GeneratedFiles::default();

    for boxed_plugin in plugins.iter() {
      let plugin = boxed_plugin.as_ref();
      let generated = plugin.generate_code(config).map_err(|e| {
        Error::PluginGenerateCodeError {
          id: plugin.get_id(),
          source: e,
        }
      })?;

      generated_files.merge(generated);
    }

    Ok(generated_files)
  }

  fn generate_wrapped_config(&self) -> Result<WrappedPluginConfig> {
    let mut wrapped_config = WrappedPluginConfig::default();
    let plugins = self.plugins.lock().unwrap();

    for boxed_plugin in plugins.iter() {
      let plugin = boxed_plugin.as_ref();
      plugin
        .mutate_config(&mut wrapped_config, &self.options)
        .map_err(|e| {
          Error::PluginMutateConfigError {
            id: plugin.get_id(),
            source: e,
          }
        })?;
    }

    Ok(wrapped_config)
  }

  fn merge(&mut self, mut wrapped_config: WrappedPluginConfig) {
    // mutate
    wrapped_config
      .keyframes
      .extend(self.config.keyframes.clone());
    wrapped_config
      .variables
      .extend(self.config.variables.clone());
    wrapped_config
      .media_queries
      .extend(self.config.media_queries.clone());
    wrapped_config
      .modifiers
      .extend(self.config.modifiers.clone());
    wrapped_config.atoms.extend(self.config.atoms.clone());
    wrapped_config.classes.extend(self.config.classes.clone());
    wrapped_config.layers.extend(self.config.layers.clone());

    let mut layers = IndexSet::<String>::new();
    let mut keyframes = IndexMap::<String, Keyframe>::new();
    let mut css_variables = IndexMap::<String, CssVariable>::new();
    let mut media_queries = IndexMap::<String, IndexMap<String, MediaQuery>>::new();
    let mut modifiers = IndexMap::<String, IndexMap<String, Modifier>>::new();
    let mut atoms = IndexMap::<String, Atom>::new();
    let mut classes = IndexMap::<String, NamedClass>::new();
    let mut palette = StringMap::default();
    let mut value_sets = IndexMap::<String, ValueSet>::new();
    let mut groups = IndexMap::<String, VariableGroup>::new();

    // layers
    wrapped_config.layers.sort_by_priority();
    layers.extend(wrapped_config.layers.into_iter().map(|layer| layer.value));

    // keyframes
    wrapped_config
      .keyframes
      .extend(self.config.keyframes.clone());
    for keyframe in wrapped_config.keyframes.into_iter() {
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
    for css_variable in wrapped_config.variables.into_iter() {
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
    let mut wrapped_media_queries = wrapped_config.media_queries;
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
    let mut wrapped_modifiers = wrapped_config.modifiers;
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
    for atom in wrapped_config.atoms.into_iter() {
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
    for class in wrapped_config.classes.into_iter() {
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
    palette.extend(wrapped_config.palette);
    palette.extend(self.config.palette.clone());

    // value_sets
    for value_set in wrapped_config.value_sets.into_iter() {
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

    // groups
    for group in wrapped_config.groups.into_iter() {
      let key = &group.name;

      match groups.get_mut(key) {
        Some(existing) => {
          existing.merge(group);
        }
        None => {
          groups.insert(key.clone(), group);
        }
      }
    }

    // sort by priority
    keyframes.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    css_variables.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    atoms.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    classes.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    value_sets.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    groups.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));

    let mut names = IndexMap::<String, IndexSet<String>>::default();
    let keyframe_names = keyframes.keys().cloned().collect();
    let css_variable_names = css_variables.keys().cloned().collect();
    let atom_names = atoms.keys().cloned().collect();
    let class_names = classes.keys().cloned().collect();
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
    names.insert("media_queries".into(), media_query_names);
    names.insert("modifiers".into(), modifier_names);

    let mut merged_config = MergedConfig::builder()
      .layers(layers)
      .keyframes(keyframes)
      .css_variables(css_variables)
      .media_queries(media_queries)
      .modifiers(modifiers)
      .atoms(atoms)
      .classes(classes)
      .palette(palette)
      .value_sets(value_sets)
      .groups(groups)
      .names(names)
      ._options(self.options.clone())
      .build();

    for (name, atom) in merged_config.atoms.iter() {
      let name_atom_name = get_atom_name_lookup_name(name);
      let atom_names = atom.values.get_names_from_config(&merged_config);
      merged_config.names.insert(name_atom_name, atom_names);
    }

    self.merged_config = Some(merged_config);
  }
}

/// The configuration after all plugins have been run.
#[derive(Clone, Debug, Default, Deserialize, Serialize, TypedBuilder)]
pub struct MergedConfig {
  pub layers: IndexSet<String>,
  pub keyframes: IndexMap<String, Keyframe>,
  pub css_variables: IndexMap<String, CssVariable>,
  pub media_queries: IndexMap<String, IndexMap<String, MediaQuery>>,
  pub modifiers: IndexMap<String, IndexMap<String, Modifier>>,
  pub atoms: IndexMap<String, Atom>,
  pub classes: IndexMap<String, NamedClass>,
  pub palette: StringMap,
  pub value_sets: IndexMap<String, ValueSet>,
  pub groups: IndexMap<String, VariableGroup>,
  #[builder(default)]
  pub names: IndexMap<String, IndexSet<String>>,
  #[serde(skip)]
  _options: Arc<Options>,
}

impl MergedConfig {
  pub fn has_media_query(&self, name: impl AsRef<str>) -> bool {
    let name = name.as_ref().to_string();
    self
      .names
      .get("media_queries")
      .as_ref()
      .map(|map| map.contains(&name))
      .unwrap_or(false)
  }

  pub fn has_keyframe(&self, name: impl AsRef<str>) -> bool {
    let name = name.as_ref().to_string();
    self
      .names
      .get("keyframes")
      .as_ref()
      .map(|map| map.contains(&name))
      .unwrap_or(false)
  }

  pub fn has_css_variable(&self, name: impl AsRef<str>) -> bool {
    let name = name.as_ref().to_string();
    self
      .names
      .get("css_variables")
      .as_ref()
      .map(|map| map.contains(&name))
      .unwrap_or(false)
  }

  pub fn has_atom(&self, name: impl AsRef<str>) -> bool {
    let name = name.as_ref().to_string();
    self
      .names
      .get("atoms")
      .as_ref()
      .map(|map| map.contains(&name))
      .unwrap_or(false)
  }

  pub fn has_class(&self, name: impl AsRef<str>) -> bool {
    let name = name.as_ref().to_string();
    self
      .names
      .get("classes")
      .as_ref()
      .map(|map| map.contains(&name))
      .unwrap_or(false)
  }

  pub fn has_modifier(&self, name: impl AsRef<str>) -> bool {
    let name = name.as_ref().to_string();
    self
      .names
      .get("modifiers")
      .as_ref()
      .map(|map| map.contains(&name))
      .unwrap_or(false)
  }

  /// Load the options
  pub fn options(&self) -> &Options {
    &self._options
  }

  pub fn get_media_query_index(&self, name: impl AsRef<str>) -> Option<usize> {
    self
      .names
      .get("media_queries")
      .and_then(|map| map.get_index_of(name.as_ref()))
  }

  pub fn get_modifier_index(&self, name: impl AsRef<str>) -> Option<usize> {
    self
      .names
      .get("modifiers")
      .and_then(|map| map.get_index_of(name.as_ref()))
  }

  pub fn get_atom_index(&self, name: impl AsRef<str>) -> Option<usize> {
    self
      .names
      .get("atoms")
      .and_then(|map| map.get_index_of(name.as_ref()))
  }

  pub fn get_named_class_index(&self, name: impl AsRef<str>) -> Option<usize> {
    self
      .names
      .get("classes")
      .and_then(|map| map.get_index_of(name.as_ref()))
  }

  pub fn get_atom_values_index(
    &self,
    atom_name: impl AsRef<str>,
    value_name: impl AsRef<str>,
  ) -> Option<usize> {
    let lookup_name = get_atom_name_lookup_name(atom_name);
    self
      .names
      .get(&lookup_name)
      .and_then(|map| map.get_index_of(value_name.as_ref()))
  }

  pub fn get_atom_is_keyframe(&self, name: impl AsRef<str>) -> bool {
    self
      .atoms
      .get(name.as_ref())
      .map(|atom| atom.values == LinkedValues::Keyframes)
      .unwrap_or(false)
  }
}

fn get_atom_name_lookup_name(atom_name: impl AsRef<str>) -> String {
  format!("atom:{}", atom_name.as_ref())
}
