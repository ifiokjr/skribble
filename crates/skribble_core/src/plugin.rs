use std::fmt::Debug;

use crate::Result;
use crate::StyleConfig;

pub trait Plugin: Debug {
  /// Get the id of the plugin. This should be globally unique and can be the
  /// published crate_name of the plugin.
  fn get_id(&self) -> &str;

  /// Mutate the [`StyleConfig`] inline.
  fn update_style_config(&mut self, _config: &mut StyleConfig) -> Result<()> {
    Ok(())
  }

  /// Set a readable name of the plugin. This is used for error messages and
  /// serialization.
  ///
  /// It defaults to the id of the plugin.
  fn get_name(&self) -> &str {
    self.get_id()
  }

  /// Get the markdown description of the plugin. Defaults to an empty string.
  fn get_description(&self) -> &str {
    ""
  }
}

impl<P: Plugin + 'static> From<P> for Box<dyn Plugin> {
  fn from(plugin: P) -> Self {
    Box::new(plugin)
  }
}
