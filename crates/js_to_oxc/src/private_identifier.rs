use crate::JsToOxc;
use oxc::ast::ast::PrivateIdentifier;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_private_identifier_base(&self, identifier: &PrivateIdentifier) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let name = identifier.name.as_str();
    quote! {
      #ast_builder.private_identifier(
        #span,
        #name,
      )
    }
  }
}
