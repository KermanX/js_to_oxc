use crate::JsToOxc;
use oxc::ast::ast::{
  AssignmentTarget, AssignmentTargetMaybeDefault, AssignmentTargetPattern,
  AssignmentTargetProperty, AssignmentTargetRest, SimpleAssignmentTarget,
};
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
      let inner = self.gen_assignment_target_pattern(target.to_assignment_target_pattern());
      quote! {
        #ast_builder.assignment_target_assignment_target_pattern(#inner)
      }
    }
  }

  pub(crate) fn gen_simple_assignment_target(
    &self,
    target: &SimpleAssignmentTarget,
  ) -> TokenStream {
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
          #ast_builder.simple_assignment_target_member_expression(#inner)
        }
      }
    }
  }

  fn gen_assignment_target_pattern(&self, node: &AssignmentTargetPattern) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      AssignmentTargetPattern::ArrayAssignmentTarget(node) => {
        let elements = self.gen_vec(&node.elements, |element| {
          self.gen_option(element, |element| self.gen_assignment_target_maybe_default(element))
        });
        let rest = self.gen_option_with_type(&node.rest, "AssignmentTargetRest", |rest| {
          self.gen_assignment_target_rest(rest)
        });
        let trailing_comma = self.gen_option(&node.trailing_comma, |_| quote! { #span });
        quote! {
          #ast_builder.assignment_target_pattern_array_assignment_target(#span, #elements, #rest, #trailing_comma)
        }
      }
      AssignmentTargetPattern::ObjectAssignmentTarget(node) => {
        let properties =
          self.gen_vec(&node.properties, |property| self.gen_assignment_target_property(property));
        let rest = self.gen_option_with_type(&node.rest, "AssignmentTargetRest", |rest| {
          self.gen_assignment_target_rest(rest)
        });
        quote! {
          #ast_builder.assignment_target_pattern_object_assignment_target(#span, #properties, #rest)
        }
      }
    }
  }

  fn gen_assignment_target_maybe_default(
    &self,
    node: &AssignmentTargetMaybeDefault,
  ) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(node) => {
        let binding = self.gen_assignment_target(&node.binding);
        let init = self.gen_expression(&node.init);
        quote! {
          #ast_builder.assignment_target_maybe_default_assignment_target_with_default(#span, #binding, #init)
        }
      }
      _ => {
        let node = node.to_assignment_target();
        let inner = self.gen_assignment_target(node);
        quote! {
          #ast_builder.assignment_target_maybe_default_assignment_target(#inner)
        }
      }
    }
  }

  fn gen_assignment_target_rest(&self, node: &AssignmentTargetRest) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let target = self.gen_assignment_target(&node.target);
    quote! {
      #ast_builder.assignment_target_rest(#span, #target)
    }
  }

  fn gen_assignment_target_property(&self, node: &AssignmentTargetProperty) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(node) => {
        let binding = self.gen_identifier_reference(&node.binding);
        let init = self.gen_option(&node.init, |init| self.gen_expression(init));
        quote! {
          #ast_builder.assignment_target_property_assignment_target_property_identifier(#span, #binding, #init)
        }
      }
      AssignmentTargetProperty::AssignmentTargetPropertyProperty(node) => {
        let name = self.gen_property_key(&node.name);
        let binding = self.gen_assignment_target_maybe_default(&node.binding);
        quote! {
          #ast_builder.assignment_target_property_assignment_target_property_property(#span, #name, #binding)
        }
      }
    }
  }
}
