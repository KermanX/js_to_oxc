use crate::JsToOxc;
use oxc::ast::ast::{Hashbang, Program};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub fn gen_program(&self, node: &Program) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let source_type = self.gen_source_type();
    let hashbang = self.gen_option(&node.hashbang, |hashbang| self.gen_hashbang(hashbang));
    let directives = self.gen_vec(&node.directives, |directive| self.gen_directive(directive));
    let body = self.gen_vec(&node.body, |statement| self.gen_statement(statement));
    quote! {
      #ast_builder.program(#span, #source_type, #hashbang, #directives, #body)
    }
  }

  fn gen_source_type(&self) -> TokenStream {
    quote! {
      SourceType::default().with_module(true)
    }
  }

  fn gen_hashbang(&self, hashbang: &Hashbang) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let value = hashbang.value.as_str();
    quote! {
      #ast_builder.hashbang(#span, #value)
    }
  }
}
