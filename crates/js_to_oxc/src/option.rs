use crate::JsToOxc;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

impl JsToOxc {
  pub(crate) fn gen_option_with_type<T, M>(
    &self,
    option: &Option<T>,
    r#type: &str,
    map: M,
  ) -> TokenStream
  where
    M: Fn(&T) -> TokenStream,
  {
    let r#type = format_ident!("{}", r#type);
    match option {
      Some(value) => {
        let value = map(value);
        quote! {
          Some(#value)
        }
      }
      None => quote! {
        None::<#r#type>
      },
    }
  }

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
