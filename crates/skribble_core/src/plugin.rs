use std::fmt::Debug;

use crate::Priority;
use crate::Result;
use crate::StyleConfig;

pub trait Plugin: Debug {
  /// Mutate the [`StyleConfig`] inline.
  fn update_style_config(&mut self, _config: &mut StyleConfig) -> Result<()> {
    Ok(())
  }

  /// Get the priority of this plugin which will be used to determine the order
  /// in which plugins are loaded. You can add the priority as a field to your
  /// plugin struct and return it here.
  fn get_priority(&self) -> Priority {
    Default::default()
  }

  /// Get the name of the plugin.
  fn get_name(&self) -> &str;

  /// Get the markdown description of the plugin.
  fn get_description(&self) -> &str;
}

impl<P: Plugin + 'static> From<P> for Box<dyn Plugin> {
  fn from(plugin: P) -> Self {
    Box::new(plugin)
  }
}
