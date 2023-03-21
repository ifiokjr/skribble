use std::env;
use std::fs;

use skribble_core::*;
use skribble_plugin_rust::RustPlugin;
use skribble_preset_default::PresetDefault;

fn main() {
  let default_preset = PresetDefault::builder().build();
  let rust_plugin = RustPlugin::builder().build();

  let config: StyleConfig = StyleConfig::builder()
    .plugins(vec![
      PluginContainer::from(default_preset),
      PluginContainer::from(rust_plugin),
    ])
    .build();

  let mut runner = SkribbleRunner::new(config);
  let _ = runner.run();
  let result = runner.generate().unwrap();
  let generated = result.get(0).unwrap();
  let content = &generated.content;

  let current_dir = env::current_dir().unwrap();
  let first_arg = env::args().nth(1).unwrap();
  let path = current_dir.join(&first_arg).join("generated_code.rs");

  fs::create_dir_all(&first_arg).unwrap();
  std::fs::write(path, content).unwrap();
}
