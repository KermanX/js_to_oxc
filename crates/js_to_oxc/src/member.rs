use crate::JsToOxc;
use oxc::ast::ast::MemberExpression;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_member_expression(&self, node: &MemberExpression) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      MemberExpression::StaticMemberExpression(node) => {
        let object = self.gen_expression(&node.object);
        let property = self.gen_identifier_name(&node.property);
        let optional = node.optional;
        quote! {
          #ast_builder.member_expression_static(
            #span,
            #object,
            #property,
            #optional,
          )
        }
      }
      MemberExpression::ComputedMemberExpression(node) => {
        let object = self.gen_expression(&node.object);
        let expression = self.gen_expression(&node.expression);
        let optional = node.optional;
        quote! {
          #ast_builder.member_expression_computed(
            #span,
            #object,
            #expression,
            #optional,
          )
        }
      }
      MemberExpression::PrivateFieldExpression(node) => {
        let object = self.gen_expression(&node.object);
        let field = self.gen_private_identifier(&node.field);
        let optional = node.optional;
        quote! {
          #ast_builder.member_expression_private_field(
            #span,
            #object,
            #field,
            #optional,
          )
        }
      }
    }
  }
}
