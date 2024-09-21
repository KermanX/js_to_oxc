use crate::JsToOxc;
use oxc::ast::ast::{RegExp, RegExpFlags, RegExpPattern};
use proc_macro2::TokenStream;
use quote::quote;

impl JsToOxc {
  pub(crate) fn gen_reg_exp(&self, regex: &RegExp) -> TokenStream {
    let pattern = self.gen_reg_exp_pattern(&regex.pattern);
    let flags = self.gen_reg_exp_flags(&regex.flags);
    quote! {
      RegExp {
        pattern: #pattern,
        flags: #flags,
      }
    }
  }

  fn gen_reg_exp_pattern(&self, pattern: &RegExpPattern) -> TokenStream {
    match pattern {
      RegExpPattern::Raw(value) => {
        quote! {
          RegExpPattern::Raw(#value)
        }
      }
      RegExpPattern::Invalid(value) => {
        quote! {
          RegExpPattern::Invalid(#value)
        }
      }
      RegExpPattern::Pattern(_) => unreachable!(),
    }
  }

  fn gen_reg_exp_flags(&self, flags: &RegExpFlags) -> TokenStream {
    let flags = flags.bits();
    quote! {
      RegExpFlags::from_bits_truncate(#flags)
    }
  }
}
