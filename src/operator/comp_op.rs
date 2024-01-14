use crate::{evaluation::Operand, RangeRng};

use super::{function, ParseOperatorError};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CompOperator
{
	Equals,
	NotEquals,
	GreaterThan,
	LessThan,
	GreaterOrEqual,
	LessOrEqual,
}
impl CompOperator
{
	fn eval_fn<R: RangeRng>(self) -> impl Fn(Operand, Operand, Operand, &mut R) -> Operand
	{
		match self
		{
			Self::Equals => function::eq_roll_comp,
			Self::NotEquals => function::ne_roll_comp,
			Self::GreaterThan => function::greater_roll_comp,
			Self::LessThan => function::less_roll_comp,
			Self::GreaterOrEqual => function::greater_eq_roll_comp,
			Self::LessOrEqual => function::less_eq_roll_comp,
		}
	}

	pub fn eval<R: RangeRng>(
		self,
		dice_lhs: Operand,
		dice_rhs: Operand,
		compare_to: Operand,
		rand: &mut R,
	) -> Operand
	{
		self.eval_fn()(dice_lhs, dice_rhs, compare_to, rand)
	}
}
impl FromStr for CompOperator
{
	type Err = ParseOperatorError;

	fn from_str(str: &str) -> Result<Self, Self::Err>
	{
		match str
		{
			"==" => Ok(Self::Equals),
			"!=" => Ok(Self::NotEquals),
			">" => Ok(Self::GreaterThan),
			"<" => Ok(Self::LessThan),
			">=" => Ok(Self::GreaterOrEqual),
			"<=" => Ok(Self::LessOrEqual),
			unrecognized => Err(ParseOperatorError::from(unrecognized)),
		}
	}
}
impl Display for CompOperator
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(
			f,
			"{}",
			match self
			{
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
