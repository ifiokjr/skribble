#![deny(clippy::all)]
#![forbid(clippy::indexing_slicing)]

doc_comment::doctest!("../readme.md");

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
  fn get_data(&self) -> PluginData {
    PluginData::builder()
      .id("skribble_preset")
      .name("Default Preset")
      .description(
        "This plugin provides a default preset for Skribble which is similar to `tailwind`, \
         `windi` and `unocss`.",
      )
      .build()
  }

  fn mutate_config(&mut self, config: &mut PluginConfig, _options: &Options) -> AnyEmptyResult {
    self.update_palette(&mut config.palette);
    self.update_media_queries(&mut config.media_queries);
    self.update_modifiers(&mut config.modifiers);
    self.update_variables(&mut config.variables);
    self.update_keyframes(&mut config.keyframes);
    self.update_atoms(&mut config.atoms);
    self.update_named_classes(&mut config.classes);
    self.update_value_sets(&mut config.value_sets);

    Ok(())
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

  fn update_modifiers(&self, modifiers: &mut Modifiers) {
    modifiers.extend(MODIFIERS.clone());

    if self.dark_mode == DarkMode::Class {
      modifiers.extend_group(DARK_PARENT_MODIFIERS.clone());
    }
  }

  fn update_variables(&self, css_variables: &mut CssVariables) {
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

  fn update_named_classes(&self, named_classes: &mut NamedClasses) {
    named_classes.extend(NAMED_CLASSES.clone());
  }

  fn update_value_sets(&self, value_sets: &mut ValueSets) {
    value_sets.extend(ATOM_VALUE_SETS.clone());
    value_sets.extend(ANIMATION_VALUE_SETS.clone());
  }
}

#[cfg(test)]
mod __tests;
