use crate::JsToOxc;
use oxc::ast::ast::ArrayExpressionElement;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_array_expression_element(
    &self,
    element: &ArrayExpressionElement,
  ) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match element {
      ArrayExpressionElement::SpreadElement(node) => {
        let expr = self.gen_expression(&node.argument);
        quote! {
          #ast_builder.array_expression_element_spread_element(#span, #expr)
        }
      }
      ArrayExpressionElement::Elision(_) => {
        quote! {
          #ast_builder.array_expression_element_elision(#span)
        }
      }
      _ => {
        let expr = self.gen_expression(element.to_expression());
        quote! {
          ArrayExpressionElement::from(#expr)
        }
      }
    }
  }
}
