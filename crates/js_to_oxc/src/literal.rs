use crate::JsToOxc;
use oxc::ast::ast::{StringLiteral, TemplateLiteral};
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

  pub(crate) fn gen_template_literal(&self, node: &TemplateLiteral) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let quasis = self.gen_vec(&node.quasis, |quasi| self.gen_template_element(quasi));
    let expressions = self.gen_vec(&node.expressions, |expression| self.gen_expression(expression));
    quote! {
      #ast_builder.template_literal(#span, #quasis, #expressions)
    }
  }
}
