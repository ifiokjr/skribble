use std::env;
use std::fs;

use skribble_core::*;
use skribble_preset::PresetDefault;
use skribble_rust::RustPlugin;

fn main() {
  let default_preset = PresetDefault::builder().build();
  let rust_plugin = RustPlugin::builder()
    .formatter("dprint")
    .formatter_args(["fmt".into(), "--stdin".into(), "file.rs".into()])
    .build();

  let config: StyleConfig = StyleConfig::builder()
    .plugins(vec![
      PluginContainer::from(default_preset),
      PluginContainer::from(rust_plugin),
    ])
    .build();

  let mut runner = SkribbleRunner::new(config);
  let _ = runner.initialize();
  let result = runner.generate().unwrap();
  let generated = result.first().unwrap();
  let content = &generated.content;

  let current_dir = env::current_dir().unwrap();
  let first_arg = env::args().nth(1).unwrap();
  let path = current_dir.join(&first_arg).join("generated_code.rs");

  fs::create_dir_all(&first_arg).unwrap();
  std::fs::write(path, content).unwrap();
}
