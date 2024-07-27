use crate::JsToOxc;
use oxc::ast::ast::{ObjectPropertyKind, PropertyKey, PropertyKind};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_object_property(&self, property: &ObjectPropertyKind) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match property {
      ObjectPropertyKind::ObjectProperty(property) => {
        let kind = self.gen_property_kind(&property.kind);
        let key = self.gen_property_key(&property.key);
        let value = self.gen_expression(&property.value);
        let init = self.gen_option(&property.init, |init| self.gen_expression(init));
        let method = property.method;
        let shorthand = property.shorthand;
        let computed = property.computed;
        quote! {
          #ast_builder.object_property_kind_object_property(
            #span,
            #kind,
            #key,
            #value,
            #init,
            #method,
            #shorthand,
            #computed,
          )
        }
      }
      ObjectPropertyKind::SpreadProperty(property) => {
        let argument = self.gen_expression(&property.argument);
        quote! {
          #ast_builder.object_property_kind_spread_element(#span, #argument)
        }
      }
    }
  }

  fn gen_property_kind(&self, kind: &PropertyKind) -> TokenStream {
    match kind {
      PropertyKind::Init => quote! { PropertyKind::Init },
      PropertyKind::Get => quote! { PropertyKind::Get },
      PropertyKind::Set => quote! { PropertyKind::Set },
    }
  }

  pub(crate) fn gen_property_key(&self, key: &PropertyKey) -> TokenStream {
    let ast_builder = &self.ast_builder;
    let span = &self.span;
    match key {
      PropertyKey::StaticIdentifier(identifier) => {
        let name = identifier.name.as_str();
        quote! {
          #ast_builder.property_key_identifier_name(#span, #name)
        }
      }
      PropertyKey::PrivateIdentifier(identifier) => {
        let name = identifier.name.as_str();
        quote! {
          #ast_builder.property_key_private_identifier(#span, #name)
        }
      }
      _ => {
        let expression = self.gen_expression(key.to_expression());
        quote! {
          #ast_builder.property_key_expression(#expression)
        }
      }
    }
  }
}
