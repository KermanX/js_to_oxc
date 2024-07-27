use crate::JsToOxc;
use oxc::ast::ast::StringLiteral;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_string_literal(&self, node: &StringLiteral) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let value = node.value.as_str();
    quote! {
      #ast_builder.string_literal(#span, #value)
    }
  }
}
