use crate::JsToOxc;
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
      Statement::ForInStatement(node) => {
        let left = self.gen_for_statement_left(&node.left);
        let right = self.gen_expression(&node.right);
        let body = self.gen_statement(&node.body);
        quote! {
          #ast_builder.statement_for_in(#span, #left, #right, #body)
        }
      }
      Statement::ForOfStatement(node) => {
        let r#await = node.r#await;
        let left = self.gen_for_statement_left(&node.left);
        let right = self.gen_expression(&node.right);
        let body = self.gen_statement(&node.body);
        quote! {
          #ast_builder.statement_for_of(#span, #r#await, #left, #right, #body)
        }
      }
      Statement::ForStatement(node) => {
        let init = self.gen_option(&node.init, |init| self.gen_for_statement_init(init));
        let test = self.gen_option(&node.test, |test| self.gen_expression(test));
        let update = self.gen_option(&node.update, |update| self.gen_expression(update));
        let body = self.gen_statement(&node.body);
        quote! {
          #ast_builder.statement_for(#span, #init, #test, #update, #body)
        }
      }
      Statement::IfStatement(node) => {
        let test = self.gen_expression(&node.test);
        let consequent = self.gen_statement(&node.consequent);
        let alternate = self.gen_option(&node.alternate, |alternate| self.gen_statement(alternate));
        quote! {
          #ast_builder.statement_if(#span, #test, #consequent, #alternate)
        }
      }
      Statement::LabeledStatement(node) => {
        let label = self.gen_label_identifier(&node.label);
        let body = self.gen_statement(&node.body);
        quote! {
          #ast_builder.statement_labeled(#span, #label, #body)
        }
      }
      Statement::ReturnStatement(node) => {
        let argument = self.gen_option(&node.argument, |argument| self.gen_expression(argument));
        quote! {
          #ast_builder.statement_return(#span, #argument)
        }
      }
      Statement::SwitchStatement(node) => {
        let discriminant = self.gen_expression(&node.discriminant);
        let cases = self.gen_vec(&node.cases, |case| self.gen_switch_case(case));
        quote! {
          #ast_builder.statement_switch(#span, #discriminant, #cases)
        }
      }
      Statement::ThrowStatement(node) => {
        let argument = self.gen_expression(&node.argument);
        quote! {
          #ast_builder.statement_throw(#span, #argument)
        }
      }
      Statement::TryStatement(node) => {
        let block = self.gen_block_statement(&node.block);
        let handler = self.gen_option(&node.handler, |handler| self.gen_catch_clause(handler));
        let finalizer =
          self.gen_option(&node.finalizer, |finalizer| self.gen_block_statement(finalizer));
        quote! {
          #ast_builder.statement_try(#span, #block, #handler, #finalizer)
        }
      }
      Statement::WhileStatement(node) => {
        let test = self.gen_expression(&node.test);
        let body = self.gen_statement(&node.body);
        quote! {
          #ast_builder.statement_while(#span, #test, #body)
        }
      }
      Statement::WithStatement(node) => {
        let object = self.gen_expression(&node.object);
        let body = self.gen_statement(&node.body);
        quote! {
          #ast_builder.statement_with(#span, #object, #body)
        }
      }
      _ => {
        if node.is_declaration() {
          let node = node.to_declaration();
          let inner = self.gen_declaration(node);
          quote! {
            #ast_builder.statement_declaration(#inner)
          }
        } else {
          let node = node.to_module_declaration();
          let inner = self.gen_module_declaration(node);
          quote! {
              #ast_builder.statement_module_declaration(#inner)
          }
        }
      }
    }
  }
}
