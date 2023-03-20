#![deny(clippy::all)]
#![deny(clippy::indexing_slicing)]

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

// #[cfg(test)]
mod explore;

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

  fn generate_code(&self, config: &MergedConfig) -> AnyResult<GeneratedFiles> {
    let mut files = GeneratedFiles::default();
    files.push(
      GeneratedFile::builder()
        .path("./src/skribble.rs")
        .content(self.generate_file_contents(config))
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
  fn generate_file_contents(&self, config: &MergedConfig) -> String {
    let mut method_names: IndexSet<String> = indexset! {};
    let indent_style = IndentStyle::default();
    let mut sections = Vec::<String>::new();
    let mut trait_names = vec![];
    let mut struct_names_map: IndexMap<String, usize> = indexmap! {"SkribbleRoot".into() => 0};

    // media queries
    for (key, map) in config.media_queries.iter() {
      let mut section = Vec::<String>::new();
      let trait_name = format!("MediaQuery{}", key.to_pascal_case());
      let struct_name = format!("{trait_name}Child");
      section.push(generate_struct(&struct_name));
      section.push(generate_impl_skribble_value(&struct_name));

      let mut methods = vec![format!("pub trait {trait_name}: SkribbleValue {{")];

      for (name, value) in map.iter() {
        let method_name = get_method_name(name, &mut method_names);

        if let Some(ref description) = value.description {
          let description = description
            .split('\n')
            .collect::<Vec<&str>>()
            .join("\n/// ");
          methods.push(indent(format!("/// {description}"), indent_style));
        }
        methods.push(indent(
          format!("fn {method_name}(&self) -> {struct_name} {{",),
          indent_style,
        ));
        methods.push(indent(
          indent(
            format!("{struct_name}::from_ref(self.append_to_skribble_value(\"{name}\"))"),
            indent_style,
          ),
          indent_style,
        ));
        methods.push(indent("}", indent_style));
      }

      methods.push("}".into());
      section.push(methods.join("\n"));

      trait_names.push(trait_name);
      struct_names_map.insert(struct_name, trait_names.len());
      sections.push(section.join("\n"));
    }

    // parent modifiers
    sections.push(generate_struct(PARENT_MODIFIER_STRUCT_NAME));
    sections.push(generate_impl_skribble_value(PARENT_MODIFIER_STRUCT_NAME));
    sections.push(format!(
      "pub trait {PARENT_MODIFIER_TRAIT_NAME}: SkribbleValue {{"
    ));

    for (name, modifier) in config.parent_modifiers.iter() {
      let method_name = get_method_name(name, &mut method_names);

      if let Some(ref description) = modifier.description {
        let description = description
          .split('\n')
          .collect::<Vec<&str>>()
          .join("\n/// ");
        sections.push(indent(format!("/// {description}"), indent_style));
      }

      sections.push(indent(
        format!("fn {method_name}(&self) -> {PARENT_MODIFIER_STRUCT_NAME} {{",),
        indent_style,
      ));

      sections.push(indent(
        indent(
          format!(
            "{PARENT_MODIFIER_STRUCT_NAME}::from_ref(self.append_to_skribble_value(\"{name}\"))"
          ),
          indent_style,
        ),
        indent_style,
      ));
    }

    trait_names.push(PARENT_MODIFIER_TRAIT_NAME.into());
    struct_names_map.insert(PARENT_MODIFIER_STRUCT_NAME.into(), trait_names.len());

    // Add the implementation for each of the structs.
    sections.push(generate_struct_implementations(
      &struct_names_map,
      &trait_names,
    ));

    format!("{HEADER}\n{}", sections.join("\n"))
  }
}

const PARENT_MODIFIER_TRAIT_NAME: &str = "ParentModifier";
const PARENT_MODIFIER_STRUCT_NAME: &str = "ParentModifierChild";

fn generate_impl_skribble_value(name: impl AsRef<str>) -> String {
  format!(
    indoc!(
      "
      impl SkribbleValue for {} {{
        fn from_ref(value: impl AsRef<str>) -> Self {{
          Self(value.as_ref().to_string())
        }}
        fn get_skribble_value(&self) -> &String {{
          &self.0
        }}
      }}
    "
    ),
    name.as_ref()
  )
}

fn generate_struct_implementations(
  struct_names_map: &IndexMap<String, usize>,
  trait_names: &Vec<String>,
) -> String {
  let mut content = Vec::<String>::new();
  for (struct_name, min_index) in struct_names_map.iter() {
    for (index, trait_name) in trait_names.iter().enumerate() {
      if *min_index <= index {
        content.push(format!("impl {trait_name} for {struct_name} {{}}",));
      }
    }
  }

  content.join("\n")
}

fn generate_struct(name: impl AsRef<str>) -> String {
  let name = name.as_ref();
  format!("pub struct {name}(String);")
}

fn get_method_name(name: impl Into<String>, method_names: &mut IndexSet<String>) -> String {
  let method_name = name.into().to_snake_case();
  let mut index = 0;
  let mut current_method_name = method_name.clone();
  loop {
    if method_names.contains(&current_method_name) {
      index += 1;
      current_method_name = format!("{}{}", method_name, index);
      continue;
    }

    method_names.insert(current_method_name.clone());
    break;
  }

  current_method_name
}

const HEADER: &str = r#"// This file was generated by skribble.
pub fn sk() -> SkribbleRoot {
  SkribbleRoot::from_ref("")
}
pub struct SkribbleRoot(String);
impl SkribbleValue for SkribbleRoot {
  fn from_ref(value: impl AsRef<str>) -> Self {
    Self(value.as_ref().to_string())
  }

  fn get_skribble_value(&self) -> &String {
    &self.0
  }
}
pub trait SkribbleAtomValue {
  fn from_ref(value: impl AsRef<str>) -> Self;
  fn get_skribble_value(&self) -> &String;
  fn append_to_skribble_value(&self, value: impl AsRef<str>) -> String {
    format!("{}{}", self.get_skribble_value(), value.as_ref())
  }
}
pub trait SkribbleValue {
  fn from_ref(value: impl AsRef<str>) -> Self;
  fn get_skribble_value(&self) -> &String;
  fn append_to_skribble_value(&self, value: impl AsRef<str>) -> String {
    format!("{}{}", self.get_skribble_value(), value.as_ref())
  }
}
"#;

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
    let content = &runner.generate().unwrap()[0].content;
    insta::assert_display_snapshot!(content);
  }
}
