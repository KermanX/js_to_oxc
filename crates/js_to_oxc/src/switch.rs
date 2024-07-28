use crate::JsToOxc;
use oxc::ast::ast::SwitchCase;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_switch_case(&self, node: &SwitchCase) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let test = self.gen_option(&node.test, |test| self.gen_expression(test));
    let consequent = self.gen_vec(&node.consequent, |statement| self.gen_statement(statement));
    quote! {
      #ast_builder.switch_case(#span, #test, #consequent)
    }
  }
}
