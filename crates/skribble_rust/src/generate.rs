use std::fmt::Write;

use indexmap::indexmap;
use indexmap::IndexMap;
use skribble_core::wrap_indent;
use skribble_core::AnyEmptyResult;
use skribble_core::AnyResult;
use skribble_core::AtomType;
use skribble_core::LinkedValues;
use skribble_core::PrioritizedString;
use skribble_core::ToSkribbleCss;
use skribble_core::TransformationScope;
use skribble_core::DEFAULT_COLOR_FIELDS;
use typed_builder::TypedBuilder;

use super::indoc;
use super::RunnerConfig;
use super::ToPascalCase;
use super::ToSnakeCase;

#[derive(TypedBuilder)]
struct StructProp {
  // #[builder(setter(into))]
  pub index: usize,
  #[builder(default, setter(into))]
  pub included: Option<Vec<String>>,
  #[builder(default, setter(into))]
  pub excluded: Option<Vec<String>>,
}

type StructNames = IndexMap<String, StructProp>;

fn generate_media_queries(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
  struct_names_map: &mut StructNames,
  trait_names: &mut Vec<String>,
) -> AnyEmptyResult {
  for (key, map) in config.media_queries.iter() {
    let mut section = Vec::<String>::new();
    let trait_name = format!("GeneratedMediaQuery{}", key.to_pascal_case());
    let struct_name = format!("{trait_name}Child");
    section.push(generate_struct(&struct_name));
    section.push(generate_impl_skribble_value(&struct_name));

    let mut methods = vec![format!("pub trait {trait_name}: GeneratedSkribbleValue {{")];

    for (name, media_query) in map.iter() {
      let method_name = get_method_name(name, GLOBAL_PREFIX, method_names)?;
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
        format!("{struct_name}::from_ref(self.append(\"{name}\"))"),
        2,
      ));
      methods.push(wrap_indent("}", 1));
      methods.push(wrap_indent(
        format!(
          "#[inline]\nfn {method_name}_(&self, property: &'static str, value: &'static str) -> \
           String {{"
        ),
        1,
      ));
      methods.push(wrap_indent(
        format!("self.append(format!(\"{name}:[{{}}={{}}]\", property.trim(), value.trim()))"),
        2,
      ));
      methods.push(wrap_indent("}", 1));
    }

    methods.push("}".into());
    section.push(methods.join("\n"));

    trait_names.push(trait_name);
    struct_names_map.insert(
      struct_name,
      StructProp::builder().index(trait_names.len()).build(),
    );
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
  struct_names_map: &mut StructNames,
  trait_names: &mut Vec<String>,
) -> AnyEmptyResult {
  for (key, map) in config.modifiers.iter() {
    let mut section = Vec::<String>::new();
    let trait_name = format!("GeneratedModifier{}", key.to_pascal_case());
    let struct_name = format!("{trait_name}Child");
    section.push(generate_struct(&struct_name));
    section.push(generate_impl_skribble_value(&struct_name));

    let mut methods = vec![format!("pub trait {trait_name}: GeneratedSkribbleValue {{")];

    for (name, modifier) in map.iter() {
      let method_name = get_method_name(name, GLOBAL_PREFIX, method_names)?;
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
        format!("{struct_name}::from_ref(self.append(\"{name}\"))"),
        2,
      ));
      methods.push(wrap_indent("}", 1));

      methods.push(wrap_indent(
        format!(
          "#[inline]\nfn {method_name}_(&self, property: &'static str, value: &'static str) -> \
           String {{"
        ),
        1,
      ));
      methods.push(wrap_indent(
        format!("self.append(format!(\"{name}:[{{}}={{}}]\", property.trim(), value.trim()))"),
        2,
      ));
      methods.push(wrap_indent("}", 1));
    }

    methods.push("}".into());
    section.push(methods.join("\n"));

    trait_names.push(trait_name);
    struct_names_map.insert(
      struct_name,
      StructProp::builder().index(trait_names.len()).build(),
    );
    sections.push(section.join("\n"));
  }

  Ok(())
}

