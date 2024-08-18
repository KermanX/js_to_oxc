use crate::JsToOxc;
use oxc::ast::ast::{BindingPattern, BindingPatternKind, BindingProperty, BindingRestElement};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_binding_pattern(&self, element: &BindingPattern) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let kind = self.gen_binding_pattern_kind(&element.kind);
    let type_annotation = quote! { None::<TSTypeAnnotation> };
    let optional = element.optional;
    quote! {
      #ast_builder.binding_pattern(#kind, #type_annotation, #optional)
    }
  }

  fn gen_binding_pattern_kind(&self, kind: &BindingPatternKind) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match kind {
      BindingPatternKind::BindingIdentifier(identifier) => {
        let name = identifier.name.as_str();
        quote! {
          #ast_builder.binding_pattern_kind_binding_identifier(#span, #name)
        }
      }
      BindingPatternKind::ObjectPattern(pattern) => {
        let properties =
          self.gen_vec(&pattern.properties, |property| self.gen_binding_property(property));
        let rest = self.gen_option_with_type(&pattern.rest, "BindingRestElement", |rest| {
          self.gen_binding_rest_element(rest)
        });
        quote! {
          #ast_builder.binding_pattern_kind_object_pattern(#span, #properties, #rest)
        }
      }
      BindingPatternKind::ArrayPattern(pattern) => {
        let elements = self.gen_vec(&pattern.elements, |element| {
          self.gen_option(element, |element| self.gen_binding_pattern(element))
        });
        let rest = self.gen_option_with_type(&pattern.rest, "BindingRestElement", |rest| {
          self.gen_binding_rest_element(rest)
        });
        quote! {
          #ast_builder.binding_pattern_kind_array_pattern(#span, #elements, #rest)
        }
      }
      BindingPatternKind::AssignmentPattern(pattern) => {
        let left = self.gen_binding_pattern(&pattern.left);
        let right = self.gen_expression(&pattern.right);
        quote! {
          #ast_builder.binding_pattern_kind_assignment_pattern(#span, #left, #right)
        }
      }
    }
  }

  fn gen_binding_property(&self, property: &BindingProperty) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let key = self.gen_property_key(&property.key);
    let value = self.gen_binding_pattern(&property.value);
    let shorthand = property.shorthand;
    let computed = property.computed;
    quote! {
      #ast_builder.binding_property(#span, #key, #value, #shorthand, #computed)
    }
  }

  pub(crate) fn gen_binding_rest_element(&self, element: &BindingRestElement) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let argument = self.gen_binding_pattern(&element.argument);
    quote! {
      #ast_builder.binding_rest_element(#span, #argument)
    }
  }
}
