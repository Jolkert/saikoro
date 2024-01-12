//! Types which represent operators

mod binary_op;
pub(crate) mod function;
mod unary_op;

pub use binary_op::*;
pub use unary_op::*;

use std::{fmt::Display, str::FromStr};
use thiserror::Error;

/// An enum representing a token which corresponds to an operator
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpToken
{
	Plus,
	Minus,
	Multiply,
	Divide,
	Modulus,
	Power,
	Dice,
	Equals,
	NotEquals,
	GreaterThan,
	LessThan,
	GreaterOrEqual,
	LessOrEqual,
}
impl FromStr for OpToken
{
	type Err = ParseOperatorError;

	fn from_str(str: &str) -> Result<Self, Self::Err>
	{
		match str
		{
			"+" => Ok(Self::Plus),
			"-" => Ok(Self::Minus),
			"*" => Ok(Self::Multiply),
			"/" => Ok(Self::Divide),
			"%" => Ok(Self::Modulus),
			"^" => Ok(Self::Power),
			"d" | "D" => Ok(Self::Dice),
			"==" => Ok(Self::Equals),
			"!=" => Ok(Self::NotEquals),
			">" => Ok(Self::GreaterThan),
			"<" => Ok(Self::LessThan),
			">=" => Ok(Self::GreaterOrEqual),
			"<=" => Ok(Self::LessOrEqual),
			other => Err(ParseOperatorError::from(other)),
		}
	}
}
impl Display for OpToken
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(
			f,
			"{}",
			match self
			{
				Self::Plus => "+",
				Self::Minus => "-",
				Self::Multiply => "*",
				Self::Divide => "/",
				Self::Modulus => "%",
				Self::Power => "^",
				Self::Dice => "d",
				Self::Equals => "==",
				Self::NotEquals => "!=",
				Self::GreaterThan => ">",
				Self::LessThan => "<",
				Self::GreaterOrEqual => ">=",
				Self::LessOrEqual => "<=",
			}
		)
	}
}

/// An error to be returned when failing to parse an operator
#[derive(Debug, Error)]
#[error("Unrecognized operator: {}", .operator)]
pub struct ParseOperatorError
{
	operator: Box<str>,
}

impl From<&str> for ParseOperatorError
{
	fn from(value: &str) -> Self
	{
		Self {
			operator: value.to_owned().into_boxed_str(),
		}
	}
}

/// An enum representing either a [`UnaryOperator`] or a [`BinaryOperator`]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator
{
	Unary(UnaryOperator),
	Binary(BinaryOperator),
}
impl From<UnaryOperator> for Operator
{
	fn from(value: UnaryOperator) -> Self
	{
		Self::Unary(value)
	}
}
impl From<BinaryOperator> for Operator
{
	fn from(value: BinaryOperator) -> Self
	{
		Self::Binary(value)
	}
}
