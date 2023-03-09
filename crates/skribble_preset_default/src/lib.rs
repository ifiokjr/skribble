#![deny(clippy::all)]

use palette::*;
use skribble_core::*;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Default, TypedBuilder)]
pub struct PresetDefault {
  #[builder(default, setter(into))]
  palette: PaletteType,
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

mod palette;
