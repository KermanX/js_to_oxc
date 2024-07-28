use crate::{utils::unimplemented, JsToOxc};
use oxc::ast::ast::Statement;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub fn gen_statement(&self, node: &Statement) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      Statement::BlockStatement(node) => {
        let body = self.gen_vec(&node.body, |statement| self.gen_statement(statement));
        quote! {
          #ast_builder.block_statement(#span, #body)
        }
      }
      Statement::BreakStatement(node) => {
        let label = self.gen_option(&node.label, |label| self.gen_label_identifier(label));
        quote! {
          #ast_builder.break_statement(#span, #label)
        }
      }
      Statement::ContinueStatement(node) => {
        let label = self.gen_option(&node.label, |label| self.gen_label_identifier(label));
        quote! {
          #ast_builder.continue_statement(#span, #label)
        }
      }
      Statement::DebuggerStatement(_) => {
        quote! {
          #ast_builder.debugger_statement(#span)
        }
      }
      Statement::DoWhileStatement(_) => unimplemented(),
      Statement::EmptyStatement(_) => {
        quote! {
          #ast_builder.empty_statement(#span)
        }
      }
      Statement::ExpressionStatement(node) => {
        let expression = self.gen_expression(&node.expression);
        quote! {
          #ast_builder.statement_expression(#span, #expression)
        }
      }
      Statement::ForInStatement(_) => unimplemented(),
      Statement::ForOfStatement(_) => unimplemented(),
      Statement::ForStatement(_) => unimplemented(),
      Statement::IfStatement(_) => unimplemented(),
      Statement::LabeledStatement(_) => unimplemented(),
      Statement::ReturnStatement(node) => {
        let argument = self.gen_option(&node.argument, |argument| self.gen_expression(argument));
        quote! {
          #ast_builder.statement_return(#span, #argument)
        }
      }
      Statement::SwitchStatement(_) => unimplemented(),
      Statement::ThrowStatement(_) => unimplemented(),
      Statement::TryStatement(_) => unimplemented(),
      Statement::WhileStatement(_) => unimplemented(),
      Statement::WithStatement(_) => unimplemented(),
      _ => unimplemented(),
    }
  }
}
