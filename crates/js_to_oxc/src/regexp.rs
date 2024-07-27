use crate::JsToOxc;
use oxc::ast::ast::{RegExp, RegExpFlags};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_reg_exp(&self, regex: &RegExp) -> TokenStream {
    let pattern = regex.pattern.as_str();
    let flags = self.gen_reg_exp_flags(&regex.flags);
    quote! {
      RegExp {
        pattern: #pattern.into(),
        flags: #flags,
      }
    }
  }

  fn gen_reg_exp_flags(&self, flags: &RegExpFlags) -> TokenStream {
    let flags = flags.bits();
    quote! {
      RegExpFlags::from_bits_truncate(#flags)
    }
  }
}
