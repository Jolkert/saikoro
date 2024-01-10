use std::{fmt::Display, ops};

pub use super::operators::OpToken;

use lazy_regex::regex;
use regex::Regex;
use thiserror::Error;

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
pub(crate) enum Token
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

struct BackingTokenStream<'a>
{
	str: &'a str,
	cursor_index: usize,
}
impl<'a> BackingTokenStream<'a>
{
	pub fn new(str: &'a str) -> Self
	{
		Self {
			str,
			cursor_index: 0,
		}
	}
}
impl<'a> Iterator for BackingTokenStream<'a>
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
pub(crate) struct TokenStream<'a>
{
	token_stream: BackingTokenStream<'a>,
	lookahead: Option<Option<Result<Token, TokenizationError>>>,
}
impl<'a> TokenStream<'a>
{
	pub fn new(str: &'a str) -> Self
	{
		Self {
			token_stream: BackingTokenStream::new(str),
			lookahead: None,
		}
	}

	pub fn peek(&mut self) -> Option<&Result<Token, TokenizationError>>
	{
		self.lookahead
			.get_or_insert_with(|| self.token_stream.next())
			.as_ref()
	}

	pub fn expect<T>(&mut self, token_type: T) -> Result<Token, TokenizationError>
	where
		T: Into<TokenFlags>,
	{
		let token_flags = token_type.into();
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
					Ok(token)
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

	pub fn consume(&mut self) -> Result<(), TokenizationError>
	{
		self.next().transpose().map(|_| ())
	}

	pub fn consume_expecting<T>(&mut self, token_type: T) -> Result<(), TokenizationError>
	where
		T: Into<TokenFlags>,
	{
		self.expect(token_type).map(|_| ())
	}
}
impl<'a> Iterator for TokenStream<'a>
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

#[cfg(test)]
mod tests
{
	pub use super::*;

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

	#[test]
	fn flags_bitor()
	{
		assert_eq!(TokenFlags(0b0011), TokenType::Number | TokenType::Operator);

		assert_eq!(
			TokenFlags(0b0111),
			TokenFlags(0b0011) | TokenType::OpenDelimiter
		);

		assert_eq!(TokenFlags(0b1111), TokenFlags(0b1100) | TokenFlags(0b0011));
	}

	mod stream
	{
		use super::*;

		#[test]
		fn whitespace()
		{
			assert_eq!(
				TokenStream::new("1+2")
					.collect::<Result<Vec<_>, _>>()
					.unwrap(),
				TokenStream::new("1 + 2")
					.collect::<Result<Vec<_>, _>>()
					.unwrap(),
			);
		}

		#[test]
		fn construction()
		{
			assert_eq!(
				TokenStream::new("2+3-5")
					.collect::<Result<Vec<_>, _>>()
					.unwrap(),
				vec![
					Token::Number(2.0),
					Token::Operator(OpToken::Plus),
					Token::Number(3.0),
					Token::Operator(OpToken::Minus),
					Token::Number(5.0),
				]
			);
		}

		#[test]
		fn peek()
		{
			let mut stream = TokenStream::new("2+3");
			assert_eq!(
				stream.peek().unwrap().to_owned().unwrap(),
				Token::Number(2.0)
			);

			assert_eq!(stream.next().unwrap().unwrap(), Token::Number(2.0));
			assert_eq!(
				stream.next().unwrap().unwrap(),
				Token::Operator(OpToken::Plus)
			);
		}

		#[test]
		fn consume_ok()
		{
			let mut stream = TokenStream::new("1");
			assert!(stream.consume().is_ok());
		}
		#[test]
		fn consume_err()
		{
			let mut stream = TokenStream::new("ɧ");
			assert!(stream.consume().is_err());
		}

		#[test]
		fn expecting()
		{
			let mut stream = TokenStream::new("3+5");
			assert!(stream.consume_expecting(TokenType::Number).is_ok());
			assert!(stream.consume_expecting(TokenType::Operator).is_ok());
			assert!(stream.consume_expecting(TokenType::OpenDelimiter).is_err());
		}
	}
}
