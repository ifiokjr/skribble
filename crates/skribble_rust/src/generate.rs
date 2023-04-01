use indexmap::indexmap;
use indexmap::IndexMap;
use skribble_core::wrap_indent;
use skribble_core::AnyEmptyResult;
use skribble_core::AnyResult;
use skribble_core::LinkedValues;
use skribble_core::ToSkribbleCss;

use super::indoc;
use super::RunnerConfig;
use super::ToPascalCase;
use super::ToSnakeCase;
use crate::error::Error;

fn generate_media_queries(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
  struct_names_map: &mut IndexMap<String, usize>,
  trait_names: &mut Vec<String>,
) -> AnyEmptyResult {
  for (key, map) in config.media_queries.iter() {
    let mut section = Vec::<String>::new();
    let trait_name = format!("MediaQuery{}", key.to_pascal_case());
    let struct_name = format!("{trait_name}Child");
    section.push(generate_struct(&struct_name));
    section.push(generate_impl_skribble_value(&struct_name));

    let mut methods = vec![format!("pub trait {trait_name}: SkribbleValue {{")];

    for (name, media_query) in map.iter() {
      let method_name = get_method_name(name, method_names)?;
      let css_docs = wrap_indent(
        wrap_docs(wrap_in_code_block(
          media_query_docs(&media_query.query),
          "css",
        )),
        1,
      );

      if let Some(ref description) = media_query.description {
        methods.push(wrap_indent(wrap_docs(description), 1));
        methods.push(wrap_indent(wrap_docs("\n"), 1));
      }

      methods.push(css_docs);
      methods.push(wrap_indent(
        format!("#[inline]\nfn {method_name}(&self) -> {struct_name} {{"),
        1,
      ));
      methods.push(wrap_indent(
        format!("{struct_name}::from_ref(self.append_to_skribble_value(\"{name}\"))"),
        2,
      ));
      methods.push(wrap_indent("}", 1));
    }

    methods.push("}".into());
    section.push(methods.join("\n"));

    trait_names.push(trait_name);
    struct_names_map.insert(struct_name, trait_names.len());
    sections.push(section.join("\n"));
  }

  Ok(())
}

fn media_query_docs(query: impl AsRef<str>) -> String {
  let query = query.as_ref();
  format!("@media {query} {{\n  /* ... */\n}}")
}

fn modifier_docs(values: &[String]) -> String {
  let value = values.join(", ");
  format!("{value} {{\n  /* ... */\n}}")
}

fn generate_modifiers(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
  struct_names_map: &mut IndexMap<String, usize>,
  trait_names: &mut Vec<String>,
) -> AnyEmptyResult {
  for (key, map) in config.modifiers.iter() {
    let mut section = Vec::<String>::new();
    let trait_name = format!("Modifier{}", key.to_pascal_case());
    let struct_name = format!("{trait_name}Child");
    section.push(generate_struct(&struct_name));
    section.push(generate_impl_skribble_value(&struct_name));

    let mut methods = vec![format!("pub trait {trait_name}: SkribbleValue {{")];

    for (name, modifier) in map.iter() {
      let method_name = get_method_name(name, method_names)?;
      let css_docs = wrap_indent(
        wrap_docs(wrap_in_code_block(modifier_docs(&modifier.values), "css")),
        1,
      );

      if let Some(ref description) = modifier.description {
        methods.push(wrap_indent(wrap_docs(description), 1));
        methods.push(wrap_indent(wrap_docs("\n"), 1));
      }

      methods.push(css_docs);
      methods.push(wrap_indent(
        format!("#[inline]\nfn {method_name}(&self) -> {struct_name} {{"),
        1,
      ));
      methods.push(wrap_indent(
        format!("{struct_name}::from_ref(self.append_to_skribble_value(\"{name}\"))"),
        2,
      ));
      methods.push(wrap_indent("}", 1));
    }

    methods.push("}".into());
    section.push(methods.join("\n"));

    trait_names.push(trait_name);
    struct_names_map.insert(struct_name, trait_names.len());
    sections.push(section.join("\n"));
  }

  Ok(())
}

