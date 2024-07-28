use crate::JsToOxc;
use oxc::ast::ast::{CatchClause, CatchParameter};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_catch_clause(&self, node: &CatchClause) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let param = self.gen_option(&node.param, |param| self.gen_catch_parameter(param));
    let body = self.gen_block_statement(&node.body);
    quote! {
      #ast_builder.catch_clause(#span, #param, #body)
    }
  }

  pub(crate) fn gen_catch_parameter(&self, param: &CatchParameter) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let pattern = self.gen_binding_pattern(&param.pattern);
    quote! {
      #ast_builder.catch_parameter(#span, #pattern)
    }
  }
}
