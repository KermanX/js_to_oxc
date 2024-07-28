use js_to_oxc::JsToOxc;
use oxc::{allocator::Allocator, diagnostics::OxcDiagnostic, parser::Parser, span::SourceType};
use quote::quote;
use wasm_bindgen::prelude::*;

fn apply_aliases(code: String, ast_builder: &str, span: &str) -> String {
  code.replace("__ast_builder", ast_builder).replace("__span", span)
}

#[wasm_bindgen(getter_with_clone)]
pub struct JsToOxcResult {
  pub result: String,
  pub errors: Option<String>,
}

#[wasm_bindgen]
pub fn generate_expression(source: &str, ast_builder: &str, span: &str) -> JsToOxcResult {
  let allocator = Allocator::default();
  let source_type = SourceType::default();
  let expression = Parser::new(&allocator, source, source_type).parse_expression();
  if let Err(errors) = expression {
    return JsToOxcResult { result: String::new(), errors: Some(format_errors(errors)) };
  }
  let gen = JsToOxc { ast_builder: quote! { __ast_builder }, span: quote! { __span } };
  let tokens = gen.gen_expression(&expression.unwrap());
  JsToOxcResult { result: apply_aliases(tokens.to_string(), ast_builder, span), errors: None }
}

#[wasm_bindgen]
pub fn generate_program(source: &str, ast_builder: &str, span: &str) -> JsToOxcResult {
  let allocator = Allocator::default();
  let source_type = SourceType::default();
  let parsed = Parser::new(&allocator, source, source_type).parse();
  let program = parsed.program;
  let gen = JsToOxc { ast_builder: quote! { __ast_builder }, span: quote! { __span } };
  let tokens = gen.gen_program(&program);
  let result = apply_aliases(tokens.to_string(), ast_builder, span);
  JsToOxcResult { result, errors: Some(format_errors(parsed.errors)) }
}

fn format_errors(errors: Vec<OxcDiagnostic>) -> String {
  errors.iter().map(|err| format!("{}", *err)).collect::<Vec<String>>().join("\n")
}
