use crate::JsToOxc;
use oxc::ast::ast::{BindingIdentifier, LabelIdentifier};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_binding_identifier(&self, identifier: &BindingIdentifier) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let name = identifier.name.as_str();
    quote! {
      #ast_builder.binding_identifier(#span, #name)
    }
  }

  pub(crate) fn gen_label_identifier(&self, identifier: &LabelIdentifier) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let name = identifier.name.as_str();
    quote! {
      #ast_builder.label_identifier(#span, #name)
    }
  }
}
