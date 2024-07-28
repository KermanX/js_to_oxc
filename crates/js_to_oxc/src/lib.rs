mod array;
mod assignment;
mod bigint;
mod binding;
mod chain;
mod expr;
mod function;
mod class;
mod identifier;
mod literal;
mod member;
mod misc;
mod number;
mod object;
mod operator;
mod option;
mod program;
mod regexp;
mod statement;
mod template_literal;
mod utils;
mod vec;

use proc_macro2::TokenStream;

pub struct JsToOxc {
  pub ast_builder: TokenStream,
  pub span: TokenStream,
}
