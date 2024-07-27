use crate::JsToOxc;
use oxc::syntax::number::NumberBase;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_number_base(&self, base: &NumberBase) -> TokenStream {
    match base {
      NumberBase::Float => quote! { NumberBase::Float },
      NumberBase::Decimal => quote! { NumberBase::Decimal },
      NumberBase::Binary => quote! { NumberBase::Binary },
      NumberBase::Octal => quote! { NumberBase::Octal },
      NumberBase::Hex => quote! { NumberBase::Hex },
    }
  }
}
