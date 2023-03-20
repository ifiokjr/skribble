use indexmap::IndexMap;
use indexmap::IndexSet;

use super::*;

pub(crate) fn generate_media_queries(
  config: &MergedConfig,
  indent_style: IndentStyle,
  method_names: &mut IndexSet<String>,
  sections: &mut Vec<String>,
  struct_names_map: &mut IndexMap<String, usize>,
  trait_names: &mut Vec<String>,
) {
  for (key, map) in config.media_queries.iter() {
    let mut section = Vec::<String>::new();
    let trait_name = format!("MediaQuery{}", key.to_pascal_case());
    let struct_name = format!("{trait_name}Child");
    section.push(generate_struct(&struct_name));
    section.push(generate_impl_skribble_value(&struct_name));

    let mut methods = vec![format!("pub trait {trait_name}: SkribbleValue {{")];

    for (name, media_query) in map.iter() {
      let method_name = get_method_name(name, method_names);

      if let Some(ref description) = media_query.description {
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
}

pub(crate) fn generate_parent_modifiers(
  config: &MergedConfig,
  indent_style: IndentStyle,
  method_names: &mut IndexSet<String>,
  sections: &mut Vec<String>,
  struct_names_map: &mut IndexMap<String, usize>,
  trait_names: &mut Vec<String>,
) {
  // parent modifiers
  sections.push(generate_struct(PARENT_MODIFIER_STRUCT_NAME));
  sections.push(generate_impl_skribble_value(PARENT_MODIFIER_STRUCT_NAME));
  sections.push(format!(
    "pub trait {PARENT_MODIFIER_TRAIT_NAME}: SkribbleValue {{"
  ));

  for (name, modifier) in config.parent_modifiers.iter() {
    let method_name = get_method_name(name, method_names);

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

    sections.push(indent("}", indent_style));
  }

  sections.push("}".into());

  trait_names.push(PARENT_MODIFIER_TRAIT_NAME.into());
  struct_names_map.insert(PARENT_MODIFIER_STRUCT_NAME.into(), trait_names.len());
}

const PARENT_MODIFIER_TRAIT_NAME: &str = "ParentModifier";
const PARENT_MODIFIER_STRUCT_NAME: &str = "ParentModifierChild";

pub(crate) fn generate_modifiers(
  config: &MergedConfig,
  indent_style: IndentStyle,
  method_names: &mut IndexSet<String>,
  sections: &mut Vec<String>,
  struct_names_map: &mut IndexMap<String, usize>,
  trait_names: &mut Vec<String>,
) {
  for (key, map) in config.modifiers.iter() {
    let mut section = Vec::<String>::new();
    let trait_name = format!("Modifier{}", key.to_pascal_case());
    let struct_name = format!("{trait_name}Child");
    section.push(generate_struct(&struct_name));
    section.push(generate_impl_skribble_value(&struct_name));

    let mut methods = vec![format!("pub trait {trait_name}: SkribbleValue {{")];

    for (name, modifier) in map.iter() {
      let method_name = get_method_name(name, method_names);

      if let Some(ref description) = modifier.description {
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
}

const ATOM_TRAIT_NAME: &str = "Atom";

pub(crate) fn generate_atoms(
  config: &MergedConfig,
  indent_style: IndentStyle,
  method_names: &mut IndexSet<String>,
  sections: &mut Vec<String>,
  trait_names: &mut Vec<String>,
) {
  let mut struct_content = Vec::<String>::new();
  let mut trait_content = Vec::<String>::new();

  // parent modifiers
  trait_content.push(format!("pub trait {ATOM_TRAIT_NAME}: SkribbleValue {{"));

  for (name, modifier) in config.atoms.iter() {
    let method_name = get_method_name(name, method_names);
    let atom_struct_name = format!("Atom{}", name.to_pascal_case());

    struct_content.push(generate_struct(&atom_struct_name));
    struct_content.push(generate_impl_skribble_value(&atom_struct_name));

    match modifier.values {
      LinkedValues::Color(ref color) => {
        struct_content.push(format!("impl Color for {atom_struct_name} {{}}"));

        if !color.ignore_palette {
          struct_content.push(format!("impl Palette for {atom_struct_name} {{}}"));
        }
      }
      LinkedValues::Values(ref value_set) => {
        for value_set_name in value_set.iter() {
          let value_set_trait_name = get_value_set_trait_name(value_set_name);

          struct_content.push(format!(
            "impl {value_set_trait_name} for {atom_struct_name} {{}}",
          ));
        }
      }
    }

    if let Some(ref description) = modifier.description {
      let description = description
        .split('\n')
        .collect::<Vec<&str>>()
        .join("\n/// ");
      trait_content.push(indent(format!("/// {description}"), indent_style));
    }

    trait_content.push(indent(
      format!("fn {method_name}(&self) -> {atom_struct_name} {{",),
      indent_style,
    ));

    trait_content.push(indent(
      indent(
        format!("{atom_struct_name}::from_ref(self.append_to_skribble_value(\"{name}\"))"),
        indent_style,
      ),
      indent_style,
    ));

    trait_content.push(indent("}", indent_style));
  }

  trait_content.push("}".into());

  trait_names.push(ATOM_TRAIT_NAME.into());
  sections.push(struct_content.join("\n"));
  sections.push(trait_content.join("\n"));
}

fn get_value_set_trait_name(value_set_name: &Prioritized<String>) -> String {
  format!("ValueSet{}", value_set_name.to_pascal_case())
}

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
      }}"
    ),
    name.as_ref()
  )
}

pub(crate) fn generate_struct_implementations(
  struct_names_map: &IndexMap<String, usize>,
  trait_names: &[String],
  sections: &mut Vec<String>,
) {
  let mut content = Vec::<String>::new();
  for (struct_name, min_index) in struct_names_map.iter() {
    for (index, trait_name) in trait_names.iter().enumerate() {
      if *min_index <= index {
        content.push(format!("impl {trait_name} for {struct_name} {{}}",));
      }
    }
  }

  sections.push(content.join("\n"));
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

pub(crate) fn combine_sections_with_header(sections: Vec<String>) -> String {
  format!("{HEADER}\n{}", sections.join("\n"))
}