fn generate_keyframes(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
  name: impl AsRef<str>,
) -> AnyEmptyResult {
  let name = name.as_ref();

  sections.push(format!("pub trait {name}: SkribbleValue {{"));

  for (name, keyframe) in config.keyframes.iter() {
    let method_name = get_method_name(name, method_names)?;
    let css_docs = wrap_indent(
      wrap_docs(wrap_in_code_block(keyframe.to_skribble_css(config)?, "css")),
      1,
    );

    if let Some(ref description) = keyframe.description {
      sections.push(wrap_indent(wrap_docs(description), 1));
      sections.push(wrap_indent(wrap_docs("\n"), 1));
    }

    sections.push(css_docs);

    sections.push(wrap_indent(
      format!("#[inline]\nfn {method_name}(&self) -> String {{"),
      1,
    ));

    sections.push(wrap_indent(
      format!("self.append_string_to_skribble_value(\"{name}\")"),
      2,
    ));

    sections.push(wrap_indent("}", 1));
  }

  sections.push("}".into());

  Ok(())
}

fn generate_value_sets(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
) -> AnyEmptyResult {
  for (name, value_set) in config.value_sets.iter() {
    let value_set_trait_name = get_value_set_trait_name(name);
    sections.push(format!(
      "pub trait {value_set_trait_name}: SkribbleValue {{"
    ));

    for (value_name, _) in value_set.values.iter() {
      let method_name = get_method_name(value_name, method_names)?;

      if let Some(ref description) = value_set.description {
        sections.push(wrap_indent(wrap_docs(description), 1));
      }

      sections.push(wrap_indent(
        format!("#[inline]\nfn {method_name}(&self) -> String {{"),
        1,
      ));

      sections.push(wrap_indent(
        format!("self.append_string_to_skribble_value(\"{value_name}\")"),
        2,
      ));

      sections.push(wrap_indent("}", 1));
    }

    sections.push("}".into());
  }

  Ok(())
}

fn generate_named_classes(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
  trait_names: &mut Vec<String>,
) -> AnyEmptyResult {
  sections.push("pub trait NamedClasses: SkribbleValue {".into());

  for (class_name, class) in config.classes.iter() {
    let method_name = get_method_name(class_name, method_names)?;

    if let Some(ref description) = class.description {
      sections.push(wrap_indent(wrap_docs(description), 1));
    }

    sections.push(wrap_indent(
      format!("#[inline]\nfn {method_name}(&self) -> String {{"),
      1,
    ));

    sections.push(wrap_indent(
      format!("self.append_string_to_skribble_value(\"{class_name}\")"),
      2,
    ));

    sections.push(wrap_indent("}", 1));
  }

  trait_names.push("NamedClasses".into());
  sections.push("}".into());

  Ok(())
}

const ATOM_TRAIT_NAME: &str = "Atom";

fn generate_atoms(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
  trait_names: &mut Vec<String>,
) -> AnyEmptyResult {
  let mut struct_content = Vec::<String>::new();
  let mut trait_content = Vec::<String>::new();

  // parent modifiers
  trait_content.push(format!("pub trait {ATOM_TRAIT_NAME}: SkribbleValue {{"));

  for (name, modifier) in config.atoms.iter() {
    let method_name = get_method_name(name, method_names)?;
    let atom_struct_name = format!("Atom{}", name.to_pascal_case());

    struct_content.push(generate_struct(&atom_struct_name));
    struct_content.push(generate_impl_skribble_value(&atom_struct_name));

    match modifier.values {
      LinkedValues::Color => {
        struct_content.push(format!("impl Color for {atom_struct_name} {{}}"));
        struct_content.push(format!("impl Palette for {atom_struct_name} {{}}"));
      }
      LinkedValues::Values(ref value_set) => {
        for value_set_name in value_set.iter() {
          let value_set_trait_name = get_value_set_trait_name(&value_set_name.value);

          struct_content.push(format!(
            "impl {value_set_trait_name} for {atom_struct_name} {{}}",
          ));
        }
      }
      LinkedValues::Keyframes => {
        let keyframe_trait_name = get_keyframe_trait_name(&atom_struct_name);
        generate_keyframes(config, method_names, sections, &keyframe_trait_name)?;

        struct_content.push(format!(
          "impl {keyframe_trait_name} for {atom_struct_name} {{}}",
        ));
      }
    }

    if let Some(ref description) = modifier.description {
      trait_content.push(wrap_indent(wrap_docs(description), 1));
    }

    trait_content.push(wrap_indent(
      format!("#[inline]\nfn {method_name}(&self) -> {atom_struct_name} {{"),
      1,
    ));

    trait_content.push(wrap_indent(
      format!("{atom_struct_name}::from_ref(self.append_to_skribble_value(\"{name}\"))"),
      2,
    ));

    trait_content.push(wrap_indent("}", 1));
  }

  trait_content.push("}".into());

  trait_names.push(ATOM_TRAIT_NAME.into());
  sections.push(struct_content.join("\n"));
  sections.push(trait_content.join("\n"));

  Ok(())
}

