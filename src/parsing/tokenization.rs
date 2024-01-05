use std::{fmt::Display, ops};

pub use super::operators::OpToken;

use lazy_regex::regex;
use regex::Regex;
use thiserror::Error;

static TOKEN_TYPES: [TokenType; 5] = [
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
impl ops::BitOr<Self> for TokenType
{
	type Output = TokenFlags;
	fn bitor(self, rhs: Self) -> Self::Output
	{
		TokenFlags(self as u8 | rhs as u8)
	}
}
impl Display for TokenType
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}", self.display_name())
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TokenFlags(u8);
impl TokenFlags
{
	pub const ANY: Self = Self(0b0001_1111);
	pub const NONE: Self = Self(0b0000_0000);

	pub fn has_set(self, token_type: TokenType) -> bool
	{
		let token_val = token_type as u8;
		self.0 & token_val == token_val
	}
}
impl ops::BitOr<Self> for TokenFlags
{
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self::Output
	{
		Self(self.0 | rhs.0)
	}
}
impl ops::BitOr<TokenType> for TokenFlags
{
	type Output = Self;
	fn bitor(self, rhs: TokenType) -> Self::Output
	{
		Self(self.0 | rhs as u8)
	}
}
impl From<TokenType> for TokenFlags
{
	fn from(value: TokenType) -> Self
	{
		Self(value as u8)
	}
}
impl Display for TokenFlags
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let set_types = TOKEN_TYPES
			.iter()
			.filter_map(|it| self.has_set(*it).then_some(it.display_name()))
			.collect::<Box<[&str]>>()
			.join(" | ");
		write!(f, "{set_types}")
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

pub struct TokenStream<'a>
{
	str: &'a str,
	cursor_index: usize,
}
impl<'a> TokenStream<'a>
{
	pub fn new(str: &'a str) -> Self
	{
		Self {
			str,
			cursor_index: 0,
		}
	}
}
impl<'a> Iterator for TokenStream<'a>
{
	type Item = Result<Token, TokenizationError>;
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.cursor_index >= self.str.len()
		{
			return None;
		}

		for token_type in TOKEN_TYPES.iter()
		{
			if let Some(mtch) = token_type.regex().find_at(self.str, self.cursor_index)
			{
				if mtch.start() != self.cursor_index
				{
					continue;
				}

				self.cursor_index += mtch.as_str().len();
				return match token_type
				{
					TokenType::Number =>
					{
						Some(Ok(Token::Number(mtch.as_str().parse::<f64>().unwrap())))
					}
					TokenType::Operator => Some(Ok(Token::Operator(
						mtch.as_str().parse::<OpToken>().unwrap(),
					))),
					TokenType::OpenDelimiter => Some(Ok(Token::OpenDelimiter)),
					TokenType::CloseDelimiter => Some(Ok(Token::CloseDelimiter)),
					TokenType::Whitespace => self.next(),
				};
			}
		}

		Some(Err(TokenizationError::from(UnknownTokenError {
			unknown_char: self.str.chars().nth(self.cursor_index).unwrap(),
			index: self.cursor_index,
		})))
	}
}

// I was using Peekable<TokenStream> before,
// but i want to be able to call methods on the underlying iterator too
// also borrow checker means i have to wrap it in a new type lol
// -morgan 2024-01-02
#[allow(clippy::option_option)] // I promise this makes sense :pensive: -morgan 2024-01-02
pub struct PeekableTokenStream<'a>
{
	token_stream: TokenStream<'a>,
	lookahead: Option<Option<Result<Token, TokenizationError>>>,
}
impl<'a> PeekableTokenStream<'a>
{
	pub fn new(str: &'a str) -> Self
	{
		Self {
			token_stream: TokenStream::new(str),
			lookahead: None,
		}
	}

	pub fn peek(&mut self) -> Option<&Result<Token, TokenizationError>>
	{
		self.lookahead
			.get_or_insert_with(|| self.token_stream.next())
			.as_ref()
	}

	pub fn consume(&mut self) -> Result<(), TokenizationError>
	{
		self.next().transpose().map(|_| ())
	}

	pub fn consume_expecting<T>(&mut self, token_type: T) -> Result<(), TokenizationError>
	where
		T: Into<TokenFlags>,
	{
		let token_flags: TokenFlags = token_type.into();
		let next = self.next().transpose()?;

		next.map_or_else(
			|| {
				Err(UnexpectedTokenError {
					found: None,
					expected: token_flags,
				}
				.into())
			},
			|token| {
				if token_flags.has_set(token.token_type())
				{
					Ok(())
				}
				else
				{
					Err(UnexpectedTokenError {
						found: Some(token.token_type()),
						expected: token_flags,
					}
					.into())
				}
			},
		)
	}
}
impl<'a> Iterator for PeekableTokenStream<'a>
{
	type Item = Result<Token, TokenizationError>;
	fn next(&mut self) -> Option<Self::Item>
	{
		match self.lookahead.take()
		{
			Some(it) => it,
			None => self.token_stream.next(),
		}
	}
}

#[derive(Debug, Error, Clone, Copy)]
pub enum TokenizationError
{
	#[error("{}", .0)]
	UnknownToken(#[from] UnknownTokenError),
	#[error("{}", .0)]
	UnexpectedToken(#[from] UnexpectedTokenError),
}

#[derive(Debug, Error, Clone, Copy)]
#[error("Found unknown token: '{}' at index {}", .unknown_char, .index)]
pub struct UnknownTokenError
{
	pub unknown_char: char,
	pub index: usize,
}

#[derive(Debug, Error, Clone, Copy)]
#[error("Found token `{:?}` when `{}` was expected", .found, .expected)]
pub struct UnexpectedTokenError
{
	pub found: Option<TokenType>,
	pub expected: TokenFlags,
}
