use glob::Paths;
use js_to_oxc::JsToOxc;
use oxc::{
  allocator::Allocator, ast::ast::Expression, codegen::CodeGenerator, parser::Parser,
  span::SourceType,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use std::env;

pub fn generate_tests<M>(name: &str, files: Paths, generator: M) -> String
where
  M: Fn(&str, &str) -> TokenStream,
{
  let mut tokens = TokenStream::new();
  let run_complex = env::var("COMPLEX_TESTS").is_ok();

  for file in files {
    let file = file.unwrap();
    let name = file.file_stem().unwrap().to_str().unwrap();
    let source = std::fs::read_to_string(file.clone()).unwrap();
    if !run_complex && source.contains("@js_to_oxc:complex") {
      continue;
    }
    tokens.append_all(generator(name, &source));
  }

  let mod_name = format_ident!("tests_{}", name);

  tokens = quote! {
    #[cfg(test)]
    mod #mod_name {
      use oxc::ast::ast::*;
      use oxc::span::{SourceType, SPAN};
      use oxc::syntax::number::{NumberBase, BigintBase};
      use oxc::syntax::operator::*;

      fn print_expr(expr: Expression) -> String {
        let mut codegen = oxc::codegen::CodeGenerator::new();
        codegen.print_expression(&expr);
        codegen.into_source_text()
      }

      fn print_program(program: Program) -> String {
        let codegen = oxc::codegen::CodeGenerator::new();
        codegen.build(&program).source_text
      }

      #tokens
    }
  };

  tokens.to_string()
}

pub fn generate_expr_tests(name: &str, source: &str) -> TokenStream {
  let allocator = Allocator::default();
  let parser = Parser::new(&allocator, source, SourceType::default().with_module(true));
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

pub fn generate_stmt_tests(name: &str, source: &str) -> TokenStream {
  let allocator = Allocator::default();
  let parser = Parser::new(&allocator, source, SourceType::default().with_module(true));
  let program = parser.parse().program;

  let js_to_oxc = JsToOxc { ast_builder: quote! { ast_builder }, span: quote! { SPAN } };

  let code = js_to_oxc.gen_program(&program);

  let codegen = CodeGenerator::new();
  let expected = codegen.build(&program).source_text;

  let name = format_ident!("{}", name);

  quote! {
    #[test]
    fn #name() {
      let allocator = oxc::allocator::Allocator::default();
      let ast_builder = oxc::ast::AstBuilder::new(&allocator);
      let actual = { #code };
      assert_eq!(print_program(actual), #expected);
    }
  }
}
