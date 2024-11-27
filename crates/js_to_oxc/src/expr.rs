use crate::JsToOxc;
use oxc::{ast::ast::*, syntax::operator::UnaryOperator};
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
        let raw = node.raw;
        quote! {
            #ast_builder.expression_reg_exp_literal(#span, #regex, #raw)
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
        if let Some(hole) = self.gen_hole(name) {
          return hole;
        }
        quote! {
            #ast_builder.expression_identifier_reference(#span, #name)
        }
      }

      Expression::MetaProperty(node) => {
        let meta = self.gen_identifier_name(&node.meta);
        let property = self.gen_identifier_name(&node.property);
        quote! {
            #ast_builder.expression_meta_property(#span, #meta, #property)
        }
      }
      Expression::Super(_node) => {
        quote! {
            #ast_builder.expression_super(#span)
        }
      }

      Expression::ArrayExpression(node) => {
        let elements =
          self.gen_vec(&node.elements, |element| self.gen_array_expression_element(element));
        let trailing_comma = self.gen_option(&node.trailing_comma, |_| quote! {span});
        quote! {
            #ast_builder.expression_array(#span, #elements, #trailing_comma)
        }
      }
      Expression::ArrowFunctionExpression(node) => {
        let expression = node.expression;
        let r#async = node.r#async;
        let type_parameters = quote! { NONE };
        let params = self.gen_formal_parameters(&node.params);
        let return_type = quote! { NONE };
        let body = self.gen_function_body(&node.body);
        quote! {
            #ast_builder.expression_arrow_function(#span, #expression, #r#async, #type_parameters, #params, #return_type, #body)
        }
      }
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
      Expression::AwaitExpression(node) => {
        let argument = self.gen_expression(&node.argument);
        quote! {
          #ast_builder.expression_await(#span, #argument)
        }
      }
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
        let arguments = self.gen_vec(&node.arguments, |argument| self.gen_argument(argument));
        let callee = self.gen_expression(&node.callee);
        let optional = node.optional;
        quote! {
          #ast_builder.expression_call(#span, #callee, NONE, #arguments, #optional)
        }
      }
      Expression::ChainExpression(node) => {
        let expression = self.gen_chain_element(&node.expression);
        quote! {
          #ast_builder.expression_chain(#span, #expression)
        }
      }
      Expression::ClassExpression(node) => {
        let r#type = self.gen_class_type(&node.r#type);
        let decorators = quote! { #ast_builder.vec() };
        let id = self.gen_option(&node.id, |id| self.gen_binding_identifier(id));
        let type_parameters = quote! { NONE };
        let super_class =
          self.gen_option(&node.super_class, |super_class| self.gen_expression(super_class));
        let super_type_parameters = quote! { NONE };
        let implements = quote! { None };
        let body = self.gen_class_body(&node.body);
        let r#abstract = node.r#abstract;
        let declare = node.declare;
        quote! {
          #ast_builder.expression_class(#r#type, #span, #decorators, #id, #type_parameters, #super_class, #super_type_parameters, #implements, #body, #r#abstract, #declare)
        }
      }
      Expression::ConditionalExpression(node) => {
        let ast_builder = &self.ast_builder;
        let span = &self.span;
        let test = self.gen_expression(&node.test);
        let consequent = self.gen_expression(&node.consequent);
        let alternate = self.gen_expression(&node.alternate);
        quote! {
          #ast_builder.expression_conditional(#span, #test, #consequent, #alternate)
        }
      }
      Expression::FunctionExpression(node) => {
        let r#type = self.gen_function_type(&node.r#type);
        let id = self.gen_option(&node.id, |id| self.gen_binding_identifier(id));
        let generator = node.generator;
        let r#async = node.r#async;
        let declare = node.declare;
        let type_parameters = quote! { NONE };
        let this_param = quote! { NONE };
        let params = self.gen_formal_parameters(&node.params);
        let return_type = quote! { NONE };
        let body = self.gen_option(&node.body, |body| self.gen_function_body(body));
        quote! {
          #ast_builder.expression_function(#r#type, #span, #id, #generator, #r#async, #declare, #type_parameters, #this_param, #params, #return_type, #body)
        }
      }
      Expression::ImportExpression(node) => {
        let source = self.gen_expression(&node.source);
        let arguments = self.gen_vec(&node.arguments, |expr| self.gen_expression(expr));
        quote! {
          #ast_builder.expression_import(#span, #source, #arguments)
        }
      }
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
      Expression::NewExpression(node) => {
        let callee = self.gen_expression(&node.callee);
        let arguments = self.gen_vec(&node.arguments, |argument| self.gen_argument(argument));
        quote! {
          #ast_builder.expression_new(#span, #callee, #arguments, NONE)
        }
      }
      Expression::ObjectExpression(node) => {
        let properties = self.gen_vec(&node.properties, |prop| self.gen_object_property(prop));
        let trailing_comma = self.gen_option(&node.trailing_comma, |_| quote! { #span });
        quote! {
          #ast_builder.expression_object(#span, #properties, #trailing_comma)
        }
      }
      Expression::ParenthesizedExpression(node) => {
        let expression = self.gen_expression(&node.expression);
        quote! {
          #ast_builder.expression_parenthesized(#span, #expression)
        }
      }
      Expression::SequenceExpression(node) => {
        let expressions = self.gen_vec(&node.expressions, |expr| self.gen_expression(expr));

        quote! {
          #ast_builder.expression_sequence(#span, #expressions)
        }
      }
      Expression::TaggedTemplateExpression(node) => {
        let tag = self.gen_expression(&node.tag);
        let quasi = self.gen_template_literal(&node.quasi);
        let type_parameters = quote! { NONE };
        quote! {
          #ast_builder.expression_tagged_template(#span, #tag, #quasi, #type_parameters)
        }
      }
      Expression::ThisExpression(_node) => {
        quote! {
          #ast_builder.expression_this(#span)
        }
      }
      Expression::UnaryExpression(node) => {
        let ast_builder = &self.ast_builder;
        let span = &self.span;
        // Handles `void 0`
        if matches!(&node.operator, UnaryOperator::Void) {
          if let Expression::NumericLiteral(node) = &node.argument {
            if node.value == 0.0 {
              return quote! {
                #ast_builder.void_0(#span)
              };
            }
          }
        }
        let operator = self.gen_unary_operator(&node.operator);
        let argument = self.gen_expression(&node.argument);
        quote! {
          #ast_builder.expression_unary(#span, #operator, #argument)
        }
      }
      Expression::UpdateExpression(node) => {
        let operator = self.gen_update_operator(&node.operator);
        let prefix = node.prefix;
        let argument = self.gen_simple_assignment_target(&node.argument);
        quote! {
          #ast_builder.expression_update(#span, #operator, #prefix, #argument)
        }
      }
      Expression::YieldExpression(node) => {
        let delegate = node.delegate;
        let argument = self.gen_option(&node.argument, |argument| self.gen_expression(argument));
        quote! {
          #ast_builder.expression_yield(#span, #delegate, #argument)
        }
      }
      Expression::PrivateInExpression(node) => {
        let left = self.gen_private_identifier(&node.left);
        let operator = self.gen_binary_operator(&node.operator);
        let right = self.gen_expression(&node.right);
        quote! {
          #ast_builder.expression_private_in(#span, #left, #operator, #right)
        }
      }

      Expression::JSXElement(_) => unimplemented!("jsx"),
      Expression::JSXFragment(_) => unimplemented!("jsx"),

      Expression::TSAsExpression(_) => unimplemented!("ts"),
      Expression::TSSatisfiesExpression(_) => unimplemented!("ts"),
      Expression::TSTypeAssertion(_) => unimplemented!("ts"),
      Expression::TSNonNullExpression(_) => unimplemented!("ts"),
      Expression::TSInstantiationExpression(_) => unimplemented!("ts"),

      _ => {
        let member_expr = self.gen_member_expression(node.to_member_expression());
        quote! {
            Expression::from(#member_expr)
        }
      }
    }
  }
}
