use crate::JsToOxc;
use oxc::allocator::Vec;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

impl JsToOxc {
  pub(crate) fn gen_vec<T, M>(&self, items: &Vec<T>, map: M) -> TokenStream
  where
    M: Fn(&T) -> TokenStream,
  {
    let ast_builder = &self.ast_builder;
    if items.len() == 0 {
      return quote! {
          #ast_builder.vec()
      };
    }
    if items.len() == 1 {
      let item = map(&items[0]);
      return quote! {
          #ast_builder.vec1(#item)
      };
    }
    let mut tokens = TokenStream::new();
    for arg in items {
      let arg = map(arg);
      tokens.append_all(quote! {
          items.push(#arg);
      });
    }
    quote! {
        {
            let mut items = #ast_builder.vec();
            #tokens
            items
        }
    }
  }
}
