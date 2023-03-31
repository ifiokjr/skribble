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

#[test]
fn contained() -> AnyEmptyResult {
  let plugin = PresetDefault::builder().build();
  let config: StyleConfig = StyleConfig::builder()
    .plugins(vec![PluginContainer::from(plugin)])
    .build();
  let mut runner = SkribbleRunner::new(config);
  let runner_config = runner.initialize()?;
  let mut classes = Classes::default();
  classes.insert_factories(vec![
    ClassFactory::from_tokens(runner_config, &["contained"]),
    ClassFactory::from_tokens(runner_config, &["lg", "contained"]),
  ]);
  classes.sort_by_class();

  insta::assert_display_snapshot!(classes.to_skribble_css(runner_config)?);

  Ok(())
}