fn generate_palette(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
) -> AnyEmptyResult {
  sections.push("pub trait Palette: SkribbleValue {".into());

  for (name, _) in config.palette.iter() {
    let method_name = get_method_name(name, method_names)?;

    sections.push(wrap_indent(
      format!("#[inline]\nfn {method_name}(&self) -> String {{"),
      1,
    ));

    sections.push(wrap_indent(
      format!("self.append_string_to_skribble_value(\"{name}\")"),
      2,
    ));

    sections.push(wrap_indent("}", 1));
  }

  sections.push("}".into());

  Ok(())
}

fn generate_css_variables(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
) -> AnyEmptyResult {
  let mut entries = vec![
    indoc!(
      "
    pub fn vars() -> CssVariables {
      CssVariables
    }
    pub struct CssVariables;
    impl CssVariables {"
    )
    .into(),
  ];
  let mut colors = vec!["pub trait Color: SkribbleValue {".into()];

  for (name, css_variable) in config.css_variables.iter() {
    let method_name = get_method_name(name, method_names)?;
    let variable_name = css_variable.get_variable(config.options());
    let mut property_rule = String::new();
    css_variable.write_property_rule(&mut property_rule, config)?;
    let css_docs = wrap_indent(wrap_docs(wrap_in_code_block(property_rule, "css")), 1);

    if let Some(ref description) = css_variable.description {
      entries.push(wrap_indent(wrap_docs(description), 1));
      entries.push(wrap_indent(wrap_docs("\n"), 1));
    }

    entries.push(css_docs.clone());

    entries.push(wrap_indent(
      format!("#[inline]\npub fn {method_name}(&self) -> String {{"),
      1,
    ));

    entries.push(wrap_indent(format!("\"{variable_name}\".into()",), 2));

    entries.push(wrap_indent("}", 1));

    if css_variable.syntax.is_color() {
      if let Some(ref description) = css_variable.description {
        colors.push(wrap_indent(wrap_docs(description), 1));
        colors.push(wrap_indent(wrap_docs("\n"), 1));
      }

      colors.push(css_docs);

      colors.push(wrap_indent(
        format!("#[inline]\nfn {method_name}(&self) -> String {{"),
        1,
      ));
      colors.push(wrap_indent(
        format!("self.append_string_to_skribble_value(\"{name}\")"),
        2,
      ));
      colors.push(wrap_indent("}", 1))
    }
  }

  entries.push("}".into());
  colors.push("}".into());
  sections.push(entries.join("\n"));
  sections.push(colors.join("\n"));

  Ok(())
}

fn wrap_docs(content: impl AsRef<str>) -> String {
  let mut result = vec![];
  for line in content.as_ref().lines() {
    result.push(format!("/// {line}"));
  }

  result.join("\n")
}

fn wrap_in_code_block(content: impl AsRef<str>, r#type: impl AsRef<str>) -> String {
  format!(
    "```{}\n{}\n```",
    r#type.as_ref(),
    content.as_ref().trim_end(),
  )
}

fn get_value_set_trait_name(value_set_name: impl Into<String>) -> String {
  format!("ValueSet{}", value_set_name.into().to_pascal_case())
}

fn get_keyframe_trait_name(atom_name: impl Into<String>) -> String {
  format!("KeyframeSet{}", atom_name.into().to_pascal_case())
}

fn generate_impl_skribble_value(name: impl AsRef<str>) -> String {
  format!(
    indoc!(
      "
      impl SkribbleValue for {} {{
        #[inline]
        fn from_ref(value: impl AsRef<str>) -> Self {{
          Self(value.as_ref().to_string())
        }}
        #[inline]
        fn get_skribble_value(&self) -> &String {{
          &self.0
        }}
      }}"
    ),
    name.as_ref()
  )
}

fn generate_struct_implementations(
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

fn get_method_name(
  name: impl AsRef<str>,
  method_names: &mut IndexMap<String, String>,
) -> AnyResult<String> {
  let method_name = safe_method_name(&name)?;
  let mut index = 0;
  let mut current_method_name = method_name.clone();

  loop {
    if method_names.contains_key(&current_method_name) {
      index += 1;
      current_method_name = format!("{}_{}", method_name, index);
      continue;
    }

    method_names.insert(current_method_name.clone(), name.as_ref().to_string());
    break;
  }

  Ok(current_method_name)
}

fn safe_method_name(name: impl AsRef<str>) -> Result<String, Error> {
  let name = name.as_ref();

  let prefix = match name.chars().next() {
    Some(first_char) if first_char.is_ascii_digit() => "n_",
    Some(first_char) if !first_char.is_ascii_alphabetic() => {
      match first_char {
        '_' => "u_",
        '-' => "m_",
        '+' => "p_",
        '.' => "d_",
        _ => return Err(Error::InvalidMethodName(name.to_string())),
      }
    }
    _ => "",
  };

  let method_name = format!("{prefix}{}", name.to_snake_case());

  if RESERVED_WORDS.contains(&method_name.as_str()) {
    return Ok(format!("r#{}", method_name));
  }

  if method_name.is_empty() {
    return Ok("__".into());
  }

  Ok(method_name)
}

const RESERVED_WORDS: &[&str] = &[
  "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do",
  "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop",
  "macro", "match", "mod", "move", "mut", "offsetof", "override", "priv", "proc", "pub", "pure",
  "ref", "return", "Self", "self", "sizeof", "static", "struct", "super", "trait", "true", "type",
  "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

const HEADER: &str = r#"#![allow(unused)]
// This file was generated by skribble.
use private::SkribbleValue;
pub fn sk() -> SkribbleRoot {
  SkribbleRoot::from_ref("")
}
pub struct SkribbleRoot(String);
impl SkribbleValue for SkribbleRoot {
  #[inline]
  fn from_ref(value: impl AsRef<str>) -> Self {
    Self(value.as_ref().to_string())
  }
  #[inline]
  fn get_skribble_value(&self) -> &String {
    &self.0
  }
}
mod private {
  #[doc(hidden)]
  pub trait SkribbleValue {
    fn from_ref(value: impl AsRef<str>) -> Self;
    fn get_skribble_value(&self) -> &String;
    #[inline]
    fn append_to_skribble_value(&self, value: impl AsRef<str>) -> String {
      let current_value = self.get_skribble_value();
      let prefix = if current_value.is_empty() {
        "".into()
      } else {
        format!("{current_value}:")
      };

      format!("{}{}", prefix, value.as_ref())
    }
    #[inline]
    fn append_string_to_skribble_value(&self, value: impl AsRef<str>) -> String {
      format!("{}:${}", self.get_skribble_value(), value.as_ref())
    }
  }
}"#;

fn combine_sections_with_header(sections: Vec<String>) -> String {
  format!("{HEADER}\n{}", sections.join("\n"))
}

pub(crate) fn generate_file_contents(
  config: &RunnerConfig,
) -> AnyResult<(String, IndexMap<String, String>)> {
  let mut method_names = IndexMap::<String, String>::new();
  let mut sections = Vec::<String>::new();
  let mut trait_names = vec![];
  let mut struct_names_map: IndexMap<String, usize> = indexmap! { "SkribbleRoot".into() => 0 };

  generate_css_variables(config, &mut method_names, &mut sections)?;
  generate_media_queries(
    config,
    &mut method_names,
    &mut sections,
    &mut struct_names_map,
    &mut trait_names,
  )?;
  generate_modifiers(
    config,
    &mut method_names,
    &mut sections,
    &mut struct_names_map,
    &mut trait_names,
  )?;
  generate_value_sets(config, &mut method_names, &mut sections)?;
  generate_palette(config, &mut method_names, &mut sections)?;
  generate_atoms(config, &mut method_names, &mut sections, &mut trait_names)?;
  generate_named_classes(config, &mut method_names, &mut sections, &mut trait_names)?;
  generate_struct_implementations(&struct_names_map, &trait_names, &mut sections);

  Ok((combine_sections_with_header(sections), method_names))
}
