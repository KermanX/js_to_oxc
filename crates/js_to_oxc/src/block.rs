use crate::JsToOxc;
use oxc::ast::ast::BlockStatement;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_block_statement(&self, node: &BlockStatement) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let body = self.gen_vec(&node.body, |statement| self.gen_statement(statement));
    quote! {
      #ast_builder.block_statement(#span, #body)
    }
  }
}
