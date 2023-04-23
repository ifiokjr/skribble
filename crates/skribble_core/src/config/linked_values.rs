use std::fmt::Write;

use indexmap::indexset;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;

use super::Atom;
use super::NameSet;
use crate::AnyEmptyResult;
use crate::Arguments;
use crate::Placeholder;
use crate::Prioritized;
use crate::RunnerConfig;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LinkedValues {
  /// The [`ValueSet`] names that will be used to populate the names that can be
  /// used.
  Values(NameSet),
  /// The atom will be linked to colors and the settings determine how the link
  /// is made.
  Color,
  /// The atom will be linked to all the `keyframes` that are available. This is
  /// used to generate the `animate` class name.
  Keyframes,
}

impl LinkedValues {
  pub fn get_names_from_config(&self, config: &RunnerConfig) -> IndexSet<String> {
    match self {
      Self::Values(ref value_set) => {
        let mut names = indexset! {};
        for value in value_set.iter() {
          if let Some(set) = config.value_sets.get(&value.value) {
            names.extend(set.values.keys().cloned());
          }
        }

        names
      }
      Self::Color => {
        let mut names = indexset! {};

        for (name, variable) in config.css_variables.iter() {
          if variable.is_color() {
            names.insert(name.to_owned());
          }
        }
        names.extend(config.palette.keys().cloned());

        names
      }
      Self::Keyframes => config.keyframes.keys().cloned().collect(),
    }
  }

  pub fn merge(&mut self, other: Self) {
    match self {
      Self::Values(value_set) => {
        if let Self::Values(other_value_set) = other {
          value_set.merge(other_value_set);
          value_set.sort_from_highest_priority();
        }
      }
      Self::Color => {
        *self = other;
      }
      Self::Keyframes => {
        *self = other;
      }
    }
  }

  pub fn write_css_properties(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    atom: &Atom,
    name: impl AsRef<str>,
  ) -> AnyEmptyResult {
    match self {
      Self::Values(ref value_set) => {
        for Prioritized { value: key, .. } in value_set.iter() {
          if let Some(css_value) = config
            .value_sets
            .get(key)
            .and_then(|value_set| value_set.values.get(name.as_ref()))
          {
            css_value.write_css(writer, config, atom)?;
            break;
          }
        }
      }
      Self::Color => {
        for (color_name, variable) in config.css_variables.iter() {
          if !variable.is_color() || name.as_ref() != color_name {
            continue;
          }
          let options = config.options();
          let color_value = options
            .color_format
            .get_color_with_parts_and_opacity(variable, options);
          // let opacity_variable =
          //   Placeholder::normalize(variable.get_opacity_variable(config.options()),
          // config); let default_opacity = variable.get_default_opacity(None);
          // writeln!(writer, "{opacity_variable}: {default_opacity};")?;

          for (property, css_value) in atom.styles.iter() {
            let property = Placeholder::normalize(property, config);
            let css_value = css_value
              .as_ref()
              .map(|value| Placeholder::normalize_value(value, &color_value, config))
              .unwrap_or_else(|| color_value.clone());

            writeln!(writer, "{}: {};", property, css_value)?;
          }

          break;
        }
      }
      Self::Keyframes => {
        for (keyframe_name, _keyframe) in config.keyframes.iter() {
          if name.as_ref() != keyframe_name {
            continue;
          }

          for (property, css_value) in atom.styles.iter() {
            let property = Placeholder::normalize(property, config);
            let css_value = css_value
              .as_ref()
              .map(|value| Placeholder::normalize_value(value, keyframe_name, config))
              .unwrap_or_else(|| keyframe_name.clone());

            writeln!(writer, "{}: {};", property, css_value)?;
          }

          break;
        }
      }
    }

    Ok(())
  }

  pub fn write_css_argument(
    &self,
    writer: &mut dyn Write,
    config: &RunnerConfig,
    atom: &Atom,
    argument: &Arguments,
  ) -> AnyEmptyResult {
    argument.write_css_atom(writer, config, atom)?;
    Ok(())
  }

  pub fn collect_css_variables(
    &self,
    config: &RunnerConfig,
    name: impl AsRef<str>,
    css_variables: &mut IndexSet<String>,
  ) {
    match self {
      Self::Values(ref value_set) => {
        for Prioritized { value: key, .. } in value_set.iter() {
          if let Some(css_value) = config
            .value_sets
            .get(key)
            .and_then(|value_set| value_set.values.get(name.as_ref()))
          {
            css_value.collect_css_variables(css_variables);
            break;
          }
        }
      }
      Self::Color => {
        css_variables.insert(name.as_ref().to_owned());
      }
      Self::Keyframes => {
        for (keyframe_name, keyframe) in config.keyframes.iter() {
          if name.as_ref() != keyframe_name {
            continue;
          }

          keyframe.collect_css_variables(css_variables);
          break;
        }
      }
    }
  }
}

impl Default for LinkedValues {
  fn default() -> Self {
    Self::Values(NameSet::default())
  }
}

impl<V: Into<NameSet>> From<V> for LinkedValues {
  fn from(value: V) -> Self {
    Self::Values(value.into())
  }
}
