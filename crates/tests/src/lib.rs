use glob::Paths;
use js_to_oxc::JsToOxc;
use oxc::{
  allocator::Allocator, ast::ast::Expression, codegen::CodeGenerator, parser::Parser,
  span::SourceType,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};

pub fn generate_expr_tests(files: Paths) -> String {
  let mut tokens = TokenStream::new();

  for file in files {
    let file = file.unwrap();
    let name = file.file_stem().unwrap().to_str().unwrap();
    let source = std::fs::read_to_string(file.clone()).unwrap();
    tokens.append_all(expr_file(name, &source));
  }

  tokens = quote! {
    #[cfg(test)]
    mod tests_expr {
      use oxc::ast::ast::*;
      use oxc::span::SPAN;
      use oxc::syntax::number::{NumberBase, BigintBase};

      fn print_expr(expr: Expression) -> String {
        let mut codegen = oxc::codegen::CodeGenerator::new();
        codegen.print_expression(&expr);
        codegen.into_source_text()
      }

      #tokens
    }
  };

  tokens.to_string()
}

fn expr_file(name: &str, source: &str) -> TokenStream {
  let allocator = Allocator::default();
  let parser = Parser::new(&allocator, source, SourceType::default());
  let expr_arr = parser.parse_expression().unwrap();

  let js_to_oxc = JsToOxc { ast_builder: quote! { ast_builder }, span: quote! { SPAN } };

  let mut tokens = TokenStream::new();
  if let Expression::ArrayExpression(expr_arr) = expr_arr {
    for (i, expr) in expr_arr.elements.iter().enumerate() {
      let expr = expr.as_expression().unwrap();
      let code = js_to_oxc.gen_expression(expr);

      let mut codegen = CodeGenerator::new();
      codegen.print_expression(expr);
      let expected: String = codegen.into_source_text();

      let name = format_ident!("{}_{}", name, i + 2);

      tokens.append_all(quote! {
        #[test]
        fn #name() {
          let allocator = oxc::allocator::Allocator::default();
          let ast_builder = oxc::ast::AstBuilder::new(&allocator);
          let actual = { #code };
          assert_eq!(print_expr(actual), #expected);
        }
      })
    }
  } else {
    panic!("unexpected expression");
  }

  tokens
}
