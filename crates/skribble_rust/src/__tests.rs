use skribble_core::*;
use skribble_preset::PresetPlugin;

use super::*;

#[test]
fn default_can_be_added_to_runner() {
  let default_preset = PresetPlugin::builder().build();
  let rust_plugin = RustPlugin::builder().build();

  let config: StyleConfig = StyleConfig::builder()
    .plugins(vec![
      PluginContainer::from(default_preset),
      PluginContainer::from(rust_plugin),
    ])
    .build();

  let mut runner = SkribbleRunner::new(config, "/", None);
  let _ = runner.initialize();
  let result = runner.generate().unwrap();
  let generated = result.first().unwrap();
  let content = &generated.content;
  insta::assert_display_snapshot!(content);
}
