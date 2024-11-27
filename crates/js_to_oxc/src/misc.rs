use crate::JsToOxc;
use oxc::ast::ast::Argument;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_argument(&self, argument: &Argument) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match argument {
      Argument::SpreadElement(argument) => {
        let argument = self.gen_expression(&argument.argument);
        quote! {
          #ast_builder.argument_spread_element(#span, #argument),
        }
      }
      _ => {
        let expr = self.gen_expression(argument.to_expression());
        quote! {
          Argument::from(#expr),
        }
      }
    }
  }
}
