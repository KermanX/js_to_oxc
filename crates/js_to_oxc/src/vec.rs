use crate::JsToOxc;
use oxc::allocator::Vec;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

impl JsToOxc {
  pub(crate) fn gen_vec<T, M>(&self, items: &Vec<T>, map: M) -> TokenStream
  where
    M: Fn(&T) -> TokenStream,
  {
    let mut tokens = TokenStream::new();
    let ast_builder = &self.ast_builder;
    for arg in items {
      let arg = map(arg);
      tokens.append_all(quote! {
          __items.push(#arg);
      });
    }
    quote! {
        {
            let mut __items = #ast_builder.vec();
            #tokens
            __items
        }
    }
  }
}
