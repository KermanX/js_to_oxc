use crate::JsToOxc;
use oxc::ast::ast::{Declaration, VariableDeclarationKind, VariableDeclarator};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_declaration(&self, node: &Declaration) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      Declaration::VariableDeclaration(node) => {
        let kind = self.gen_variable_declaration_kind(&node.kind);
        let declarations =
          self.gen_vec(&node.declarations, |declarator| self.gen_variable_declarator(declarator));
        let declare = node.declare;
        quote! {
          #ast_builder.declaration_variable(#span, #kind, #declarations, #declare)
        }
      }
      Declaration::FunctionDeclaration(node) => {
        let r#type = self.gen_function_type(&node.r#type);
        let id = self.gen_option(&node.id, |id| self.gen_binding_identifier(id));
        let generator = node.generator;
        let r#async = node.r#async;
        let declare = node.declare;
        let type_parameters = quote! { Option::<TSTypeParameterDeclaration>::None };
        let this_param = quote! { Option::<TSThisParameter>::None };
        let params = self.gen_formal_parameters(&node.params);
        let return_type = quote! { Option::<TSTypeAnnotation>::None };
        let body = self.gen_option(&node.body, |body| self.gen_function_body(body));
        quote! {
            #ast_builder.declaration_function(#r#type, #span, #id, #generator, #r#async, #declare, #type_parameters, #this_param, #params, #return_type, #body)
        }
      }
      Declaration::ClassDeclaration(node) => {
        let r#type = self.gen_class_type(&node.r#type);
        let decorators = quote! { #ast_builder.vec() };
        let id = self.gen_option(&node.id, |id| self.gen_binding_identifier(id));
        let type_parameters = quote! { Option::<TSTypeParameterDeclaration>::None };
        let super_class =
          self.gen_option(&node.super_class, |super_class| self.gen_expression(super_class));
        let super_type_parameters = quote! { Option::<TSTypeParameterInstantiation>::None };
        let implements = quote! { None };
        let body = self.gen_class_body(&node.body);
        let r#abstract = node.r#abstract;
        let declare = node.declare;
        quote! {
          #ast_builder.declaration_class(#r#type, #span, #decorators, #id, #type_parameters, #super_class, #super_type_parameters, #implements, #body, #r#abstract, #declare)
        }
      }
      Declaration::UsingDeclaration(node) => {
        let is_await = node.is_await;
        let declarations =
          self.gen_vec(&node.declarations, |declarator| self.gen_variable_declarator(declarator));
        quote! {
          #ast_builder.declaration_using(#span, #is_await, #declarations)
        }
      }
      _ => unimplemented!("ts"),
    }
  }

  pub(crate) fn gen_variable_declaration_kind(
    &self,
    kind: &VariableDeclarationKind,
  ) -> TokenStream {
    match kind {
      VariableDeclarationKind::Var => quote! { VariableDeclarationKind::Var },
      VariableDeclarationKind::Let => quote! { VariableDeclarationKind::Let },
      VariableDeclarationKind::Const => quote! { VariableDeclarationKind::Const },
    }
  }

  pub(crate) fn gen_variable_declarator(&self, node: &VariableDeclarator) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let kind = self.gen_variable_declaration_kind(&node.kind);
    let id = self.gen_binding_pattern(&node.id);
    let init = self.gen_option(&node.init, |init| self.gen_expression(init));
    let definite = node.definite;
    quote! {
      #ast_builder.variable_declarator(#span, #kind, #id, #init, #definite)
    }
  }
}
