use skribble_core::*;

use super::*;

#[test]
fn default_can_be_added_to_runner() -> AnyEmptyResult {
  let plugin = PresetDefault::builder().build();

  let config: StyleConfig = StyleConfig::builder()
    .plugins(vec![PluginContainer::from(plugin)])
    .build();

  let mut runner = SkribbleRunner::new(config);
  let runner_config = runner.initialize()?;
  insta::assert_debug_snapshot!(runner_config);

  Ok(())
}
