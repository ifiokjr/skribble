use std::fmt::Write;

use crate::AnyEmptyResult;
use crate::AnyResult;
use crate::RunnerConfig;

pub trait ToSkribbleCss {
  /// Appends css to the provided writer when also provided with the
  /// configuration for the active runner.
  fn write_skribble_css(&self, writer: &mut dyn Write, config: &RunnerConfig) -> AnyEmptyResult;

  /// Returns the css as a string when also provided with the configuration for
  /// the runner.
  fn to_skribble_css(&self, config: &RunnerConfig) -> AnyResult<String> {
    let mut writer = String::new();
    self.write_skribble_css(&mut writer, config)?;
    Ok(writer)
  }
}
