use std::{ops, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct Operator
{
	pub priority: u16,
	pub valency: u8,
	pub associativity: Associativity,
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Associativity
{
	Left = 0,
	Right = 1,
}
impl ops::Add<Associativity> for u16
{
	type Output = u16;
	fn add(self, rhs: Associativity) -> Self::Output
	{
		self + if rhs == Associativity::Left { 0 } else { 1 }
	}
}

pub enum OpOrDelim
{
	Operator(Operator),
	Delimiter
	{
		is_open: bool,
	},
}

#[derive(Debug, PartialEq, Eq)]
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
impl OperatorToken
{
	pub fn priority(&self) -> u16
	{
		match self
		{
			Self::Plus | Self::Minus => 0,
			Self::Multiply | Self::Divide | Self::Modulus => 1,
			Self::Power => 2,
			Self::Dice => 3,
			Self::Equals
			| Self::NotEquals
			| Self::GreaterThan
			| Self::LessThan
			| Self::GreaterOrEqual
			| Self::LessOrEqual => 4,
		}
	}
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
