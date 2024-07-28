use crate::utils::unimplemented;
use crate::JsToOxc;
use oxc::ast::ast::{
  ExportDefaultDeclarationKind, ExportSpecifier, ImportAttribute, ImportAttributeKey,
  ImportDeclarationSpecifier, ImportOrExportKind, ModuleDeclaration, ModuleExportName, WithClause,
};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_module_declaration(&self, node: &ModuleDeclaration) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      ModuleDeclaration::ImportDeclaration(node) => {
        let spefifiers = self.gen_option(&node.specifiers, |specifiers| {
          self.gen_vec(specifiers, |specifier| self.gen_import_declaration_specifier(specifier))
        });
        let source = self.gen_string_literal(&node.source);
        let with_clause =
          self.gen_option(&node.with_clause, |with_clause| self.gen_with_clause(with_clause));
        let import_kind = self.gen_import_or_export_kind(&node.import_kind);
        quote! {
          #ast_builder.module_declaration_import_declaration(
            #span,
            #spefifiers,
            #source,
            #with_clause,
            #import_kind
          )
        }
      }
      ModuleDeclaration::ExportAllDeclaration(node) => {
        let exported =
          self.gen_option(&node.exported, |exported| self.gen_module_export_name(exported));
        let source = self.gen_string_literal(&node.source);
        let with_clause =
          self.gen_option(&node.with_clause, |with_clause| self.gen_with_clause(with_clause));
        let export_kind = self.gen_import_or_export_kind(&node.export_kind);
        quote! {
          #ast_builder.module_declaration_export_all_declaration(
            #span,
            #exported,
            #source,
            #with_clause,
            #export_kind
          )
        }
      }
      ModuleDeclaration::ExportDefaultDeclaration(node) => {
        let declaration = self.gen_export_default_declaration_kind(&node.declaration);
        let exported = self.gen_module_export_name(&node.exported);
        quote! {
          #ast_builder.module_declaration_export_default_declaration(
            #span,
            #declaration,
            #exported
          )
        }
      }
      ModuleDeclaration::ExportNamedDeclaration(node) => {
        let declaration =
          self.gen_option(&node.declaration, |declaration| self.gen_declaration(declaration));
        let specifiers =
          self.gen_vec(&node.specifiers, |specifier| self.gen_export_specifier(specifier));
        let source = self.gen_option(&node.source, |source| self.gen_string_literal(source));
        let export_kind = self.gen_import_or_export_kind(&node.export_kind);
        let with_clause =
          self.gen_option(&node.with_clause, |with_clause| self.gen_with_clause(with_clause));
        quote! {
          #ast_builder.module_declaration_export_named_declaration(
            #span,
            #declaration,
            #specifiers,
            #source,
            #export_kind,
            #with_clause,
          )
        }
      }
      _ => unimplemented(),
    }
  }

  fn gen_import_declaration_specifier(&self, node: &ImportDeclarationSpecifier) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      ImportDeclarationSpecifier::ImportSpecifier(node) => {
        let imported = self.gen_module_export_name(&node.imported);
        let local = self.gen_binding_identifier(&node.local);
        let import_kind = self.gen_import_or_export_kind(&node.import_kind);
        quote! {
          #ast_builder.import_declaration_specifier_import_specifier(
            #span,
            #imported,
            #local,
            #import_kind,
          )
        }
      }
      ImportDeclarationSpecifier::ImportDefaultSpecifier(node) => {
        let local = self.gen_binding_identifier(&node.local);
        quote! {
          #ast_builder.import_declaration_specifier_import_default_specifier(
            #span,
            #local
          )
        }
      }
      ImportDeclarationSpecifier::ImportNamespaceSpecifier(node) => {
        let local = self.gen_binding_identifier(&node.local);
        quote! {
          #ast_builder.import_declaration_specifier_import_namespace_specifier(
            #span,
            #local
          )
        }
      }
    }
  }

  fn gen_module_export_name(&self, node: &ModuleExportName) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      ModuleExportName::IdentifierName(node) => {
        let name = node.name.as_str();
        quote! {
          #ast_builder.module_export_name_identifier(
            #span,
            #name
          )
        }
      }
      ModuleExportName::IdentifierReference(node) => {
        let name = node.name.as_str();
        quote! {
          #ast_builder.module_export_name_reference(
            #span,
            #name
          )
        }
      }
      ModuleExportName::StringLiteral(node) => {
        let value = node.value.as_str();
        quote! {
          #ast_builder.module_export_name_string_literal(
            #span,
            #value
          )
        }
      }
    }
  }

  fn gen_import_or_export_kind(&self, kind: &ImportOrExportKind) -> TokenStream {
    match kind {
      ImportOrExportKind::Value => quote! { ImportOrExportKind::Value },
      ImportOrExportKind::Type => quote! { ImportOrExportKind::Type },
    }
  }

  fn gen_with_clause(&self, node: &WithClause) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let attributes_keyword = self.gen_identifier_name(&node.attributes_keyword);
    let with_entries = self
      .gen_vec(&node.with_entries, |import_attribute| self.gen_import_attribute(import_attribute));
    quote! {
      #ast_builder.with_clause(
        #span,
        #attributes_keyword,
        #with_entries
      )
    }
  }

  fn gen_import_attribute(&self, node: &ImportAttribute) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    let key = self.gen_import_attribute_key(&node.key);
    let value = self.gen_string_literal(&node.value);
    quote! {
      #ast_builder.import_attribute(
        #span,
        #key,
        #value
      )
    }
  }

  fn gen_import_attribute_key(&self, node: &ImportAttributeKey) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      ImportAttributeKey::Identifier(node) => {
        let name = node.name.as_str();
        quote! {
          #ast_builder.import_attribute_key_identifier_name(
            #span,
            #name
          )
        }
      }
      ImportAttributeKey::StringLiteral(node) => {
        let value = node.value.as_str();
        quote! {
          #ast_builder.import_attribute_key_string_literal(
            #span,
            #value
          )
        }
      }
    }
  }

  fn gen_export_default_declaration_kind(
    &self,
    node: &ExportDefaultDeclarationKind,
  ) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match node {
      ExportDefaultDeclarationKind::FunctionDeclaration(node) => {
        let r#type = self.gen_function_type(&node.r#type);
        let id = self.gen_option(&node.id, |id| self.gen_binding_identifier(id));
        let generator = node.generator;
        let r#async = node.r#async;
        let declare = node.declare;
        let type_parameters = quote! { Option::<TSTypeParameterDeclaration>::None };
        let this_param = quote! { Option::<TSThisParameter>::None };
        let params = self.gen_formal_parameters(&node.params);
        let return_type = quote! { Option::<TSTypeAnnotation>::None };
        let body = self.gen_option(&node.body, |body| self.gen_function_body(body));
        quote! {
          #ast_builder.export_default_declaration_kind_function(
            #r#type,
            #span,
            #id,
            #generator,
            #r#async,
            #declare,
            #type_parameters,
            #this_param,
            #params,
            #return_type,
            #body
          )
        }
      }
      ExportDefaultDeclarationKind::ClassDeclaration(node) => {
        let r#type = self.gen_class_type(&node.r#type);
        let decorators = quote! { #ast_builder.vec() };
        let id = self.gen_option(&node.id, |id| self.gen_binding_identifier(id));
        let type_parameters = quote! { Option::<TSTypeParameterDeclaration>::None };
        let super_class =
          self.gen_option(&node.super_class, |super_class| self.gen_expression(super_class));
        let super_type_parameters = quote! { Option::<TSTypeParameterInstantiation>::None };
        let implements = quote! { None };
        let body = self.gen_class_body(&node.body);
        let r#abstract = node.r#abstract;
        let declare = node.declare;
        quote! {
          #ast_builder.export_default_declaration_kind_class(
            #r#type,
            #span,
            #decorators,
            #id,
            #type_parameters,
            #super_class,
            #super_type_parameters,
            #implements,
            #body,
            #r#abstract,
            #declare
          )
        }
      }
      ExportDefaultDeclarationKind::TSInterfaceDeclaration(_) => unimplemented(),
      _ => {
        let node = node.to_expression();
        let inner = self.gen_expression(node);
        quote! {
          #ast_builder.export_default_declaration_kind_expression(
            #span,
            #inner
          )
        }
      }
    }
  }
  fn gen_export_specifier(&self, node: &ExportSpecifier) -> TokenStream {
    let ast_builder: &TokenStream = &self.ast_builder;
    let span = &self.span;
    let local = self.gen_module_export_name(&node.local);
    let exported = self.gen_module_export_name(&node.exported);
    let export_kind = self.gen_import_or_export_kind(&node.export_kind);
    quote! {
      #ast_builder.export_specifier(
        #span,
        #local,
        #exported,
        #export_kind
      )
    }
  }
}
