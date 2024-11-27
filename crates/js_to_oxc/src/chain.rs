use crate::JsToOxc;
use oxc::ast::ast::ChainElement;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_chain_element(&self, element: &ChainElement) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    if let ChainElement::CallExpression(node) = element {
      let arguments = self.gen_vec(&node.arguments, |argument| self.gen_argument(argument));
      let callee = self.gen_expression(&node.callee);
      let optional = node.optional;
      quote! {
        #ast_builder.chain_element_call_expression(#span, #callee, NONE, #arguments, #optional)
      }
    } else {
      let node = element.to_member_expression();
      let inner = self.gen_member_expression(&node);
      quote! {
        ChainElement::from(#inner)
      }
    }
  }
}
