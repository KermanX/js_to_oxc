use crate::JsToOxc;
use oxc::ast::ast::{
  AccessorPropertyType, ClassBody, ClassElement, ClassType, MethodDefinitionKind,
  MethodDefinitionType, PropertyDefinitionType,
};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_class_type(&self, node: &ClassType) -> TokenStream {
    match node {
      ClassType::ClassDeclaration => {
        quote! { ClassType::ClassDeclaration }
      }
      ClassType::ClassExpression => {
        quote! { ClassType::ClassExpression }
      }
    }
  }

  pub(crate) fn gen_class_body(&self, node: &ClassBody) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let body = self.gen_vec(&node.body, |element| self.gen_class_element(element));
    quote! {
      #ast_builder.class_body(#span, #body)
    }
  }

  fn gen_class_element(&self, element: &ClassElement) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match element {
      ClassElement::StaticBlock(node) => {
        let body = self.gen_vec(&node.body, |statement| self.gen_statement(statement));
        quote! {
          #ast_builder.class_element_static_block(#span, #body)
        }
      }
      ClassElement::MethodDefinition(node) => {
        let r#type = self.gen_method_definition_type(&node.r#type);
        let decorators = quote! { #ast_builder.vec() };
        let key = self.gen_property_key(&node.key);
        let value = self.gen_function(&node.value);
        let kind = self.gen_method_definition_kind(&node.kind);
        let computed = node.computed;
        let r#static = node.r#static;
        let r#override = node.r#override;
        let optional = node.optional;
        let accessibility = quote! { None::<TSAccessibility> };
        quote! {
          #ast_builder.class_element_method_definition(
            #r#type,
            #span,
            #decorators,
            #key,
            #value,
            #kind,
            #computed,
            #r#static,
            #r#override,
            #optional,
            #accessibility,
          )
        }
      }
      ClassElement::PropertyDefinition(node) => {
        let r#type = self.gen_property_definition_type(&node.r#type);
        let decorators = quote! { #ast_builder.vec() };
        let key = self.gen_property_key(&node.key);
        let value =
          self.gen_option_with_type(&node.value, "Expression", |value| self.gen_expression(value));
        let computed = node.computed;
        let r#static = node.r#static;
        let declare = node.declare;
        let r#override = node.r#override;
        let optional = node.optional;
        let definite = node.definite;
        let readonly = node.readonly;
        let type_annotation = quote! { None::<TSTypeAnnotation> };
        let accessibility = quote! { None::<TSAccessibility> };
        quote! {
          #ast_builder.class_element_property_definition(
            #r#type,
            #span,
            #decorators,
            #key,
            #value,
            #computed,
            #r#static,
            #declare,
            #r#override,
            #optional,
            #definite,
            #readonly,
            #type_annotation,
            #accessibility,
          )
        }
      }
      ClassElement::AccessorProperty(node) => {
        let r#type = self.gen_accessor_property_type(&node.r#type);
        let decorators = quote! { #ast_builder.vec() };
        let key = self.gen_property_key(&node.key);
        let value =
          self.gen_option_with_type(&node.value, "Expression", |value| self.gen_expression(value));
        let computed = node.computed;
        let r#static = node.r#static;
        quote! {
          #ast_builder.class_element_accessor_property(
            #r#type,
            #span,
            #decorators,
            #key,
            #value,
            #computed,
            #r#static,
          )
        }
      }
      ClassElement::TSIndexSignature(_) => unimplemented!("ts"),
    }
  }

  fn gen_method_definition_type(&self, node: &MethodDefinitionType) -> TokenStream {
    match node {
      MethodDefinitionType::MethodDefinition => quote! { MethodDefinitionType::MethodDefinition },
      MethodDefinitionType::TSAbstractMethodDefinition => {
        quote! { MethodDefinitionType::TSAbstractMethodDefinition }
      }
    }
  }

  fn gen_method_definition_kind(&self, node: &MethodDefinitionKind) -> TokenStream {
    match node {
      MethodDefinitionKind::Constructor => quote! { MethodDefinitionKind::Constructor },
      MethodDefinitionKind::Method => quote! { MethodDefinitionKind::Method },
      MethodDefinitionKind::Get => quote! { MethodDefinitionKind::Get },
      MethodDefinitionKind::Set => quote! { MethodDefinitionKind::Set },
    }
  }

  fn gen_property_definition_type(&self, node: &PropertyDefinitionType) -> TokenStream {
    match node {
      PropertyDefinitionType::PropertyDefinition => {
        quote! { PropertyDefinitionType::PropertyDefinition }
      }
      PropertyDefinitionType::TSAbstractPropertyDefinition => {
        quote! { PropertyDefinitionType::TSAbstractPropertyDefinition }
      }
    }
  }

  fn gen_accessor_property_type(&self, node: &AccessorPropertyType) -> TokenStream {
    match node {
      AccessorPropertyType::AccessorProperty => quote! { AccessorPropertyType::AccessorProperty },
      AccessorPropertyType::TSAbstractAccessorProperty => {
        quote! { AbstractPropertyType::TSAbstractAccessorProperty }
      }
    }
  }
}
