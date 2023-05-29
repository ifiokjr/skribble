use std::fmt::Write;

use indexmap::IndexMap;
use rstml::node::Node;
use rstml::node::NodeAttribute;
use rstml::node::NodeBlock;
use rstml::parse2;
use skribble_core::AnyResult;
use skribble_core::Arguments;
use skribble_core::AtomType;
use skribble_core::ClassFactory;
use skribble_core::Classes;
use skribble_core::RunnerConfig;
use syn::visit;
use syn::visit::Visit;
use syn::File;

use crate::generate::COLORS_PREFIX;
use crate::generate::GLOBAL_PREFIX;
use crate::generate::KEYFRAMES_PREFIX;
use crate::generate::TRANSFORMER_PREFIX;

#[readonly::make]
struct ScanVisitor<'config, 'names> {
  pub classes: Classes,
  config: &'config RunnerConfig,
  method_names: &'names IndexMap<String, String>,
}

impl<'config, 'names> ScanVisitor<'config, 'names> {
  pub fn new(
    config: &'config RunnerConfig,
    method_names: &'names IndexMap<String, String>,
  ) -> Self {
    Self {
      classes: Classes::default(),
      config,
      method_names,
    }
  }

  fn update_with_tokens(&mut self, tokens: &[String], transformers: &[String], arguments: &String) {
    if !starts_with_sk(tokens) {
      return;
    }

    let mut factory = ClassFactory::new(self.config);
    let arguments = if arguments.is_empty() {
      None
    } else {
      Some(Arguments::from(arguments))
    };

    if let Some(tokens) = tokens.get(1..) {
      for token in tokens {
        let token_identifier =
          if token == "__" && arguments.as_ref().map(|args| args.is_kv()).unwrap_or(false) {
            continue;
          } else if token.ends_with('_') {
            token.trim_end_matches('_')
          } else {
            token
          };

        let key = match factory.get_atom_type() {
          Some(AtomType::Color) => {
            format!("{COLORS_PREFIX}:::{token_identifier}")
          }
          Some(AtomType::Keyframes) => {
            format!("{KEYFRAMES_PREFIX}:::{token_identifier}")
          }
          Some(_) => {
            if let Some(atom) = factory.get_atom() {
              format!("{atom}:::{token_identifier}")
            } else {
              String::new()
            }
          }
          None => format!("{GLOBAL_PREFIX}:::{token_identifier}"),
        };

        let token = if let Some(value) = self.method_names.get(&key) {
          value
        } else {
          token_identifier
        };

        factory.add_token(token);
      }
    }

    for transformer in transformers.iter() {
      factory.add_transformer(&transformer.into());
    }

    if let Some(value) = arguments {
      factory.add_argument(value);
    }

    self.classes.insert_factory(factory);
  }

  fn visit_leptos_view_macro(&mut self, node: &syn::Macro) -> bool {
    // check if this is a leptos view (by name for now)
    let Some(view_segment) = node.path.segments.last() else {
      return false;
    };

    if view_segment.ident != "view" {
      return false;
    }

    let Ok(nodes) = parse2(node.tokens.clone()) else {
      return false;
    };

    self.visit_leptos_nodes(&nodes);
    true
  }

  fn visit_leptos_nodes(&mut self, nodes: &[Node]) {
    for node in nodes.iter() {
      match node {
        Node::Fragment(fragment) => self.visit_leptos_nodes(&fragment.children),
        Node::Element(element) => {
          self.visit_leptos_nodes(&element.children);

          let attributes = element.attributes();

          for attribute in attributes.iter() {
            match attribute {
              NodeAttribute::Block(NodeBlock::ValidBlock(block)) => self.visit_block(block),
              NodeAttribute::Attribute(attribute) => {
                let Some(keyed_attribute) = attribute.possible_value.as_ref() else {
                  continue;
                };

                self.visit_expr(&keyed_attribute.value);
              }
              _ => {}
            }
          }
        }
        Node::Block(NodeBlock::ValidBlock(block)) => {
          self.visit_block(block);
        }

        _ => {}
      }
    }
  }

  fn update_skribble_method(&mut self, node: &syn::ExprMethodCall) -> bool {
    let mut tokens = vec![node.method.to_string()];
    let mut arguments = String::new();
    let mut transformers = vec![];
    read_arguments_from_method_call(node, &mut arguments);
    read_tokens_from_expression(
      self.method_names,
      node.receiver.as_ref(),
      &mut tokens,
      &mut transformers,
    );
    self.update_with_tokens(&tokens, &transformers, &arguments);

    starts_with_sk(&tokens)
  }
}

impl<'ast, 'config, 'names> Visit<'ast> for ScanVisitor<'config, 'names> {
  fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
    if !self.update_skribble_method(node) {
      visit::visit_expr_method_call(self, node);
    }
  }

  fn visit_macro(&mut self, node: &'ast syn::Macro) {
    if self.visit_leptos_view_macro(node) {
      return;
    }

    visit::visit_macro(self, node);
  }
}

fn read_arguments_from_method_call(method: &syn::ExprMethodCall, arguments: &mut dyn Write) {
  match (method.args.len(), method.args.first(), method.args.last()) {
    (1, Some(syn::Expr::Lit(lit)), _) => {
      if let syn::Lit::Str(value_literal) = &lit.lit {
        write!(arguments, "{}", value_literal.value()).unwrap();
      }
    }
    (2, Some(syn::Expr::Lit(property)), Some(syn::Expr::Lit(value))) => {
      if let (syn::Lit::Str(property_literal), syn::Lit::Str(value_literal)) =
        (&property.lit, &value.lit)
      {
        write!(
          arguments,
          "{}={}",
          property_literal.value(),
          value_literal.value()
        )
        .ok();
      }
    }
    _ => {}
  }
}

fn read_tokens_from_expression(
  method_names: &IndexMap<String, String>,
  node: &syn::Expr,
  tokens: &mut Vec<String>,
  transformers: &mut Vec<String>,
) {
  match node {
    syn::Expr::MethodCall(method) => {
      let method_name = method.method.to_string();
      let mut arguments = String::new();
      read_arguments_from_method_call(method, &mut arguments);
      let key = format!("{}:::{method_name}", TRANSFORMER_PREFIX);

      if let Some(transformer) = method_names.get(&key) {
        if arguments.is_empty() {
          transformers.push(transformer.into());
        } else {
          transformers.push(format!("{transformer}={arguments}"));
        }
      } else {
        tokens.insert(0, method_name);
      }

      read_tokens_from_expression(method_names, method.receiver.as_ref(), tokens, transformers);
    }
    syn::Expr::Call(call) => {
      read_tokens_from_expression(method_names, call.func.as_ref(), tokens, transformers);
    }
    syn::Expr::Path(path) => {
      if let Some(path) = path
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
      {
        tokens.insert(0, path);
      }
    }
    _ => {}
  }
}

pub(crate) fn scan(
  config: &RunnerConfig,
  _file_path: impl AsRef<str>,
  content: impl AsRef<str>,
  method_names: &IndexMap<String, String>,
) -> AnyResult<Classes> {
  let mut visitor = ScanVisitor::new(config, method_names);
  let syntax_tree: File = syn::parse_str(content.as_ref())?;

  visitor.visit_file(&syntax_tree);

  Ok(visitor.classes)
}

fn starts_with_sk(tokens: &[String]) -> bool {
  tokens.first() == Some(&"sk".to_string())
}
