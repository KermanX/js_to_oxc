mod array;
mod assignment;
mod bigint;
mod binding;
mod block;
mod catch;
mod chain;
mod class;
mod declaration;
mod expr;
mod r#for;
mod function;
mod identifier;
mod literal;
mod member;
mod misc;
mod module;
mod number;
mod object;
mod operator;
mod option;
mod program;
mod regexp;
mod statement;
mod switch;
mod template_literal;
mod vec;

use proc_macro2::TokenStream;

pub struct JsToOxc {
  pub ast_builder: TokenStream,
  pub span: TokenStream,
}
