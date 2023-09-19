use std::{ops, str::FromStr};

use crate::evaluation::{functions, Item};
use crate::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct Operator
{
	pub priority: Priority,
	pub valency: Valency,
	pub associativity: Associativity,
	pub token: OperatorToken,
}
impl Operator
{
	pub fn from_token(token: OperatorToken, valency: Valency) -> Self
	{
		let priority = match token
		{
			OperatorToken::Plus | OperatorToken::Minus => Priority::ADDITIVE,
			OperatorToken::Multiply | OperatorToken::Divide | OperatorToken::Modulus =>
			{
				Priority::MULTIPLICITIVE
			}
			OperatorToken::Power => Priority::POWER,
			OperatorToken::Dice => Priority::DICE,
			OperatorToken::Equals
			| OperatorToken::NotEquals
			| OperatorToken::GreaterThan
			| OperatorToken::LessThan
			| OperatorToken::GreaterOrEqual
			| OperatorToken::LessOrEqual => Priority::COMPARISON,
		};

		Operator {
			priority,
			valency,
			associativity: if matches!(token, OperatorToken::Power)
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

	pub fn eval_fn(&self) -> Box<dyn Fn(&mut Vec<Item>) -> Result<Item, Error>>
	{
		use OperatorToken as Token;
		Box::new(match self.token
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
		})
	}

	pub fn eval(&self, stack: &mut Vec<Item>) -> Result<Item, Error>
	{
		self.eval_fn()(stack)
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Priority(u32);
impl Priority
{
	pub const ADDITIVE: Priority = Priority(0);
	pub const MULTIPLICITIVE: Priority = Priority(1);
	pub const POWER: Priority = Priority(2);
	pub const DICE: Priority = Priority(3);
	pub const COMPARISON: Priority = Priority(4);
}
impl ops::Add<u32> for Priority
{
	type Output = Self;

	fn add(self, rhs: u32) -> Self::Output
	{
		Priority(self.0 + rhs)
	}
}
impl ops::Add<Associativity> for Priority
{
	type Output = Self;
	fn add(self, rhs: Associativity) -> Self::Output
	{
		self + if rhs == Associativity::Right { 1 } else { 0 }
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
pub enum OperatorToken
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

impl FromStr for OperatorToken
{
	type Err = OperatorParseError;

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
			_ => Err(OperatorParseError()),
		}
	}
}

#[derive(Debug)]
pub struct OperatorParseError();
