use crate::JsToOxc;
use oxc::ast::ast::{TemplateElement, TemplateElementValue};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_template_element(&self, element: &TemplateElement) -> TokenStream {
    let span = &self.span;
    let tail = element.tail;
    let value = self.gen_template_element_value(&element.value);
    quote! {
      TemplateElement {
        span: #span,
        tail: #tail,
        value: #value,
      }
    }
  }

  fn gen_template_element_value(&self, value: &TemplateElementValue) -> TokenStream {
    let raw = value.raw.as_str();
    let cooked = self.gen_option(&value.cooked, |cooked| {
      let cooked = cooked.as_str();
      quote! {#cooked.into()}
    });
    quote! {
      TemplateElementValue {
        raw: #raw.into(),
        cooked: #cooked,
      }
    }
  }
}
