use crate::JsToOxc;
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_option<T, M>(&self, option: &Option<T>, map: M) -> TokenStream
  where
    M: Fn(&T) -> TokenStream,
  {
    match option {
      Some(value) => {
        let value = map(value);
        quote! {
          Some(#value)
        }
      }
      None => quote! {
        None
      },
    }
  }
}
