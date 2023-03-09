#![deny(clippy::all)]

pub use base::PaletteType;
use base::*;
use skribble_core::*;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Default, TypedBuilder)]
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
        update_palette(palette, self.palette);
      }
      ConfigEnum::MediaQueries(ref mut media_queries) => {
        update_media_queries(media_queries, self.dark_mode);
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

mod base;
