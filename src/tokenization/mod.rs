mod flags;
mod stream;

pub use flags::*;
pub use stream::*;

use crate::operator::OpToken;
use lazy_regex::regex;
use regex::Regex;
use std::fmt::Display;

static TOKEN_TYPES: &[TokenType] = &[
	TokenType::Number,
	TokenType::Operator,
	TokenType::OpenDelimiter,
	TokenType::CloseDelimiter,
	TokenType::Whitespace,
];
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType
{
	Number = 1 << 0,
	Operator = 1 << 1,
	OpenDelimiter = 1 << 2,
	CloseDelimiter = 1 << 3,
	Whitespace = 1 << 4,
}
impl TokenType
{
	pub fn regex(self) -> &'static Regex
	{
		match self
		{
			Self::Number => regex!(r"\d+(\.\d+)?"),
			Self::Operator => regex!(r"[\+\-\*\/%^dD]|[=!<>]=|>|<"),
			Self::OpenDelimiter => regex!(r"\("),
			Self::CloseDelimiter => regex!(r"\)"),
			Self::Whitespace => regex!(r"\s+"),
		}
	}

	fn display_name(self) -> &'static str
	{
		match self
		{
			Self::Number => "Number",
			Self::Operator => "Operator",
			Self::OpenDelimiter => "OpenDelim",
			Self::CloseDelimiter => "CloseDelim",
			Self::Whitespace => "Whitespace",
		}
	}
}

impl Display for TokenType
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}", self.display_name())
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token
{
	Number(f64),
	Operator(OpToken),
	OpenDelimiter,
	CloseDelimiter,
}
impl Token
{
	pub fn token_type(&self) -> TokenType
	{
		match self
		{
			Self::Number(_) => TokenType::Number,
			Self::Operator(_) => TokenType::Operator,
			Self::OpenDelimiter => TokenType::OpenDelimiter,
			Self::CloseDelimiter => TokenType::CloseDelimiter,
		}
	}
}
impl From<f64> for Token
{
	fn from(value: f64) -> Self
	{
		Self::Number(value)
	}
}
impl From<OpToken> for Token
{
	fn from(value: OpToken) -> Self
	{
		Self::Operator(value)
	}
}

#[cfg(test)]
mod test
{
	use super::*;

	#[test]
	fn type_from_token()
	{
		assert_eq!(Token::Number(3.0).token_type(), TokenType::Number);
		assert_eq!(
			Token::Operator(OpToken::Plus).token_type(),
			TokenType::Operator
		);
		assert_eq!(Token::OpenDelimiter.token_type(), TokenType::OpenDelimiter);
		assert_eq!(
			Token::CloseDelimiter.token_type(),
			TokenType::CloseDelimiter
		);
	}
}
