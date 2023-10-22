use hex::ToHex;
use rstest::rstest;
use skribble_core::vfs::MemoryFS;
use skribble_core::*;
use skribble_preset::PresetPlugin;
use skribble_test::set_snapshot_suffix;

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

	let vfs: VfsPath = MemoryFS::new().into();
	let mut runner = SkribbleRunner::new(config, "/", Some(vfs));
	let _ = runner.initialize()?;
	let result = runner.generate()?;
	let GeneratedFile { content, .. } = result.first().ok_or(Error::Unknown)?;
	insta::assert_display_snapshot!(content);

	Ok(())
}

#[test]
fn can_generate_skribble_rust_methods() -> AnyEmptyResult {
	let default_preset = PresetPlugin::builder().build();
	let rust_plugin = RustPlugin::builder().build();

	let config: StyleConfig = StyleConfig::builder()
		.plugins(vec![
			PluginContainer::from(default_preset),
			PluginContainer::from(rust_plugin),
		])
		.build();

	let vfs: VfsPath = MemoryFS::new().into();
	let mut runner = SkribbleRunner::new(config, "/", Some(vfs));
	let _ = runner.initialize()?;
	let result = runner.generate()?;
	let GeneratedFile { content, .. } = result.last().ok_or(Error::Unknown)?;
	insta::assert_display_snapshot!(content);

	Ok(())
}

#[rstest]
#[case("function-default", &[("src/lib.rs", function("default", DEFAULT_NAMES))])]
#[case("basic-component-default", &[("src/lib.rs", basic_component("default", DEFAULT_NAMES))])]
#[case("component-default", &[("src/lib.rs", component("default", DEFAULT_NAMES))])]
#[case("variables-default", &[("src/lib.rs", variables(DEFAULT_NAMES))])]
#[case("gradients", &[("src/lib.rs", GRADIENTS)])]
fn can_scan_and_generate_css<S: AsRef<str>>(
	#[case] id: &str,
	#[case] files: &[(&str, S)],
) -> AnyEmptyResult {
	let default_preset = PresetPlugin::builder().build();
	let rust_plugin = RustPlugin::builder().build();

	let config: StyleConfig = StyleConfig::builder()
		.plugins(vec![
			PluginContainer::from(default_preset),
			PluginContainer::from(rust_plugin),
		])
		.build();

	let vfs: VfsPath = create_memory_fs(files)?;
	let mut runner = SkribbleRunner::new(config, "/", Some(vfs));
	let _ = runner.initialize()?;
	let _ = runner.generate()?;
	let scanned = runner.scan()?;
	set_snapshot_suffix!("{id}");
	insta::assert_display_snapshot!(scanned.code);

	Ok(())
}

fn create_memory_fs<S: AsRef<str>>(files: &[(&str, S)]) -> AnyResult<VfsPath> {
	let vfs: VfsPath = MemoryFS::new().into();

	for file in files {
		let path = vfs.join(file.0)?;
		path.create_dir_all()?;
		let mut writer = path.create_file()?;
		write!(writer, "{}", file.1.as_ref())?;
	}

	Ok(vfs)
}

fn component(name: &str, values: &[&str]) -> String {
	let classes = values.join(", ");
	format!(
		r#"
use leptos::*;
use crate::skribble::*;
#[component]
fn {name}(cx: Scope) -> impl IntoView {{
  view! {{
    cx,
    <div class={{sk().__("width", "50px")}}>
      {{&[{classes}].into_iter()
        .map(|class| view! {{ cx, <span class={{class}}>{{class}}</span> }})
        .collect_view(cx)
      }}
      <span class=sk().xxl().p().n1()>"Hello World"</span>
    </div>
  }}
}}
"#
	)
}

fn basic_component(name: &str, values: &[&str]) -> String {
	let classes = values.join(", ");
	format!(
		r#"
use leptos::*;
use crate::skribble::*;
#[component]
fn {name}(cx: Scope) -> impl IntoView {{
  let classes = &[{classes}].join(" ");
  view! {{
    cx,
    <div class={classes}>
      <h1>"Hello World"</h1>
    </div>
  }}
}}
"#
	)
}

const GRADIENTS: &str = r#"
use crate::skribble::*;

#[component]
fn BackgroundGradient(cx: Scope) -> impl IntoView {{
  let class = &[sk().bg_gradient().to_right(), sk().from_color().cyan500(), sk().to_color().blue500()].join(" ");
  view! {{
    cx,
    <div class=class>"gradient"</div>
  }}
}}
"#;

fn function(name: &str, values: &[&str]) -> String {
	let classes = values.join(", ");
	format!(
		r#"
use crate::skribble::*;
pub fn {name}() -> String {{
  [{classes}].join(" ")
}}
"#
	)
}

fn variables(values: &[&str]) -> String {
	let classes = values
		.iter()
		.map(|value| format!("let _{} = {value};", value.encode_hex::<String>()))
		.collect::<Vec<_>>()
		.join("  \n");
	format!(
		r#"
use crate::skribble::*;
pub fn variables() -> String {{
  {classes}
}}
"#
	)
}

const DEFAULT_NAMES: &[&str] = &[
	r#"sk().md().p().px()"#,
	r#"sk().dark().p().px()"#,
	r#"sk().bg().accent()"#,
	r#"sk().md().pt_("1px")"#,
	r#"sk().md_("padding", "1px")"#,
	r#"sk().screen().md_("padding", "1px")"#,
	r#"sk().p_("101px")"#,
	r#"sk().bg().red100()"#,
	r#"sk().aspect().square()"#,
	r#"sk().__("height", "50px")"#,
	r#"sk().md().darken_050().bg().red100()"#,
	r#"sk().md().alpha("0.5").bg().pink100()"#,
];
