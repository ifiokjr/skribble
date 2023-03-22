use std::sync::Arc;
use std::sync::Mutex;

use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::AdditionalFields;
use crate::Atom;
use crate::BoxedPlugin;
use crate::CssVariable;
use crate::Error;
use crate::GeneratedFiles;
use crate::Keyframe;
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
  /// This is only available once the runner has been set up.
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
      let generated = plugin.generate_code(config, &self.options).map_err(|e| {
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

  fn merge(&mut self, wrapped_config: WrappedPluginConfig) {
    // let merge_rules = &self.config.options.merge_rules;
    let mut keyframes = IndexMap::<String, Keyframe>::new();
    let mut css_variables = IndexMap::<String, CssVariable>::new();
    let mut media_queries = IndexMap::<String, IndexMap<String, MediaQuery>>::new();
    let mut modifiers = IndexMap::<String, IndexMap<String, Modifier>>::new();
    let mut atoms = IndexMap::<String, Atom>::new();
    let mut classes = IndexMap::<String, NamedClass>::new();
    let mut palette = StringMap::default();
    let mut value_sets = IndexMap::<String, ValueSet>::new();
    let mut groups = IndexMap::<String, VariableGroup>::new();
    let mut additional_fields = AdditionalFields::default();

    // keyframes
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

    // additional_fields
    additional_fields.extend(wrapped_config.additional_fields);
    additional_fields.extend(self.config.additional_fields.clone());

    // sort by priority
    keyframes.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    css_variables.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    atoms.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    classes.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    value_sets.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));
    groups.sort_by(|_, a_value, _, z_value| z_value.priority.cmp(&a_value.priority));

    self.merged_config = Some(
      MergedConfig::builder()
        .keyframes(keyframes)
        .css_variables(css_variables)
        .media_queries(media_queries)
        .modifiers(modifiers)
        .atoms(atoms)
        .classes(classes)
        .palette(palette)
        .value_sets(value_sets)
        .groups(groups)
        .additional_fields(additional_fields)
        .build(),
    );
  }
}

/// The configuration after all plugins have been run.
#[derive(Clone, Debug, Default, Deserialize, Serialize, TypedBuilder)]
pub struct MergedConfig {
  pub keyframes: IndexMap<String, Keyframe>,
  pub css_variables: IndexMap<String, CssVariable>,
  pub media_queries: IndexMap<String, IndexMap<String, MediaQuery>>,
  pub modifiers: IndexMap<String, IndexMap<String, Modifier>>,
  pub atoms: IndexMap<String, Atom>,
  pub classes: IndexMap<String, NamedClass>,
  pub palette: StringMap,
  pub value_sets: IndexMap<String, ValueSet>,
  pub groups: IndexMap<String, VariableGroup>,
  pub additional_fields: AdditionalFields,
}
