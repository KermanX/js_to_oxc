use crate::JsToOxc;
use oxc::allocator::Vec;
use proc_macro2::{Literal, TokenStream};
use quote::{quote, TokenStreamExt};

impl JsToOxc {
  pub(crate) fn gen_vec<T, M>(&self, items: &Vec<T>, map: M) -> TokenStream
  where
    M: Fn(&T) -> TokenStream,
  {
    let ast_builder = &self.ast_builder;
    let len = items.len();
    if len == 0 {
      return quote! {
          #ast_builder.vec()
      };
    }
    if len == 1 {
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
    let len = Literal::usize_unsuffixed(len);
    quote! {
        {
            let mut items = #ast_builder.vec_with_capacity(#len);
            #tokens
            items
        }
    }
  }
}
