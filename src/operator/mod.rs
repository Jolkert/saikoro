//! Types which represent operators

mod binary_op;
mod comp_op;
pub(crate) mod function;
mod unary_op;

pub use binary_op::*;
pub use comp_op::*;
pub use unary_op::*;

use std::{fmt::Display, str::FromStr};
use thiserror::Error;

/// An enum representing a token which corresponds to an operator
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpToken
{
	/// Represents the string token `+`
	Plus,
	/// Represents the string token `-`
	Minus,
	/// Represents the string token `*`
	Multiply,
	/// Represents the string token `/`
	Divide,
	/// Represents the string token `%`
	Modulus,
	/// Represents the string token `^`
	Power,
	/// Represents the string token `d` (case insensitive)
	Dice,
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
			unrecognized => Err(ParseOperatorError::from(unrecognized)),
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
	/// An operator which only takes one operand
	Unary(UnaryOperator),
	/// An operator which takes two operands
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