fn generate_transformers(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
  struct_names_map: &mut StructNames,
  trait_names: &mut Vec<String>,
) -> AnyEmptyResult {
  for (key, map) in config.transformers.iter() {
    let mut section = Vec::<String>::new();
    let trait_name = format!("GeneratedTransformer{}", key.to_pascal_case());
    let mut methods = vec![format!("pub trait {trait_name}: GeneratedSkribbleValue {{")];
    trait_names.push(trait_name.clone());

    for (name, transformer) in map.iter() {
      let method_name = get_method_name(name, TRANSFORMER_PREFIX, method_names)?;
      let struct_name = format!("{trait_name}Group{}Child", name.to_pascal_case());
      section.push(generate_struct(&struct_name));
      section.push(generate_impl_skribble_value(&struct_name));

      match transformer.values.as_ref() {
        None => {
          if let Some(ref description) = transformer.description {
            methods.push(wrap_indent(wrap_docs(description), 1));
            methods.push(wrap_indent(wrap_docs("\n"), 1));
          }

          methods.push(wrap_indent(
            format!("#[inline]\nfn {method_name}(&self) -> {struct_name} {{"),
            1,
          ));
          methods.push(wrap_indent(
            format!("{struct_name}::from_ref(self.append_transformer(\"{name}\", None))"),
            2,
          ));
          methods.push(wrap_indent("}", 1));
        }
        Some(values) => {
          if let Some(ref description) = transformer.description {
            methods.push(wrap_indent(wrap_docs(description), 1));
            methods.push(wrap_indent(wrap_docs("\n"), 1));
          }

          methods.push(wrap_indent(
            format!("#[inline]\nfn {method_name}(&self, value: &'static str,) -> {struct_name} {{"),
            1,
          ));
          methods.push(wrap_indent(
            format!("{struct_name}::from_ref(self.append_transformer(\"{name}\", Some(value)))"),
            2,
          ));
          methods.push(wrap_indent("}", 1));

          for (value_name, _value) in values.iter() {
            let value_method_name = get_method_name(
              format!("{name}=={value_name}"),
              TRANSFORMER_PREFIX,
              method_names,
            )?;
            methods.push(wrap_indent(
              format!("#[inline]\nfn {value_method_name}(&self) -> {struct_name} {{"),
              1,
            ));
            methods.push(wrap_indent(
              format!(
                "{struct_name}::from_ref(self.append_transformer(\"{name}\", \
                 Some(\"={value_name}\")))"
              ),
              2,
            ));
            methods.push(wrap_indent("}", 1));
          }
        }
      }

      let included: Option<Vec<String>> = match &transformer.scope {
        TransformationScope::All => None,
        TransformationScope::Color => {
          Some(
            config
              .atoms
              .iter()
              .filter_map(|(name, atom)| {
                if let AtomType::Color = atom.get_type() {
                  Some(generate_atom_trait_name(name))
                } else {
                  None
                }
              })
              .collect(),
          )
        }
        TransformationScope::Atoms(names) => {
          Some(names.iter().map(generate_atom_trait_name).collect())
        }
        _ => None,
      };
      struct_names_map.insert(
        struct_name,
        StructProp::builder()
          .index(trait_names.len())
          .included(included)
          .build(),
      );
    }

    methods.push("}".into());
    section.push(methods.join("\n"));
    sections.push(section.join("\n"));
  }

  Ok(())
}

fn get_keyframe_trait_name(keyframe_name: impl AsRef<str>) -> String {
  let keyframe_name = keyframe_name.as_ref();
  format!("generated-keyframe-{keyframe_name}").to_pascal_case()
}

