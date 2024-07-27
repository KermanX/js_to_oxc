use crate::JsToOxc;
use oxc::syntax::number::BigintBase;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_big_int_base(&self, base: &BigintBase) -> TokenStream {
    match base {
      BigintBase::Decimal => quote! { BigintBase::Decimal },
      BigintBase::Binary => quote! { BigintBase::Binary },
      BigintBase::Octal => quote! { BigintBase::Octal },
      BigintBase::Hex => quote! { BigintBase::Hex },
    }
  }
}
