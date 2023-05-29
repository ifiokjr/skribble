use std::env;
use std::fs;

use skribble_core::*;
use skribble_preset::PresetPlugin;
use skribble_rust::RustPlugin;

fn main() -> AnyEmptyResult {
  let default_preset = PresetPlugin::builder().build();
  let rust_plugin = RustPlugin::builder().build();
  let options = Options::builder()
    .formatters(vec![
      Formatter::builder()
        .command("dprint")
        .args(vec!["fmt"])
        .globs(vec!["**/*.rs"])
        .build(),
    ])
    .build();

  let config: StyleConfig = StyleConfig::builder()
    .plugins(vec![
      PluginContainer::from(default_preset),
      PluginContainer::from(rust_plugin),
    ])
    .options(options)
    .build();

  let mut runner = SkribbleRunner::try_new(config)?;
  let _ = runner.initialize();
  let result = runner.generate()?;
  let generated = result.first().unwrap();
  let content = &generated.content;

  let current_dir = env::current_dir()?;
  let first_arg = env::args().nth(1).unwrap();
  let path = current_dir.join(&first_arg).join("generated_code.rs");

  fs::create_dir_all(&first_arg)?;
  std::fs::write(path, content)?;

  Ok(())
}
