use crate::{
	evaluation::{
		functions::{self, MissingOperandError},
		Operand,
	},
	RangeRng,
};
use std::{fmt::Display, ops, str::FromStr};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Operator
{
	pub priority: Priority,
	pub valency: Valency,
	pub associativity: Associativity,
	pub token: OpToken,
}
impl Operator
{
	pub fn from_token(token: OpToken, valency: Valency) -> Self
	{
		let priority = match token
		{
			OpToken::Plus | OpToken::Minus => Priority::ADDITIVE,
			OpToken::Multiply | OpToken::Divide | OpToken::Modulus => Priority::MULTIPLICITIVE,
			OpToken::Power => Priority::POWER,
			OpToken::Dice => Priority::DICE,
			OpToken::Equals
			| OpToken::NotEquals
			| OpToken::GreaterThan
			| OpToken::LessThan
			| OpToken::GreaterOrEqual
			| OpToken::LessOrEqual => Priority::COMPARISON,
		};

		Self {
			priority,
			valency,
			associativity: if matches!(token, OpToken::Power)
			{
				Associativity::Right
			}
			else
			{
				Associativity::Left
			},
			token,
		}
	}

	pub fn eval_fn<R>(
		&self,
	) -> impl Fn(&mut Vec<Operand>, &mut R) -> Result<Operand, MissingOperandError>
	where
		R: RangeRng,
	{
		use OpToken as Token;
		match self.token
		{
			Token::Plus =>
			{
				if self.valency == Valency::Unary
				{
					functions::unary_plus
				}
				else
				{
					functions::add
				}
			}
			Token::Minus =>
			{
				if self.valency == Valency::Unary
				{
					functions::unary_minus
				}
				else
				{
					functions::subtract
				}
			}
			Token::Multiply => functions::multiply,
			Token::Divide => functions::divide,
			Token::Modulus => functions::modulo,
			Token::Power => functions::pow,
			Token::Dice => functions::roll,
			Token::Equals => functions::equal,
			Token::NotEquals => functions::not_equal,
			Token::GreaterThan => functions::greater,
			Token::LessThan => functions::less,
			Token::GreaterOrEqual => functions::greater_or_equal,
			Token::LessOrEqual => functions::less_or_equal,
		}
	}

	pub fn eval<R>(
		&self,
		stack: &mut Vec<Operand>,
		random: &mut R,
	) -> Result<Operand, MissingOperandError>
	where
		R: RangeRng,
	{
		self.eval_fn()(stack, random)
	}
}
impl From<(OpToken, Valency)> for Operator
{
	fn from(value: (OpToken, Valency)) -> Self
	{
		Self::from_token(value.0, value.1)
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Priority(u32);
impl Priority
{
	pub const ADDITIVE: Self = Self(0);
	pub const MULTIPLICITIVE: Self = Self(1);
	pub const COMPARISON: Self = Self(2);
	pub const POWER: Self = Self(3);
	pub const DICE: Self = Self(4);
}
impl ops::Add<u32> for Priority
{
	type Output = Self;

	fn add(self, rhs: u32) -> Self::Output
	{
		Self(self.0 + rhs)
	}
}
impl ops::Add<Associativity> for Priority
{
	type Output = Self;
	fn add(self, rhs: Associativity) -> Self::Output
	{
		self + u32::from(rhs == Associativity::Right)
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Valency
{
	Unary = 1,
	Binary,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Associativity
{
	Left,
	Right,
}

pub enum OpOrDelim
{
	Operator(Operator),
	Delimiter
	{
		is_open: bool,
	},
}

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
