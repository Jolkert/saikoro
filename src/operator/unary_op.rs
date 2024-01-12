use super::{function, OpToken};
use crate::{error::InvalidOperatorError, evaluation::Operand, RangeRng};
use std::fmt::Display;

/// Represents an operator which takes only one [`Operand`] as an argument
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UnaryOperator
{
	pub token: UnaryOpToken,
	pub binding_power: u8,
	pub direction: UnaryDirection,
}
impl UnaryOperator
{
	fn eval_fn<R: RangeRng>(self) -> impl Fn(Operand, &mut R) -> Operand
	{
		match self.token
		{
			UnaryOpToken::Plus => function::unary_plus,
			UnaryOpToken::Minus => function::unary_minus,
			UnaryOpToken::Dice => function::unary_dice,
		}
	}

	/// Evaluates the given [`Operand`] over the operator's evaluation function using the provided
	/// [`RangeRng`] where applicable
	/// # Examples
	/// ```rust
	/// # use saikoro::{evaluation::Operand, operator::{UnaryOperator, UnaryOpToken}};
	/// let unary_minus = UnaryOperator::from(UnaryOpToken::Minus);
	/// // Passing a RangeRng is required even when it isn't used
	/// let result = unary_minus.eval(Operand::Number(5.0), &mut rand::thread_rng());
	/// assert_eq!(result, Operand::Number(-5.0));
	/// ```
	pub fn eval<R: RangeRng>(&self, operand: Operand, rng: &mut R) -> Operand
	{
		self.eval_fn()(operand, rng)
	}
}
impl TryFrom<OpToken> for UnaryOperator
{
	type Error = InvalidOperatorError;

	fn try_from(token: OpToken) -> Result<Self, Self::Error>
	{
		Ok(Self::from(UnaryOpToken::try_from(token)?))
	}
}
impl From<UnaryOpToken> for UnaryOperator
{
	fn from(value: UnaryOpToken) -> Self
	{
		use UnaryOpToken as Op;
		let (binding_power, direction) = match value
		{
			Op::Plus | Op::Minus => (7, UnaryDirection::Prefix),
			Op::Dice => (13, UnaryDirection::Prefix),
		};
		Self {
			token: value,
			binding_power,
			direction,
		}
	}
}

/// An enum representing a token which corresponds to a [`UnaryOperator`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOpToken
{
	Plus,
	Minus,
	Dice,
}
impl TryFrom<OpToken> for UnaryOpToken
{
	type Error = InvalidOperatorError;
	fn try_from(value: OpToken) -> Result<Self, Self::Error>
	{
		match value
		{
			OpToken::Plus => Ok(Self::Plus),
			OpToken::Minus => Ok(Self::Minus),
			OpToken::Dice => Ok(Self::Dice),
			_ => Err(InvalidOperatorError(value)),
		}
	}
}

/// An enum representing whether a [`UnaryOperator`] is a prefix or postfix operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryDirection
{
	Prefix,
	Postfix,
}
impl Display for UnaryDirection
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(
			f,
			"{}",
			match self
			{
				Self::Prefix => "Prefix",
				Self::Postfix => "Postfix",
			}
		)
	}
}
