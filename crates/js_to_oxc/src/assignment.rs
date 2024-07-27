use crate::{utils::unimplemented, JsToOxc};
use oxc::ast::ast::{AssignmentTarget, SimpleAssignmentTarget};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_assignment_target(&self, target: &AssignmentTarget) -> TokenStream {
    let ast_builder = &self.ast_builder;
    if target.is_simple_assignment_target() {
      let inner = self.gen_simple_assignment_target(target.to_simple_assignment_target());
      quote! {
        #ast_builder.assignment_target_simple(#inner)
      }
    } else {
      unimplemented()
    }
  }

  fn gen_simple_assignment_target(&self, target: &SimpleAssignmentTarget) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match target {
      SimpleAssignmentTarget::AssignmentTargetIdentifier(node) => {
        let name = node.name.as_str();
        quote! {
          #ast_builder.simple_assignment_target_identifier_reference(#span, #name)
        }
      }
      _ => {
        let node = target.to_member_expression();
        let inner = self.gen_member_expression(&node);
        quote! {
          #ast_builder.simple_assignment_target_member_expression(#span, #inner)
        }
      }
    }
  }
}
