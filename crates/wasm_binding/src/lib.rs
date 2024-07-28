use js_to_oxc::JsToOxc;
use oxc::{allocator::Allocator, parser::Parser, span::SourceType};
use quote::quote;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_expression(source: &str) -> String {
  let allocator = Allocator::default();
  let source_type = SourceType::default();
  let expression = Parser::new(&allocator, source, source_type).parse_expression().unwrap();
  let gen = JsToOxc { ast_builder: quote! { self.ast_builder }, span: quote! { SPAN } };
  let tokens = gen.gen_expression(&expression);
  tokens.to_string()
}

#[wasm_bindgen]
pub fn generate_program(source: &str) -> String {
  let allocator = Allocator::default();
  let source_type = SourceType::default();
  let program = Parser::new(&allocator, source, source_type).parse().program;
  let gen = JsToOxc { ast_builder: quote! { self.ast_builder }, span: quote! { SPAN } };
  let tokens = gen.gen_program(&program);
  tokens.to_string()
}
