use indexmap::IndexMap;
use indexmap::IndexSet;

use super::*;

fn generate_media_queries(
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
      methods.push(indent(
        format!("#[inline]\nfn {method_name}(&self) -> {struct_name} {{"),
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

fn media_query_docs(query: impl AsRef<str>) -> String {
  let query = query.as_ref();
  format!("@media {query} {{\n  /* ... */\n}}")
}

fn generate_parent_modifiers(
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
    let css_docs = wrap_indent(
      wrap_docs(wrap_in_code_block(modifier_docs(&modifier.values), "css")),
      1,
    );

    if let Some(ref description) = modifier.description {
      sections.push(wrap_indent(wrap_docs(description), 1));
      sections.push(wrap_indent(wrap_docs("\n"), 1));
    }

    sections.push(css_docs);
    sections.push(indent(
      format!("#[inline]\nfn {method_name}(&self) -> {PARENT_MODIFIER_STRUCT_NAME} {{"),
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

fn modifier_docs(values: &Vec<String>) -> String {
  let value = values.join(", ");
  format!("{value} {{\n  /* ... */\n}}")
}

const PARENT_MODIFIER_TRAIT_NAME: &str = "ParentModifier";
const PARENT_MODIFIER_STRUCT_NAME: &str = "ParentModifierChild";

fn generate_modifiers(
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
      let css_docs = wrap_indent(
        wrap_docs(wrap_in_code_block(modifier_docs(&modifier.values), "css")),
        1,
      );

      if let Some(ref description) = modifier.description {
        methods.push(wrap_indent(wrap_docs(description), 1));
        methods.push(wrap_indent(wrap_docs("\n"), 1));
      }

      methods.push(css_docs);
      methods.push(indent(
        format!("#[inline]\nfn {method_name}(&self) -> {struct_name} {{"),
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

fn generate_value_sets(
  config: &MergedConfig,
  indent_style: IndentStyle,
  sections: &mut Vec<String>,
) {
  for (name, value_set) in config.value_sets.iter() {
    let value_set_trait_name = get_value_set_trait_name(name);
    sections.push(format!(
      "pub trait {value_set_trait_name}: SkribbleValue {{"
    ));

    for (value_name, _) in value_set.values.iter() {
      let method_name = safe_method_name(value_name);

      if let Some(ref description) = value_set.description {
        sections.push(wrap_indent(wrap_docs(description), 1));
      }

      sections.push(indent(
        format!("#[inline]\nfn {method_name}(&self) -> String {{"),
        indent_style,
      ));

      sections.push(indent(
        indent(
          format!("self.append_string_to_skribble_value(\"{value_name}\")"),
          indent_style,
        ),
        indent_style,
      ));

      sections.push(indent("}", indent_style));
    }

    sections.push("}".into());
  }
}

fn generate_named_classes(
  config: &MergedConfig,
  indent_style: IndentStyle,
  sections: &mut Vec<String>,
  trait_names: &mut Vec<String>,
) {
  sections.push("pub trait NamedClasses: SkribbleValue {".into());

  for (class_name, class) in config.classes.iter() {
    let method_name = safe_method_name(class_name);

    if let Some(ref description) = class.description {
      sections.push(wrap_indent(wrap_docs(description), 1));
    }

    sections.push(indent(
      format!("#[inline]\nfn {method_name}(&self) -> String {{"),
      indent_style,
    ));

    sections.push(indent(
      indent(
        format!("self.append_string_to_skribble_value(\"{class_name}\")"),
        indent_style,
      ),
      indent_style,
    ));

    sections.push(indent("}", indent_style));
  }

  trait_names.push("NamedClasses".into());
  sections.push("}".into());
}

const ATOM_TRAIT_NAME: &str = "Atom";

fn generate_atoms(
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
          let value_set_trait_name = get_value_set_trait_name(&value_set_name.value);

          struct_content.push(format!(
            "impl {value_set_trait_name} for {atom_struct_name} {{}}",
          ));
        }
      }
    }

    if let Some(ref description) = modifier.description {
      trait_content.push(wrap_indent(wrap_docs(description), 1));
    }

    trait_content.push(indent(
      format!("#[inline]\nfn {method_name}(&self) -> {atom_struct_name} {{"),
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

fn generate_palette(config: &MergedConfig, indent_style: IndentStyle, sections: &mut Vec<String>) {
  sections.push("pub trait Palette: SkribbleValue {".into());

  for (name, _) in config.palette.iter() {
    let method_name = safe_method_name(name);

    sections.push(indent(
      format!("#[inline]\nfn {method_name}(&self) -> String {{"),
      indent_style,
    ));

    sections.push(indent(
      indent(
        format!("self.append_string_to_skribble_value(\"{name}\")"),
        indent_style,
      ),
      indent_style,
    ));

    sections.push(indent("}", indent_style));
  }

  sections.push("}".into());
}

fn generate_css_variables(
  config: &MergedConfig,
  variable_prefix: impl AsRef<str>,
  indent_style: IndentStyle,
  sections: &mut Vec<String>,
) {
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
    let method_name = safe_method_name(name);
    let variable_name = css_variable.get_variable(variable_prefix.as_ref());
    let css_docs = wrap_indent(
      wrap_docs(wrap_in_code_block(
        css_property_syntax(&variable_name, &css_variable.syntax, &css_variable.value),
        "css",
      )),
      1,
    );

    if let Some(ref description) = css_variable.description {
      entries.push(wrap_indent(wrap_docs(description), 1));
      entries.push(wrap_indent(wrap_docs("\n"), 1));
    }

    entries.push(css_docs.clone());

    entries.push(indent(
      format!("#[inline]\npub fn {method_name}(&self) -> String {{"),
      indent_style,
    ));

    entries.push(indent(
      indent(format!("\"{variable_name}\".into()",), indent_style),
      indent_style,
    ));

    entries.push(indent("}", indent_style));

    if css_variable.syntax.is_color() {
      if let Some(ref description) = css_variable.description {
        colors.push(wrap_indent(wrap_docs(description), 1));
        colors.push(wrap_indent(wrap_docs("\n"), 1));
      }

      colors.push(css_docs);

      colors.push(indent(
        format!("#[inline]\nfn {method_name}(&self) -> String {{"),
        indent_style,
      ));
      colors.push(indent(
        indent(
          format!("self.append_string_to_skribble_value(\"{name}\")"),
          indent_style,
        ),
        indent_style,
      ));
      colors.push(indent("}", indent_style))
    }
  }

  entries.push("}".into());
  colors.push("}".into());
  sections.push(entries.join("\n"));
  sections.push(colors.join("\n"));
}

fn wrap_indent(content: impl AsRef<str>, level: u8) -> String {
  let mut result = String::new();
  let indent_style = IndentStyle::default();
  for _ in 0..level {
    result = indent(&content, indent_style);
  }

  result
}

fn wrap_docs(content: impl AsRef<str>) -> String {
  let mut result = vec![];
  for line in content.as_ref().lines() {
    result.push(format!("/// {line}"));
  }

  result.join("\n")
}

fn css_property_syntax(
  variable_name: impl AsRef<str>,
  syntax: &PropertySyntax,
  initial_value: &Option<String>,
) -> String {
  let variable_name = variable_name.as_ref();
  let default_initial_value = "/* */".into();
  let initial_value = initial_value.as_ref().unwrap_or(&default_initial_value);
  format!(
    "@property {variable_name} {{\n  syntax: \"{syntax}\";\n  inherits: false;\n  initial-value: \
     {initial_value};\n}}"
  )
}

fn wrap_in_code_block(content: impl AsRef<str>, r#type: impl AsRef<str>) -> String {
  format!("```{}\n{}\n```", r#type.as_ref(), content.as_ref(),)
}

fn get_value_set_trait_name(value_set_name: impl Into<String>) -> String {
  format!("ValueSet{}", value_set_name.into().to_pascal_case())
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

fn get_method_name(name: impl AsRef<str>, method_names: &mut IndexSet<String>) -> String {
  let method_name = safe_method_name(name);
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

fn safe_method_name(name: impl AsRef<str>) -> String {
  let name = name.as_ref();

  let prefix = match name.chars().next() {
    Some(first_char) if first_char.is_ascii_digit() => "n_",
    Some(first_char) if !first_char.is_ascii_alphabetic() => {
      match first_char {
        '-' => "minus_",
        '+' => "plus_",
        _ => "__",
      }
    }
    _ => "",
  };

  let method_name = format!("{prefix}{}", name.to_snake_case());

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

const HEADER: &str = r#"// This file was generated by skribble.
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
  // let indent_style = IndentStyle::default();
  // let mut content = vec![];
  // content.push("pub use generated_skribble_module::sk;".into());
  // content.push("pub use generated_skribble_module::vars;".into());
  // content.push("use generated_skribble_module::*;".into());
  // content.push("mod generated_skribble_module {".into());
  // content.push(indent(HEADER, indent_style));
  // content.push(indent(sections.join("\n"), indent_style));
  // content.push("}".into());

  // content.join("\n")
  format!("{HEADER}\n{}", sections.join("\n"))
}

pub(crate) fn generate_file_contents(config: &MergedConfig, options: &Options) -> String {
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
