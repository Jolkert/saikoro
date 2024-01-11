use super::{function, OpToken};
use crate::{errors::InvalidOperatorError, evaluation::Operand, RangeRng};
use std::fmt::Display;

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

	#[allow(clippy::needless_pass_by_value)]
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
		use UnaryOpToken as Op;
		let token = UnaryOpToken::try_from(token)?;
		let (binding_power, direction) = match token
		{
			Op::Plus | Op::Minus => (7, UnaryDirection::Prefix),
			Op::Dice => (13, UnaryDirection::Prefix),
		};
		Ok(Self {
			token,
			binding_power,
			direction,
		})
	}
}

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
