use crate::utils::unimplemented;
use crate::JsToOxc;
use oxc::ast::ast::*;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub fn gen_expression<'ast>(&self, node: &Expression<'ast>) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      Expression::BooleanLiteral(node) => {
        let value = node.value;
        quote! {
            #ast_builder.expression_boolean_literal(#span, #value)
        }
      }
      Expression::NullLiteral(_node) => {
        quote! {
            #ast_builder.expression_null_literal(#span)
        }
      }
      Expression::NumericLiteral(node) => {
        let value = node.value;
        let raw = node.raw;
        let base = self.gen_number_base(&node.base);
        quote! {
            #ast_builder.expression_numeric_literal(#span, #value, #raw, #base)
        }
      }
      Expression::BigIntLiteral(node) => {
        let raw = node.raw.as_str();
        let base = self.gen_big_int_base(&node.base);
        quote! {
            #ast_builder.expression_big_int_literal(#span, #raw, #base)
        }
      }
      Expression::RegExpLiteral(node) => {
        let regex = self.gen_reg_exp(&node.regex);
        quote! {
            #ast_builder.expression_reg_exp_literal(#span, EmptyObject {}, #regex)
        }
      }
      Expression::StringLiteral(node) => {
        let value = node.value.as_str();
        quote! {
            #ast_builder.expression_string_literal(#span, #value)
        }
      }
      Expression::TemplateLiteral(node) => {
        let quasis = self.gen_vec(&node.quasis, |quasi| self.gen_template_element(quasi));
        let expressions = self.gen_vec(&node.expressions, |expr| self.gen_expression(expr));
        quote! {
            #ast_builder.expression_template_literal(#span, #quasis, #expressions)
        }
      }

      Expression::Identifier(node) => {
        let name: &str = node.name.as_str();
        quote! {
            #ast_builder.expression_identifier_reference(#span, #name)
        }
      }

      Expression::MetaProperty(_) => unimplemented(),
      Expression::Super(_) => unimplemented(),

      Expression::ArrayExpression(node) => {
        let elements =
          self.gen_vec(&node.elements, |element| self.gen_array_expression_element(element));
        let trailing_comma = self.gen_option(&node.trailing_comma, |_| quote! {span});
        quote! {
            #ast_builder.expression_array(#span, #elements, #trailing_comma)
        }
      }
      Expression::ArrowFunctionExpression(_) => unimplemented(),
      Expression::AssignmentExpression(node) => {
        let ast_builder = &self.ast_builder;
        let span = &self.span;
        let left = self.gen_assignment_target(&node.left);
        let right = self.gen_expression(&node.right);
        let operator = self.gen_assignment_operator(&node.operator);
        quote! {
          #ast_builder.expression_assignment(#span, #operator, #left, #right)
        }
      }
      Expression::AwaitExpression(_) => unimplemented(),
      Expression::BinaryExpression(node) => {
        let ast_builder = &self.ast_builder;
        let span = &self.span;
        let left = self.gen_expression(&node.left);
        let right = self.gen_expression(&node.right);
        let operator = self.gen_binary_operator(&node.operator);
        quote! {
          #ast_builder.expression_binary(#span, #left, #operator, #right)
        }
      }
      Expression::CallExpression(node) => {
        let arguments = self.gen_arguments(&node.arguments);
        let callee = self.gen_expression(&node.callee);
        let optional = node.optional;
        quote! {
            #ast_builder.expression_call(#span, #arguments, #callee, Option::<TSTypeParameterInstantiation>::None, #optional)
        }
      }
      Expression::ChainExpression(_) => unimplemented(),
      Expression::ClassExpression(_) => unimplemented(),
      Expression::ConditionalExpression(_) => unimplemented(),
      Expression::FunctionExpression(_) => unimplemented(),
      Expression::ImportExpression(_) => unimplemented(),
      Expression::LogicalExpression(node) => {
        let ast_builder = &self.ast_builder;
        let span = &self.span;
        let left = self.gen_expression(&node.left);
        let right = self.gen_expression(&node.right);
        let operator = self.gen_logical_operator(&node.operator);
        quote! {
          #ast_builder.expression_logical(#span, #left, #operator, #right)
        }
      }
      Expression::NewExpression(_) => unimplemented(),
      Expression::ObjectExpression(_) => unimplemented(),
      Expression::ParenthesizedExpression(_) => unimplemented(),
      Expression::SequenceExpression(_) => unimplemented(),
      Expression::TaggedTemplateExpression(_) => unimplemented(),
      Expression::ThisExpression(_) => unimplemented(),
      Expression::UnaryExpression(node) => {
        let ast_builder = &self.ast_builder;
        let span = &self.span;
        let operator = self.gen_unary_operator(&node.operator);
        let argument = self.gen_expression(&node.argument);
        quote! {
          #ast_builder.expression_unary(#span, #operator, #argument)
        }
      }
      Expression::UpdateExpression(_) => unimplemented(),
      Expression::YieldExpression(_) => unimplemented(),
      Expression::PrivateInExpression(_) => unimplemented(),

      Expression::JSXElement(_) => unimplemented(),
      Expression::JSXFragment(_) => unimplemented(),

      Expression::TSAsExpression(_) => unimplemented(),
      Expression::TSSatisfiesExpression(_) => unimplemented(),
      Expression::TSTypeAssertion(_) => unimplemented(),
      Expression::TSNonNullExpression(_) => unimplemented(),
      Expression::TSInstantiationExpression(_) => unimplemented(),

      _ => {
        let member_expr = self.gen_member_expression(node.to_member_expression());
        quote! {
            #ast_builder.expression_member(#member_expr)
        }
      }
    }
  }
}