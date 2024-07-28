use crate::JsToOxc;
use oxc::ast::ast::{VariableDeclarationKind, VariableDeclarator};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
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
