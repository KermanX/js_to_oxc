use crate::JsToOxc;
use oxc::ast::ast::{
  Directive, FormalParameter, FormalParameterKind, FormalParameters, Function, FunctionBody,
  FunctionType,
};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_function(&self, function: &Function) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let r#type = self.gen_function_type(&function.r#type);
    let id = self.gen_option(&function.id, |id| self.gen_binding_identifier(id));
    let generator = function.generator;
    let r#async = function.r#async;
    let declare = function.declare;
    let type_parameters = quote! { NONE };
    let this_param = quote! { NONE };
    let params = self.gen_formal_parameters(&function.params);
    let return_type = quote! { NONE };
    let body = self.gen_option(&function.body, |body| self.gen_function_body(body));
    quote! {
      #ast_builder.function(#r#type, #span, #id, #generator, #r#async, #declare, #type_parameters, #this_param, #params, #return_type, #body)
    }
  }

  pub(crate) fn gen_function_type(&self, r#type: &FunctionType) -> TokenStream {
    match r#type {
      FunctionType::FunctionDeclaration => quote! { FunctionType::FunctionDeclaration },
      FunctionType::FunctionExpression => quote! { FunctionType::FunctionExpression },
      FunctionType::TSDeclareFunction => quote! { FunctionType::TSDeclareFunction },
      FunctionType::TSEmptyBodyFunctionExpression => {
        quote! { FunctionType::TSEmptyBodyFunctionExpression }
      }
    }
  }

  pub(crate) fn gen_formal_parameters(&self, params: &FormalParameters) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let kind = self.gen_formal_parameter_kind(&params.kind);
    let items = self.gen_vec(&params.items, |item| self.gen_formal_parameter(item));
    let rest = self.gen_option_with_type(&params.rest, "BindingRestElement", |rest| {
      self.gen_binding_rest_element(rest)
    });
    quote! {
      #ast_builder.formal_parameters(#span, #kind, #items, #rest)
    }
  }

  fn gen_formal_parameter_kind(&self, kind: &FormalParameterKind) -> TokenStream {
    match kind {
      FormalParameterKind::FormalParameter => quote! { FormalParameterKind::FormalParameter },
      FormalParameterKind::UniqueFormalParameters => {
        quote! { FormalParameterKind::UniqueFormalParameters }
      }
      FormalParameterKind::ArrowFormalParameters => {
        quote! { FormalParameterKind::ArrowFormalParameters }
      }
      FormalParameterKind::Signature => quote! { FormalParameterKind::Signature },
    }
  }

  fn gen_formal_parameter(&self, param: &FormalParameter) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let decorators = quote! { #ast_builder.vec() };
    let pattern = self.gen_binding_pattern(&param.pattern);
    let accessibility = quote! { None };
    let readonly = param.readonly;
    let r#override = param.r#override;
    quote! {
      #ast_builder.formal_parameter(#span, #decorators, #pattern, #accessibility, #readonly, #r#override)
    }
  }

  pub(crate) fn gen_function_body(&self, body: &FunctionBody) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let directives = self.gen_vec(&body.directives, |directive| self.gen_directive(directive));
    let statements = self.gen_vec(&body.statements, |statement| self.gen_statement(statement));
    quote! {
      #ast_builder.function_body(#span, #directives, #statements)
    }
  }

  pub(crate) fn gen_directive(&self, directive: &Directive) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let expression = self.gen_string_literal(&directive.expression);
    let directive = directive.directive.as_str();
    quote! {
      #ast_builder.directive(#span, #expression, #directive)
    }
  }
}
