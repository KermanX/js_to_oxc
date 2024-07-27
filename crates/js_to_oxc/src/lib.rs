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
mod chain;
mod expr;
mod member;
mod number;
mod object;
mod operator;
mod option;
mod private_identifier;
mod regexp;
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
  pub fn gen_program<'ast>(&self, node: &Program<'ast>) -> TokenStream {
    let mut tokens = TokenStream::new();
    for stmt in &node.body {
      tokens.append_all(self.gen_statement(stmt));
    }
    tokens
  }

  pub fn gen_statement<'ast>(&self, node: &Statement<'ast>) -> TokenStream {
    let mut tokens = TokenStream::new();
    match node {
      Statement::BlockStatement(block) => {
        for stmt in &block.body {
          tokens.append_all(self.gen_statement(stmt));
        }
      }
      Statement::ExpressionStatement(expr) => {
        tokens.append_all(self.gen_expression(&expr.expression));
      }
      _ => tokens.append_all(unimplemented()),
    }
    tokens
  }

  fn gen_arguments<'ast>(&self, node: &Vec<'ast, Argument<'ast>>) -> TokenStream {
    let mut arguments = TokenStream::new();
    let ast_builder = &self.ast_builder;
    for arg in node {
      let arg = self.gen_argument(arg);
      arguments.append_all(quote! {
          __arguments.push(#arg);
      });
    }
    quote! {
        {
            let mut __arguments = #ast_builder.vec();
            #arguments
            __arguments
        }
    }
  }

  fn gen_argument<'ast>(&self, node: &Argument<'ast>) -> TokenStream {
    let mut tokens = TokenStream::new();
    let ast_builder = &self.ast_builder;
    match node {
      Argument::SpreadElement(_node) => tokens.append_all(unimplemented()),
      _ => {
        let expr = self.gen_expression(node.to_expression());
        tokens.append_all(quote! {
            #ast_builder.argument_expression(#expr),
        });
      }
    }
    tokens
  }
}