fn generate_keyframes(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
) -> AnyEmptyResult {
  for (name, keyframe) in config.keyframes.iter() {
    let keyframe_trait_name = get_keyframe_trait_name(name);
    sections.push(format!(
      "pub trait {keyframe_trait_name}: GeneratedSkribbleValue {{"
    ));
    let method_name = get_method_name(name, KEYFRAMES_PREFIX, method_names)?;

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

    sections.push(wrap_indent(format!("self.append_value(\"{name}\")"), 2));

    sections.push(wrap_indent("}", 1));
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
  sections.push("pub trait GeneratedNamedClasses: GeneratedSkribbleValue {".into());

  for (name, named_class) in config.classes.iter() {
    if named_class.is_reference() {
      continue;
    }

    let method_name = get_method_name(name, GLOBAL_PREFIX, method_names)?;

    if let Some(ref description) = named_class.description {
      sections.push(wrap_indent(wrap_docs(description), 1));
    }

    sections.push(wrap_indent(
      format!("#[inline]\nfn {method_name}(&self) -> String {{"),
      1,
    ));

    sections.push(wrap_indent(format!("self.append_value(\"{name}\")"), 2));

    sections.push(wrap_indent("}", 1));
  }

  trait_names.push("GeneratedNamedClasses".into());
  sections.push("}".into());

  Ok(())
}

fn generate_aliases(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
  trait_names: &mut Vec<String>,
) -> AnyEmptyResult {
  sections.push("pub trait GeneratedAliases: GeneratedSkribbleValue {".into());

  for (alias_name, alias) in config.aliases.iter() {
    let method_name = get_method_name(alias_name, GLOBAL_PREFIX, method_names)?;
    let classes = alias
      .classes
      .iter()
      .map(|class| format!("\"{class}\""))
      .collect::<Vec<String>>()
      .join(", ");
    let classes_array = format!("[{classes}].map(|class| self.append(class)).join(\" \")");

    if let Some(ref description) = alias.description {
      sections.push(wrap_indent(wrap_docs(description), 1));
    }

    sections.push(wrap_indent(
      format!("#[inline]\nfn {method_name}(&self) -> String {{"),
      1,
    ));

    sections.push(wrap_indent(&classes_array, 2));

    sections.push(wrap_indent("}", 1));
  }

  trait_names.push("GeneratedAliases".into());
  sections.push("}".into());

  Ok(())
}

fn generate_atom_trait_name(name: impl AsRef<str>) -> String {
  let name = name.as_ref();
  format!("{ATOM_TRAIT_NAME}{}", name.to_pascal_case())
}

fn generate_atoms(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
  trait_names: &mut Vec<String>,
) -> AnyEmptyResult {
  let mut struct_content = Vec::<String>::new();
  let mut trait_content = Vec::<String>::new();

  for (atom_name, atom) in config.atoms.iter() {
    let atom_trait_name = generate_atom_trait_name(atom_name);
    let method_name = get_method_name(atom_name, GLOBAL_PREFIX, method_names)?;
    let atom_struct_name = format!("{atom_trait_name}Child");
    trait_content.push(format!(
      "pub trait {atom_trait_name}: GeneratedSkribbleValue {{"
    ));

    struct_content.push(generate_struct(&atom_struct_name));
    struct_content.push(generate_impl_skribble_value(&atom_struct_name));

    match atom.values {
      LinkedValues::Color(ref color_field) => {
        for name in color_field.named_fields.keys() {
          generate_color_method(name, sections, method_names, None)?;
        }

        let mut valid_color_names = Vec::<String>::new();

        valid_color_names.extend(config.css_variables.iter().filter_map(|(name, variable)| {
          if variable.is_color() && !color_field.excluded.contains(name) {
            Some(name.clone())
          } else {
            None
          }
        }));

        if !color_field.disable_palette {
          valid_color_names.extend(config.palette.keys().cloned());
        }

        let fields = color_field.get_fields();
        valid_color_names.extend(fields.keys().cloned());

        for name in valid_color_names.iter() {
          let color_trait_name = get_color_trait_name(name);
          struct_content.push(format!(
            "impl {color_trait_name} for {atom_struct_name} {{}}"
          ));
        }
      }
      LinkedValues::Keyframes => {
        for name in config.keyframes.keys() {
          let keyframe_trait_name = get_keyframe_trait_name(name);
          struct_content.push(format!(
            "impl {keyframe_trait_name} for {atom_struct_name} {{}}"
          ));
        }
      }
      LinkedValues::Values(ref value_sets) => {
        for PrioritizedString {
          value: value_set_name,
          ..
        } in value_sets.iter()
        {
          let Some(value_set) = config.value_sets.get(value_set_name) else {
            continue;
          };

          for value_name in value_set.values.keys() {
            get_method_name(value_name, atom_name, method_names)?;
            let value_set_trait_name = get_value_set_trait_name(value_set_name, value_name);
            struct_content.push(format!(
              "impl {value_set_trait_name} for {atom_struct_name} {{}}",
            ));
          }
        }
      }
    }

    // The atom
    if let Some(ref description) = atom.description {
      trait_content.push(wrap_indent(wrap_docs(description), 1));
    }

    trait_content.push(wrap_indent(
      format!("#[inline]\nfn {method_name}(&self) -> {atom_struct_name} {{"),
      1,
    ));

    trait_content.push(wrap_indent(
      format!("{atom_struct_name}::from_ref(self.append(\"{atom_name}\"))"),
      2,
    ));

    trait_content.push(wrap_indent("}", 1));

    // The atom argument
    trait_content.push(wrap_indent(
      format!("#[inline]\nfn {method_name}_(&self, value: &'static str) -> String {{"),
      1,
    ));

    trait_content.push(wrap_indent(
      format!("self.append(format!(\"{atom_name}:[{{}}]\", value.trim()))"),
      2,
    ));

    trait_content.push(wrap_indent("}", 1));
    trait_content.push("}".into());
    trait_names.push(atom_trait_name);
  }

  sections.push(struct_content.join("\n"));
  sections.push(trait_content.join("\n"));

  Ok(())
}

fn get_value_set_trait_name(
  value_set_name: impl AsRef<str>,
  value_name: impl AsRef<str>,
) -> String {
  let value_set_name = value_set_name.as_ref();
  let value_name = safe_method_name(value_name);
  format!("generated-value-set-{value_set_name}-{value_name}").to_pascal_case()
}

fn generate_value_sets(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
) -> AnyEmptyResult {
  for (value_set_name, value_set) in config.value_sets.iter() {
    for value_name in value_set.values.keys() {
      let value_set_trait_name = get_value_set_trait_name(value_set_name, value_name);
      let method_name = get_method_name(
        value_name,
        format!("{VALUE_SET_PREFIX}:::{value_set_name}"),
        method_names,
      )?;

      sections.push(format!(
        "pub trait {value_set_trait_name}: GeneratedSkribbleValue {{",
      ));

      sections.push(wrap_indent(
        format!("#[inline]\nfn {method_name}(&self) -> String {{"),
        1,
      ));

      sections.push(wrap_indent(
        format!("self.append_value(\"{value_name}\")"),
        2,
      ));

      sections.push(wrap_indent("}", 1));

      sections.push("}".into());
    }
  }

  Ok(())
}

fn get_color_trait_name(color_name: impl AsRef<str>) -> String {
  let color_name = color_name.as_ref();
  format!("generated-color-{color_name}").to_pascal_case()
}

fn generate_colors(
  config: &RunnerConfig,
  method_names: &mut IndexMap<String, String>,
  sections: &mut Vec<String>,
) -> AnyEmptyResult {
  for (name, css_variable) in config.css_variables.iter() {
    if !css_variable.is_color() {
      continue;
    }

    let mut property_rule = String::new();
    css_variable.write_property_rule(&mut property_rule, config, false)?;
    let mut css_docs = String::new();

    if let Some(ref description) = css_variable.description {
      write!(css_docs, "{}", wrap_indent(wrap_docs(description), 1))?;
      write!(css_docs, "{}", wrap_indent(wrap_docs("\n"), 1))?;
    }

    write!(
      css_docs,
      "{}",
      wrap_indent(wrap_docs(wrap_in_code_block(property_rule, "css")), 1)
    )?;

    generate_color_method(name, sections, method_names, Some(css_docs))?;
  }

  for name in config.palette.keys() {
    generate_color_method(name, sections, method_names, None)?;
  }

  let named_colors = DEFAULT_COLOR_FIELDS.clone();

  for name in named_colors.keys() {
    generate_color_method(name, sections, method_names, None)?;
  }

  Ok(())
}

fn generate_color_method(
  name: &String,
  sections: &mut Vec<String>,
  method_names: &mut IndexMap<String, String>,
  css_docs: Option<String>,
) -> AnyEmptyResult {
  let method_name = get_method_name(name, COLORS_PREFIX, method_names)?;
  let color_trait_name = get_color_trait_name(name);

  sections.push(format!(
    "pub trait {color_trait_name}: GeneratedSkribbleValue {{"
  ));

  if let Some(docs) = css_docs {
    sections.push(docs);
  }

  sections.push(wrap_indent(
    format!("#[inline]\nfn {method_name}(&self) -> String {{"),
    1,
  ));
  sections.push(wrap_indent(format!("self.append_value(\"{name}\")"), 2));
  sections.push(wrap_indent("}", 1));
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
    pub fn vars() -> GeneratedCssVariables {
      GeneratedCssVariables
    }
    pub struct GeneratedCssVariables;
    impl GeneratedCssVariables {"
    )
    .into(),
  ];

  for (name, css_variable) in config.css_variables.iter() {
    let method_name = get_method_name(name, VARIABLES_PREFIX, method_names)?;
    let variable_name = css_variable.get_variable(config.options());
    let mut property_rule = String::new();
    css_variable.write_property_rule(&mut property_rule, config, false)?;
    let css_docs = wrap_indent(wrap_docs(wrap_in_code_block(property_rule, "css")), 1);

    if let Some(ref description) = css_variable.description {
      entries.push(wrap_indent(wrap_docs(description), 1));
      entries.push(wrap_indent(wrap_docs("\n"), 1));
    }

    entries.push(css_docs);

    entries.push(wrap_indent(
      format!("#[inline]\npub fn {method_name}(&self) -> String {{"),
      1,
    ));

    entries.push(wrap_indent(format!("\"{variable_name}\".into()",), 2));

    entries.push(wrap_indent("}", 1));
  }

  entries.push("}".into());
  sections.push(entries.join("\n"));

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

fn generate_impl_skribble_value(name: impl AsRef<str>) -> String {
  format!(
    indoc!(
      "
      impl GeneratedSkribbleValue for {} {{
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
  struct_names_map: &StructNames,
  trait_names: &[String],
  sections: &mut Vec<String>,
) {
  let mut content = Vec::<String>::new();
  for (struct_name, prop) in struct_names_map.iter() {
    for (index, trait_name) in trait_names.iter().enumerate() {
      if prop.index > index {
        continue;
      }

      if let Some(ref included) = prop.included {
        // println!(
        //   "included: {included:?}\ntrait_name: {trait_name:?}\nstruct_name:
        // {struct_name}\n" );
        if !included.contains(trait_name) {
          continue;
        }
      }

      if let Some(ref excluded) = prop.excluded {
        if excluded.contains(trait_name) {
          continue;
        }
      }

      content.push(format!("impl {trait_name} for {struct_name} {{}}"));
    }
  }

  sections.push(content.join("\n"));
}

fn generate_struct(name: impl AsRef<str>) -> String {
  let name = name.as_ref();
  format!("pub struct {name}(String);")
}

fn get_method_name(
  value: impl AsRef<str>,
  prefix: impl AsRef<str>,
  method_names: &mut IndexMap<String, String>,
) -> AnyResult<String> {
  let method_name = safe_method_name(&value);
  let mut index = 0;
  let mut current_method_name = method_name.clone();

  loop {
    let with_prefix = format!("{}:::{}", prefix.as_ref(), current_method_name);
    if method_names.contains_key(&with_prefix) {
      index += 1;
      current_method_name = format!("{}_{}", method_name, index);
      continue;
    }

    method_names.insert(with_prefix, value.as_ref().to_string());
    break;
  }

  Ok(current_method_name)
}

fn safe_method_name(name: impl AsRef<str>) -> String {
  let name = name.as_ref();

  let prefix = match name.chars().next() {
    Some(first_char) if first_char.is_ascii_digit() => "n",
    Some(first_char) if !first_char.is_ascii_alphabetic() => {
      match first_char {
        '_' => "u",
        '-' => "m",
        '+' => "p",
        '.' => "d",
        _ => "ERROR_GENERATING_METHOD_NAME",
      }
    }
    _ => "",
  };

  let method_name = format!("{prefix}{}", name.to_snake_case());

  if "self" == &method_name {
    return "s_lf".into();
  }

  if "super" == &method_name {
    return "sup_r".into();
  }

  if RESERVED_WORDS.contains(&method_name.as_str()) {
    return format!("r#{}", method_name);
  }

  if method_name.is_empty() {
    return "__".into();
  }

  method_name
}

const RESERVED_WORDS: &[&str] = &[
  "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do",
  "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop",
  "macro", "match", "mod", "move", "mut", "offsetof", "override", "priv", "proc", "pub", "pure",
  "ref", "return", "Self", "self", "sizeof", "static", "struct", "super", "trait", "true", "type",
  "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];
const ATOM_TRAIT_NAME: &str = "GeneratedAtom";
pub(crate) const GLOBAL_PREFIX: &str = "global";
pub(crate) const VALUE_SET_PREFIX: &str = "values";
pub(crate) const TRANSFORMER_PREFIX: &str = "transformers";
pub(crate) const COLORS_PREFIX: &str = "colors";
pub(crate) const KEYFRAMES_PREFIX: &str = "keyframes";
pub(crate) const VARIABLES_PREFIX: &str = "variables";

const HEADER: &str = r#"#![allow(clippy::all)]
#![allow(unused)]
// This file was generated by skribble.
use private::GeneratedSkribbleValue;
pub fn sk() -> GeneratedSkribbleRoot {
  GeneratedSkribbleRoot::from_ref("")
}
pub struct GeneratedSkribbleRoot(String);
impl GeneratedSkribbleValue for GeneratedSkribbleRoot {
  #[inline]
  fn from_ref(value: impl AsRef<str>) -> Self {
    Self(value.as_ref().to_string())
  }
  #[inline]
  fn get_skribble_value(&self) -> &String {
    &self.0
  }
}
impl GeneratedSkribbleRoot {
  pub fn __(&self, property: &'static str, value: &'static str) -> String {
    self.append(format!("[{}={}]", property.trim(), value.trim()))
  }
}
mod private {
  #[doc(hidden)]
  pub trait GeneratedSkribbleValue {
    fn from_ref(value: impl AsRef<str>) -> Self;
    fn get_skribble_value(&self) -> &String;
    #[inline]
    fn append(&self, value: impl AsRef<str>) -> String {
      let current_value = self.get_skribble_value();
      let prefix = if current_value.is_empty() {
        "".into()
      } else {
        format!("{current_value}:")
      };

      format!("{}{}", prefix, value.as_ref())
    }
    #[inline]
    fn append_transformer(&self, name: impl AsRef<str>, value: Option<&'static str>) -> String {
      self.append(if let Some(value) = value {
        format!("({}={})", name.as_ref(), value.trim())
      } else {
        format!("({})", name.as_ref())
      })
    }
    #[inline]
    fn append_value(&self, value: impl AsRef<str>) -> String {
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
  let mut struct_names_map: StructNames = indexmap! {
    "GeneratedSkribbleRoot".into() => StructProp::builder().index(0).build()
  };

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
  generate_transformers(
    config,
    &mut method_names,
    &mut sections,
    &mut struct_names_map,
    &mut trait_names,
  )?;
  generate_keyframes(config, &mut method_names, &mut sections)?;
  generate_colors(config, &mut method_names, &mut sections)?;
  generate_value_sets(config, &mut method_names, &mut sections)?;
  generate_atoms(config, &mut method_names, &mut sections, &mut trait_names)?;
  generate_named_classes(config, &mut method_names, &mut sections, &mut trait_names)?;
  generate_aliases(config, &mut method_names, &mut sections, &mut trait_names)?;
  generate_struct_implementations(&struct_names_map, &trait_names, &mut sections);

  Ok((combine_sections_with_header(sections), method_names))
}
