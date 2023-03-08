use std::fmt::Debug;

use crate::Result;
use crate::StyleConfig;

pub trait Plugin: Debug {
  /// Update the configuration inline.
  fn update_config(&mut self, _config: &mut StyleConfig) -> Result<()> {
    Ok(())
  }
}
