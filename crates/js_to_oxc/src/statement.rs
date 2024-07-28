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
          #ast_builder.statement_block(#span, #body)
        }
      }
      Statement::BreakStatement(node) => {
        let label = self.gen_option(&node.label, |label| self.gen_label_identifier(label));
        quote! {
          #ast_builder.statement_break(#span, #label)
        }
      }
      Statement::ContinueStatement(node) => {
        let label = self.gen_option(&node.label, |label| self.gen_label_identifier(label));
        quote! {
          #ast_builder.statement_continue(#span, #label)
        }
      }
      Statement::DebuggerStatement(_) => {
        quote! {
          #ast_builder.statement_debugger(#span)
        }
      }
      Statement::DoWhileStatement(node) => {
        let body = self.gen_statement(&node.body);
        let test = self.gen_expression(&node.test);
        quote! {
          #ast_builder.statement_do_while(#span, #body, #test)
        }
      }
      Statement::EmptyStatement(_) => {
        quote! {
          #ast_builder.statement_empty(#span)
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
