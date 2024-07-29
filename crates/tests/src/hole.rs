#[cfg(test)]
mod tests_hold {
  use js_to_oxc::JsToOxc;
  use oxc::{allocator::Allocator, ast::ast::Expression, parser::Parser, span::SourceType};
  use quote::quote;
  use std::{fs::read_to_string, path::PathBuf};

  #[test]
  fn holes() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_root = root.join("fixtures");
    let source = read_to_string(fixture_root.join("hole/hole.js")).unwrap();

    let allocator = Allocator::default();
    let parser = Parser::new(&allocator, source.as_str(), SourceType::default().with_module(true));
    let expr_arr = parser.parse_expression().unwrap();

    let js_to_oxc = JsToOxc { ast_builder: quote! { ast_builder }, span: quote! { SPAN } };

    if let Expression::ArrayExpression(expr_arr) = expr_arr {
      for (i, expr) in expr_arr.elements.iter().enumerate() {
        let expr = expr.as_expression().unwrap();
        let code = js_to_oxc.gen_expression(expr);

        let name = format!("{}", i + 2);

        insta::assert_snapshot!(name, code);
      }
    } else {
      panic!("unexpected expression");
    }
  }
}
