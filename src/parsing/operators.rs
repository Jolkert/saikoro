use crate::{
	evaluation::{
		functions::{self, FilterNumberError},
		Operand,
	},
	RangeRng,
};
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UnaryOperator
{
	pub token: UnaryOpToken,
	pub binding_power: u8,
	pub direction: UnaryDirection,
}
impl UnaryOperator
{
	fn eval_fn<R: RangeRng>(self) -> impl Fn(&Operand, &mut R) -> Operand
	{
		match self.token
		{
			UnaryOpToken::Plus => functions::unary_plus,
			UnaryOpToken::Minus => functions::unary_minus,
			UnaryOpToken::Dice => functions::unary_dice,
		}
	}

	pub fn eval<R: RangeRng>(&self, operand: &Operand, rng: &mut R) -> Operand
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BinaryOperator
{
	pub token: OpToken, // if we add an operator that cant be binary, we have to change this -morgan 2024-01-04
	pub binding_power: BindingPower,
}
impl BinaryOperator
{
	fn eval_fn<R: RangeRng>(
		self,
	) -> impl Fn(&Operand, &Operand, &mut R) -> Result<Operand, FilterNumberError>
	{
		match self.token
		{
			OpToken::Plus => functions::add,
			OpToken::Minus => functions::subtract,
			OpToken::Multiply => functions::multiply,
			OpToken::Divide => functions::divide,
			OpToken::Modulus => functions::modulo,
			OpToken::Power => functions::power,
			OpToken::Dice => functions::dice,
			OpToken::Equals => functions::equal,
			OpToken::NotEquals => functions::not_equal,
			OpToken::GreaterThan => functions::greater,
			OpToken::LessThan => functions::less,
			OpToken::GreaterOrEqual => functions::greater_or_equal,
			OpToken::LessOrEqual => functions::less_or_equal,
		}
	}

	pub fn eval<R: RangeRng>(
		&self,
		lhs: &Operand,
		rhs: &Operand,
		random: &mut R,
	) -> Result<Operand, FilterNumberError>
	{
		self.eval_fn()(lhs, rhs, random)
	}
}
impl From<OpToken> for BinaryOperator
{
	fn from(token: OpToken) -> Self
	{
		use OpToken as Op;
		Self {
			token,
			binding_power: match token
			{
				Op::Plus | Op::Minus => BindingPower::new(1, 2),
				Op::Multiply | Op::Divide | Op::Modulus => BindingPower::new(3, 4),
				Op::Equals
				| Op::NotEquals
				| Op::GreaterThan
				| Op::LessThan
				| Op::GreaterOrEqual
				| Op::LessOrEqual => BindingPower::new(5, 6),
				Op::Power => BindingPower::new(10, 9),
				Op::Dice => BindingPower::new(11, 12),
			},
		}
	}
}

#[derive(Debug, Error, Clone, Copy)]
#[error("Failed to convert {} to unary operator!", .0)]
pub struct InvalidOperatorError(OpToken);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BindingPower
{
	pub left: u8,
	pub right: u8,
}
impl BindingPower
{
	pub const fn new(left: u8, right: u8) -> Self
	{
		Self { left, right }
	}
}
