use oxc::{
  allocator::{Allocator, Vec},
  ast::ast::*,
  parser::Parser,
  span::SourceType,
};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use utils::unimplemented;

mod array;
mod assignment;
mod bigint;
mod binding;
mod chain;
mod expr;
mod function;
mod identifier;
mod literal;
mod member;
mod misc;
mod number;
mod object;
mod operator;
mod option;
mod private_identifier;
mod regexp;
mod statement;
mod template_literal;
mod utils;
mod vec;

pub fn js_to_oxc(source: &str) -> String {
  let allocator = Allocator::default();
  let source_type = SourceType::default();
  let ret = Parser::new(&allocator, source, source_type).parse();
  let gen = JsToOxc { ast_builder: quote! { self.ast_builder }, span: quote! { SPAN } };
  let tokens = gen.gen_program(&ret.program);
  tokens.to_string()
}

pub struct JsToOxc {
  pub ast_builder: TokenStream,
  pub span: TokenStream,
}

impl JsToOxc {
  pub fn gen_program(&self, node: &Program) -> TokenStream {
    let mut tokens = TokenStream::new();
    for stmt in &node.body {
      tokens.append_all(self.gen_statement(stmt));
    }
    tokens
  }
}
