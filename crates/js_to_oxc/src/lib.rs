use oxc::{
    allocator::{Allocator, Vec},
    ast::ast::*,
    parser::Parser,
    span::SourceType,
};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

pub fn js_to_oxc(source: &str) -> String {
    let allocator = Allocator::default();
    let source_type = SourceType::default();
    let ret = Parser::new(&allocator, source, source_type).parse();
    let gen = JsToOxc {
        ast_builder: quote! { self.ast_builder },
        span: quote! { SPAN },
    };
    let tokens = gen.gen_program(&ret.program);
    tokens.to_string()
}

struct JsToOxc {
    pub ast_builder: TokenStream,
    pub span: TokenStream,
}

impl JsToOxc {
    pub fn gen_program<'ast>(&self, node: &Program<'ast>) -> TokenStream {
        let mut tokens = TokenStream::new();
        for stmt in &node.body {
            tokens.append_all(self.gen_statement(stmt));
        }
        tokens
    }

    fn gen_statement<'ast>(&self, node: &Statement<'ast>) -> TokenStream {
        let mut tokens = TokenStream::new();
        match node {
            Statement::BlockStatement(block) => {
                for stmt in &block.body {
                    tokens.append_all(self.gen_statement(stmt));
                }
            }
            Statement::ExpressionStatement(expr) => {
                tokens.append_all(self.gen_expression(&expr.expression));
            }
            _ => unimplemented!(),
        }
        tokens
    }

    fn gen_expression<'ast>(&self, node: &Expression<'ast>) -> TokenStream {
        let mut tokens = TokenStream::new();
        let ast_builder = &self.ast_builder;
        let span = &self.span;
        match node {
            Expression::CallExpression(node) => {
                let arguments = self.gen_arguments(&node.arguments);
                let callee = self.gen_expression(&node.callee);
                let optional = node.optional;
                tokens.append_all(quote! {
                    #ast_builder.expression_call(#span, #arguments, #callee, Option::<TSTypeParameterInstantiation>::None, #optional)
                });
            }
            Expression::StringLiteral(node) => {
                let value = node.value.as_str();
                tokens.append_all(quote! {
                    #ast_builder.expression_string_literal(#span, #value)
                });
            }
            Expression::Identifier(node) => {
                let name = node.name.as_str();
                tokens.append_all(quote! {
                    #ast_builder.expression_identifier_reference(#span, #name)
                });
            }
            Expression::StaticMemberExpression(node) => {
                let object = self.gen_expression(&node.object);
                let property = node.property.name.as_str();
                let optional = node.optional;
                tokens.append_all(quote! {
                    #ast_builder.expression_member(
                        #ast_builder.member_expression_static(#span, #object, #property, #optional)
                    )
                });
            }
            _ => unimplemented!(),
        }
        tokens
    }

    fn gen_arguments<'ast>(&self, node: &Vec<'ast, Argument<'ast>>) -> TokenStream {
        let mut arguments = TokenStream::new();
        let ast_builder = &self.ast_builder;
        for arg in node {
            let arg = self.gen_argument(arg);
            arguments.append_all(quote! {
                __arguments.push(#arg);
            });
        }
        quote! {
            {
                let mut __arguments = #ast_builder.vec();
                #arguments
                __arguments
            }
        }
    }

    fn gen_argument<'ast>(&self, node: &Argument<'ast>) -> TokenStream {
        let mut tokens = TokenStream::new();
        let ast_builder = &self.ast_builder;
        match node {
            Argument::SpreadElement(_node) => unimplemented!(),
            _ => {
                let expr = self.gen_expression(node.to_expression());
                tokens.append_all(quote! {
                    #ast_builder.argument_expression(#expr),
                });
            }
        }
        tokens
    }
}
