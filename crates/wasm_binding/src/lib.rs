use js_to_oxc::JsToOxc;
use oxc::{allocator::Allocator, parser::Parser, span::SourceType};
use quote::quote;
use wasm_bindgen::prelude::*;

fn apply_aliases(code: String, ast_builder: &str, span: &str) -> String {
  code
    .replace("__ast_builder", ast_builder)
    .replace("__span", span)
}

#[wasm_bindgen]
pub fn generate_expression(source: &str, ast_builder: &str, span: &str) -> String {
  let allocator = Allocator::default();
  let source_type = SourceType::default();
  let expression = Parser::new(&allocator, source, source_type).parse_expression().unwrap();
  let gen = JsToOxc { ast_builder: quote! { __ast_builder }, span: quote! { __span } };
  let tokens = gen.gen_expression(&expression);
  apply_aliases(tokens.to_string(), ast_builder, span)
}

#[wasm_bindgen]
pub fn generate_program(source: &str, ast_builder: &str, span: &str) -> String {
  let allocator = Allocator::default();
  let source_type = SourceType::default();
  let program = Parser::new(&allocator, source, source_type).parse().program;
  let gen = JsToOxc { ast_builder: quote! { __ast_builder }, span: quote! { __span } };
  let tokens = gen.gen_program(&program);
  apply_aliases(tokens.to_string(), ast_builder, span)
}
