use super::{Token, TokenFlags, TokenType, TOKEN_TYPES};
use crate::{
	errors::{TokenizationError, UnexpectedTokenError, UnknownTokenError},
	operator::OpToken,
};

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
pub struct TokenStream<'a>
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

#[cfg(test)]
mod tests
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
