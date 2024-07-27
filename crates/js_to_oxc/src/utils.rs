use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

pub fn unimplemented() -> TokenStream {
  let t = format_ident!("unimplemented");
  quote! {
    #t!()
  }
}
