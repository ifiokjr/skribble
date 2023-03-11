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

  fn mutate_config(&mut self, config: &mut ConfigEnum) -> AnyResult {
    match config {
      ConfigEnum::Palette(ref mut palette) => self.update_palette(palette),
      ConfigEnum::MediaQueries(ref mut media_queries) => self.update_media_queries(media_queries),
      ConfigEnum::ParentModifiers(ref mut parent) => self.update_parent_modifiers(parent),
      ConfigEnum::Modifiers(ref mut modifiers) => self.update_modifiers(modifiers),
      ConfigEnum::CssVariables(ref mut css_variables) => self.update_css_variables(css_variables),
      ConfigEnum::Keyframes(ref mut keyframes) => self.update_keyframes(keyframes),
      ConfigEnum::Atoms(ref mut atoms) => self.update_atoms(atoms),
      ConfigEnum::Groups(ref mut groups) => self.update_groups(groups),
      ConfigEnum::NamedClasses(ref mut named_classes) => self.update_named_classes(named_classes),
      ConfigEnum::ValueSets(ref mut value_sets) => self.update_value_sets(value_sets),
      _ => {}
    }

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
