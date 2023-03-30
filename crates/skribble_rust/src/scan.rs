use std::path::Path;

use skribble_core::AnyResult;
use skribble_core::ClassFactory;
use skribble_core::Classes;
use skribble_core::RunnerConfig;
use syn::visit;
use syn::visit::Visit;
use syn::File;

#[readonly::make]
struct ScanVisitor<'config> {
  pub classes: Classes,
  config: &'config RunnerConfig,
}

impl<'config> ScanVisitor<'config> {
  pub fn new(config: &'config RunnerConfig) -> Self {
    Self {
      classes: Classes::default(),
      config,
    }
  }

  fn visit_receiver(&mut self, node: &syn::Expr, tokens: &mut Vec<String>) {
    match node {
      syn::Expr::MethodCall(method) => {
        tokens.insert(0, method.method.to_string());
        self.visit_receiver(method.receiver.as_ref(), tokens);
      }
      syn::Expr::Call(call) => {
        self.visit_receiver(call.func.as_ref(), tokens);
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
}

impl<'ast, 'config> Visit<'ast> for ScanVisitor<'config> {
  fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
    // Recursively visit any nested items
    visit::visit_item_fn(self, node);
  }

  fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
    let mut tokens = vec![node.method.to_string()];
    println!("Expression: {:#?}", node);
    // println!("ParentToken: {:#?}", node.paren_token);
    // find out if the root of this method call is the `sk` function
    self.visit_receiver(node.receiver.as_ref(), &mut tokens);

    println!("{:#?}", tokens);
    // let method = node.method.to_string();
    // println!("Method: {}", method);
    if tokens.first() != Some(&"sk".to_string()) {
      return;
    }

    let mut factory = ClassFactory::new(self.config);
    for token in tokens[1..].iter() {
      factory.add_token(token);
    }

    self.classes.insert_factory(factory);
  }
}

pub(crate) fn scan(
  config: &RunnerConfig,
  _file_path: impl AsRef<Path>,
  bytes: Vec<u8>,
) -> AnyResult<Classes> {
  let mut visitor = ScanVisitor::new(config);
  let syntax_tree: File = syn::parse_str(String::from_utf8(bytes)?.as_str())?;

  visitor.visit_file(&syntax_tree);

  Ok(visitor.classes)
}

#[cfg(test)]
mod tests {
  use skribble_core::AnyEmptyResult;
  use skribble_core::PluginContainer;
  use skribble_core::SkribbleRunner;
  use skribble_core::StyleConfig;
  use skribble_core::ToSkribbleCss;
  use skribble_preset::PresetDefault;

  use super::*;
  use crate::RustPlugin;

  #[test]
  fn can_scan_expressions() -> AnyEmptyResult {
    let default_preset = PresetDefault::builder().build();
    let rust_plugin = RustPlugin::builder().build();

    let config: StyleConfig = StyleConfig::builder()
      .plugins(vec![
        PluginContainer::from(default_preset),
        PluginContainer::from(rust_plugin),
      ])
      .build();

    let mut runner = SkribbleRunner::new(config);
    let config = runner.initialize()?;
    let code = br#"
      pub fn foo() {
        let something = sk().p().px();
      }

      struct Awesome {
        pub foo: String,
      }

      impl Awesome {
        pub fn bar(&self) {
          let another_one = sk().md().p().px();
          let contained = sk().contained();
        }
      }
    "#;
    let classes = scan(config, "", code.to_vec())?;

    insta::assert_display_snapshot!(classes.to_skribble_css(config)?);

    Ok(())
  }
}
