use oxc::syntax::operator::{BinaryOperator, LogicalOperator, UnaryOperator};
use proc_macro2::TokenStream;
use quote::quote;

use crate::JsToOxc;

impl JsToOxc {
  pub(crate) fn gen_binary_operator(&self, operator: &BinaryOperator) -> TokenStream {
    match operator {
      BinaryOperator::Equality => quote! { BinaryOperator::Equality },
      BinaryOperator::Inequality => quote! { BinaryOperator::Inequality },
      BinaryOperator::StrictEquality => quote! { BinaryOperator::StrictEquality },
      BinaryOperator::StrictInequality => quote! { BinaryOperator::StrictInequality },
      BinaryOperator::LessThan => quote! { BinaryOperator::LessThan },
      BinaryOperator::LessEqualThan => quote! { BinaryOperator::LessEqualThan },
      BinaryOperator::GreaterThan => quote! { BinaryOperator::GreaterThan },
      BinaryOperator::GreaterEqualThan => quote! { BinaryOperator::GreaterEqualThan },
      BinaryOperator::ShiftLeft => quote! { BinaryOperator::ShiftLeft },
      BinaryOperator::ShiftRight => quote! { BinaryOperator::ShiftRight },
      BinaryOperator::ShiftRightZeroFill => quote! { BinaryOperator::ShiftRightZeroFill },
      BinaryOperator::Addition => quote! { BinaryOperator::Addition },
      BinaryOperator::Subtraction => quote! { BinaryOperator::Subtraction },
      BinaryOperator::Multiplication => quote! { BinaryOperator::Multiplication },
      BinaryOperator::Division => quote! { BinaryOperator::Division },
      BinaryOperator::Remainder => quote! { BinaryOperator::Remainder },
      BinaryOperator::BitwiseOR => quote! { BinaryOperator::BitwiseOR },
      BinaryOperator::BitwiseXOR => quote! { BinaryOperator::BitwiseXOR },
      BinaryOperator::BitwiseAnd => quote! { BinaryOperator::BitwiseAnd },
      BinaryOperator::In => quote! { BinaryOperator::In },
      BinaryOperator::Instanceof => quote! { BinaryOperator::Instanceof },
      BinaryOperator::Exponential => quote! { BinaryOperator::Exponential },
    }
  }

  pub(crate) fn gen_logical_operator(&self, operator: &LogicalOperator) -> TokenStream {
    match operator {
      LogicalOperator::Or => quote! { LogicalOperator::Or },
      LogicalOperator::And => quote! { LogicalOperator::And },
      LogicalOperator::Coalesce => quote! { LogicalOperator::Coalesce },
    }
  }

  pub(crate) fn gen_unary_operator(&self, operator: &UnaryOperator) -> TokenStream {
    match operator {
      UnaryOperator::UnaryNegation => quote! { UnaryOperator::UnaryNegation },
      UnaryOperator::UnaryPlus => quote! { UnaryOperator::UnaryPlus },
      UnaryOperator::LogicalNot => quote! { UnaryOperator::LogicalNot },
      UnaryOperator::BitwiseNot => quote! { UnaryOperator::BitwiseNot },
      UnaryOperator::Typeof => quote! { UnaryOperator::Typeof },
      UnaryOperator::Void => quote! { UnaryOperator::Void },
      UnaryOperator::Delete => quote! { UnaryOperator::Delete },
    }
  }
}
