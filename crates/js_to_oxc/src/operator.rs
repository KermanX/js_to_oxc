use oxc::syntax::operator::{
  AssignmentOperator, BinaryOperator, LogicalOperator, UnaryOperator, UpdateOperator,
};
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

  pub(crate) fn gen_assignment_operator(&self, operator: &AssignmentOperator) -> TokenStream {
    match operator {
      AssignmentOperator::Assign => quote! { AssignmentOperator::Assign },
      AssignmentOperator::Addition => quote! { AssignmentOperator::Addition },
      AssignmentOperator::Subtraction => quote! { AssignmentOperator::Subtraction },
      AssignmentOperator::Multiplication => quote! { AssignmentOperator::Multiplication },
      AssignmentOperator::Division => quote! { AssignmentOperator::Division },
      AssignmentOperator::Remainder => quote! { AssignmentOperator::Remainder },
      AssignmentOperator::ShiftLeft => quote! { AssignmentOperator::ShiftLeft },
      AssignmentOperator::ShiftRight => quote! { AssignmentOperator::ShiftRight },
      AssignmentOperator::ShiftRightZeroFill => quote! { AssignmentOperator::ShiftRightZeroFill },
      AssignmentOperator::BitwiseOR => quote! { AssignmentOperator::BitwiseOR },
      AssignmentOperator::BitwiseXOR => quote! { AssignmentOperator::BitwiseXOR },
      AssignmentOperator::BitwiseAnd => quote! { AssignmentOperator::BitwiseAnd },
      AssignmentOperator::LogicalAnd => quote! { AssignmentOperator::LogicalAnd },
      AssignmentOperator::LogicalOr => quote! { AssignmentOperator::LogicalOr },
      AssignmentOperator::LogicalNullish => quote! { AssignmentOperator::LogicalNullish },
      AssignmentOperator::Exponential => quote! { AssignmentOperator::Exponential },
    }
  }

  pub(crate) fn gen_update_operator(&self, operator: &UpdateOperator) -> TokenStream {
    match operator {
      UpdateOperator::Increment => quote! { UpdateOperator::Increment },
      UpdateOperator::Decrement => quote! { UpdateOperator::Decrement },
    }
  }
}
