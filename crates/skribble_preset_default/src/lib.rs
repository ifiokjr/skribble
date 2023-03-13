#![deny(clippy::all)]

use data::*;
use enums::*;
use serde::Deserialize;
use serde::Serialize;
use skribble_core::*;
use typed_builder::TypedBuilder;

pub(crate) mod data;
mod enums;

#[derive(Debug, Clone, Default, Deserialize, TypedBuilder, Serialize)]
pub struct PresetDefault {
  /// Choose the palette colors from either Tailwind or OpenColors.
  #[builder(default, setter(into))]
  pub palette: PaletteType,
  /// Choose how `light` and `dark` mode are handled. via `prefers-color-scheme`
  /// or `class` based.
  #[builder(default, setter(into))]
  pub dark_mode: DarkMode,
  /// Choose to ignore the color variables so that you can define your own
  /// custom theme.
  #[builder(default, setter(into))]
  pub ignore_colors: bool,
}

impl Plugin for PresetDefault {
  fn get_id(&self) -> String {
    "skribble_preset_default".into()
  }

  fn mutate_config(&self, config: &mut WrappedPluginConfig) -> AnyResult {
    self.update_palette(&mut config.palette);
    self.update_media_queries(&mut config.media_queries);
    self.update_parent_modifiers(&mut config.parent_modifiers);
    self.update_modifiers(&mut config.modifiers);
    self.update_css_variables(&mut config.css_variables);
    self.update_keyframes(&mut config.keyframes);
    self.update_atoms(&mut config.atoms);
    self.update_groups(&mut config.groups);
    self.update_named_classes(&mut config.named_classes);
    self.update_value_sets(&mut config.value_sets);

    Ok(())
  }

  fn get_name(&self) -> String {
    self.get_id()
  }

  fn get_description(&self) -> String {
    "".into()
  }
}

impl PresetDefault {
  fn update_media_queries(&self, media_queries: &mut MediaQueries) {
    media_queries.extend(MEDIA_QUERIES.clone());

    if self.dark_mode == DarkMode::Media {
      media_queries.extend(DARK_MEDIA_QUERIES.clone());
    }
  }

  fn update_palette(&self, palette: &mut Palette) {
    palette.extend(self.palette.palette());
  }

  fn update_parent_modifiers(&self, parent_modifiers: &mut ParentModifiers) {
    if self.dark_mode == DarkMode::Class {
      parent_modifiers.extend(DARK_PARENT_MODIFIERS.clone());
    }

    parent_modifiers.extend(PARENT_MODIFIERS.clone());
  }

  fn update_modifiers(&self, modifiers: &mut Modifiers) {
    modifiers.extend(MODIFIERS.clone());
  }

  fn update_css_variables(&self, css_variables: &mut CssVariables) {
    if !self.ignore_colors {
      css_variables.extend(COLOR_CSS_VARIABLES.clone());
    }

    css_variables.extend(CSS_VARIABLES.clone());
  }

  fn update_keyframes(&self, keyframes: &mut Keyframes) {
    keyframes.extend(KEYFRAMES.clone());
  }

  fn update_atoms(&self, atoms: &mut Atoms) {
    atoms.extend(ATOMS.clone());
  }

  fn update_groups(&self, groups: &mut VariableGroups) {
    groups.extend(GROUPS.clone());
  }

  fn update_named_classes(&self, named_classes: &mut NamedClasses) {
    named_classes.extend(NAMED_CLASSES.clone());
  }

  fn update_value_sets(&self, value_sets: &mut ValueSets) {
    value_sets.extend(ATOM_VALUE_SETS.clone());
    value_sets.extend(ANIMATION_VALUE_SETS.clone());
  }
}

#[cfg(test)]
mod tests {
  use skribble_core::*;

  use super::*;

  #[test]
  fn default_can_be_added_to_runner() {
    let plugin = PresetDefault::builder().build();

    let config: StyleConfig = StyleConfig::builder()
      .plugins(vec![PluginContainer::from(plugin)])
      .build();

    let mut runner = SkribbleRunner::new(config);
    let _ = runner.run();
  }
}
