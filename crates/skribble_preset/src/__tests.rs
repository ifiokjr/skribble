use rstest::*;
use skribble_core::vfs::MemoryFS;
use skribble_core::*;
use skribble_test::set_snapshot_suffix;

use super::*;

#[test]
fn default_can_be_added_to_runner() -> AnyEmptyResult {
  let plugin = PresetPlugin::builder().reset("tailwindCompat").build();
  let config: StyleConfig = StyleConfig::builder()
    .plugins(vec![PluginContainer::from(plugin)])
    .build();

  let mut runner = SkribbleRunner::try_new(config)?;
  let runner_config = runner.initialize()?;
  insta::assert_json_snapshot!(runner_config);

  Ok(())
}

#[rstest]
#[case("contained", &["$contained", "lg:$contained", "xl:$contained"])]
#[case("aspect-ratio", &["aspect:$square", "aspect:$portrait", "aspect:[2/1]"])]
#[case("atom-modifiers", &["space-x:$px", "md:space-y:$64", "$space-x-reverse"])]
#[case("modifiers", &["hover:bg:$primary"])]
#[case("chained-modifiers", &["rtl:required:hover:bg:$primary"])]
#[case("font-size", &["font-size:$xs", "font-size:$8xl"])]
#[case("important", &["(important):font-size:$xs", "(important):font-size:$8xl"])]
#[case("darken", &["(darken==005):bg:red100", "(darken=5%):bg:pink900", "(darken==050):bg:primary", "(darken=50%):bg:secondary"])]
#[case("lighten", &["(lighten==005):bg:red100", "(lighten=5%):bg:pink900", "(lighten==050):bg:primary", "(lighten=50%):bg:secondary"])]
#[case("alpha", &["(alpha==005):bg:red100", "(alpha=0.05):bg:pink900", "(alpha==050):bg:primary", "(alpha=0.5):bg:secondary"])]
fn css_from_class_names(#[case] id: &str, #[case] names: &[&str]) -> AnyEmptyResult {
  let plugin = PresetPlugin::default();
  let config: StyleConfig = StyleConfig::builder()
    .plugins(vec![PluginContainer::from(plugin)])
    .build();
  let mut runner = SkribbleRunner::try_new(config)?;
  let runner_config = runner.initialize()?;
  let mut classes = Classes::default();

  for name in names.iter() {
    classes.insert_factory(ClassFactory::from_string(runner_config, name));
  }

  classes.sort_by_class();

  set_snapshot_suffix!("{id}");
  insta::assert_display_snapshot!(classes.to_skribble_css(runner_config)?);

  Ok(())
}

#[test]
fn auto_generate_reset_css() -> AnyEmptyResult {
  let plugin = PresetPlugin::builder().reset("tailwindCompat").build();
  let config: StyleConfig = StyleConfig::builder()
    .plugins(vec![PluginContainer::from(plugin)])
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

const FILES: &[(&str, &str)] = &[];
