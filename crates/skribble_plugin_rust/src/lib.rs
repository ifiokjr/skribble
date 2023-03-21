#![deny(clippy::all)]
#![deny(clippy::indexing_slicing)]

use generate::*;
use heck::ToPascalCase;
use heck::ToSnakeCase;
use indexmap::indexmap;
use indexmap::indexset;
use indexmap::IndexMap;
use indexmap::IndexSet;
use indoc::indoc;
use serde::Deserialize;
use serde::Serialize;
use skribble_core::*;
use typed_builder::TypedBuilder;

mod generate;

/// This plugin generates `rust` code from the configuration.
#[derive(Debug, Clone, Default, Deserialize, TypedBuilder, Serialize)]
pub struct RustPlugin {
  /// The formatter command.
  /// e.g. `dprint`
  #[builder(default, setter(into, strip_option))]
  pub formatter: Option<String>,

  /// The formatter arguments.
  /// e.g. `["fmt", "--stdin", "file.rs"]`
  #[builder(default, setter(into))]
  pub formatter_args: Vec<String>,
}

impl Plugin for RustPlugin {
  fn get_id(&self) -> String {
    "skribble_plugin_rust".into()
  }

  fn generate_code(&self, config: &MergedConfig, options: &Options) -> AnyResult<GeneratedFiles> {
    let mut files = GeneratedFiles::default();
    files.push(
      GeneratedFile::builder()
        .path("./src/skribble.rs")
        .content(self.generate_file_contents(config, options))
        .build(),
    );

    Ok(files)
  }

  fn get_description(&self) -> String {
    "This plugin provides support for generating rust code from your `skribble` configuration."
      .into()
  }
}

impl RustPlugin {
  fn generate_file_contents(&self, config: &MergedConfig, options: &Options) -> String {
    let mut method_names: IndexSet<String> = indexset! {};
    let indent_style = IndentStyle::default();
    let mut sections = Vec::<String>::new();
    let mut trait_names = vec![];
    let mut struct_names_map: IndexMap<String, usize> = indexmap! { "SkribbleRoot".into() => 0 };

    generate_css_variables(
      config,
      &options.variable_prefix,
      indent_style,
      &mut sections,
    );

    // media queries
    generate_media_queries(
      config,
      indent_style,
      &mut method_names,
      &mut sections,
      &mut struct_names_map,
      &mut trait_names,
    );

    generate_parent_modifiers(
      config,
      indent_style,
      &mut method_names,
      &mut sections,
      &mut struct_names_map,
      &mut trait_names,
    );

    generate_modifiers(
      config,
      indent_style,
      &mut method_names,
      &mut sections,
      &mut struct_names_map,
      &mut trait_names,
    );

    generate_value_sets(config, indent_style, &mut sections);
    generate_palette(config, indent_style, &mut sections);

    generate_atoms(
      config,
      indent_style,
      &mut method_names,
      &mut sections,
      &mut trait_names,
    );

    generate_named_classes(config, indent_style, &mut sections, &mut trait_names);

    // Add the implementation for each of the structs.
    generate_struct_implementations(&struct_names_map, &trait_names, &mut sections);
    combine_sections_with_header(sections)
  }
}

#[cfg(test)]
mod tests {
  use skribble_core::*;
  use skribble_preset_default::PresetDefault;

  use super::*;

  #[test]
  fn default_can_be_added_to_runner() {
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
    insta::assert_display_snapshot!(content);
  }
}
