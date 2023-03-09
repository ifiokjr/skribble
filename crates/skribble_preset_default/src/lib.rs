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
}

impl Plugin for PresetDefault {
  fn get_id(&self) -> String {
    "skribble_preset_default".into()
  }

  fn mutate_config(&mut self, config: &mut ConfigEnum) -> AnyResult {
    match config {
      ConfigEnum::Palette(ref mut palette) => {
        self.update_palette(palette);
      }
      ConfigEnum::MediaQueries(ref mut media_queries) => {
        self.update_media_queries(media_queries);
      }
      ConfigEnum::ParentModifiers(ref mut parent_modifiers) => {
        self.update_parent_modifiers(parent_modifiers);
      }
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
}
