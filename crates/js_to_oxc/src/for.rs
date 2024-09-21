use crate::JsToOxc;
use oxc::ast::ast::{ForStatementInit, ForStatementLeft};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_for_statement_left(&self, node: &ForStatementLeft) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      ForStatementLeft::VariableDeclaration(node) => {
        let kind = self.gen_variable_declaration_kind(&node.kind);
        let declarations =
          self.gen_vec(&node.declarations, |decl| self.gen_variable_declarator(decl));
        let declare = node.declare;
        quote! {
          #ast_builder.for_statement_left_variable_declaration(#span, #kind, #declarations, #declare)
        }
      }
      _ => {
        let node = node.to_assignment_target();
        let inner = self.gen_assignment_target(node);
        quote! {
          #ast_builder.for_statement_left_assignment_target(#span, #inner)
        }
      }
    }
  }

  pub(crate) fn gen_for_statement_init(&self, node: &ForStatementInit) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      ForStatementInit::VariableDeclaration(node) => {
        let kind = self.gen_variable_declaration_kind(&node.kind);
        let declarations =
          self.gen_vec(&node.declarations, |decl| self.gen_variable_declarator(decl));
        let declare = node.declare;
        quote! {
          #ast_builder.for_statement_init_variable_declaration(#span, #kind, #declarations, #declare)
        }
      }
      _ => {
        let node = node.to_expression();
        let inner = self.gen_expression(node);
        quote! {
          #ast_builder.for_statement_init_expression(#inner)
        }
      }
    }
  }
}
