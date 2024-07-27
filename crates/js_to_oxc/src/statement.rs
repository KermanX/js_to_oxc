use crate::{utils::unimplemented, JsToOxc};
use oxc::ast::ast::Statement;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;

impl JsToOxc {
  pub fn gen_statement(&self, node: &Statement) -> TokenStream {
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
}
