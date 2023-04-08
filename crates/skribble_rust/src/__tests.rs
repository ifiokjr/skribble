use skribble_core::vfs::MemoryFS;
use skribble_core::*;
use skribble_preset::PresetPlugin;

use super::*;

#[test]
fn can_generate_skribble_rust_code() -> AnyEmptyResult {
  let default_preset = PresetPlugin::builder().build();
  let rust_plugin = RustPlugin::builder().build();

  let config: StyleConfig = StyleConfig::builder()
    .plugins(vec![
      PluginContainer::from(default_preset),
      PluginContainer::from(rust_plugin),
    ])
    .build();

  let vfs: VfsPath = create_memory_fs()?;
  let mut runner = SkribbleRunner::new(config, "/", Some(vfs));
  let _ = runner.initialize()?;
  let result = runner.generate()?;
  let GeneratedFile { content, .. } = result.first().ok_or(Error::Unknown)?;
  insta::assert_display_snapshot!(content);

  Ok(())
}

#[test]
fn can_scan_and_generate_css() -> AnyEmptyResult {
  let default_preset = PresetPlugin::builder().build();
  let rust_plugin = RustPlugin::builder().build();

  let config: StyleConfig = StyleConfig::builder()
    .plugins(vec![
      PluginContainer::from(default_preset),
      PluginContainer::from(rust_plugin),
    ])
    .build();

  let vfs: VfsPath = create_memory_fs()?;
  let mut runner = SkribbleRunner::new(config, "/", Some(vfs));
  let _ = runner.initialize()?;
  let scanned = runner.scan()?;
  insta::assert_display_snapshot!(scanned.code);

  Ok(())
}

fn create_memory_fs() -> AnyResult<VfsPath> {
  let vfs: VfsPath = MemoryFS::new().into();

  for file in FILES {
    let path = vfs.join(file.0)?;
    path.create_dir_all()?;
    let mut writer = path.create_file()?;
    write!(writer, "{}", file.1)?;
  }

  Ok(vfs)
}

const FILES: &[(&str, &str)] = &[
  (
    "src/lib.rs",
    r#"
use leptos::*;
use crate::skribble::*;
#[component]
fn App(cx: Scope) -> impl IntoView {
  let s = sk().md().p().px();
  let a = sk().bg().secondary();
  let classes = vec![sk().bg().secondary(), sk().p().px()].join(" ");

  view! {
    cx,
    <div class={classes}>
      <h1 class={sk().text().color().primary()}>Hello World</h1>
    </div>
  }
}

mod other;
"#,
  ),
  (
    "src/other.rs",
    r#"
use crate::skribble::*;
pub fn other() -> String {
  sk().bg().primary()
}
  "#,
  ),
];
