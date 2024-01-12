use crate::tokenization::{TokenFlags, TokenType};
use thiserror::Error;

/// An error representing any error that can occur while a dice string is being tokenized
#[derive(Debug, Error, Clone, Copy)]
pub enum TokenizationError
{
	#[error("{}", .0)]
	UnknownToken(#[from] UnknownTokenError),
	#[error("{}", .0)]
	UnexpectedToken(#[from] UnexpectedTokenError),
}

/// An error representing an unsupported token
#[derive(Debug, Error, Clone, Copy)]
#[error("Found unknown token: '{}' at index {}", .unknown_char, .index)]
pub struct UnknownTokenError
{
	pub unknown_char: char,
	pub index: usize,
}

/// An error representing a token in a position where a different token was expected
#[derive(Debug, Error, Clone, Copy)]
#[error("Found token `{:?}` when `{}` was expected", .found, .expected)]
pub struct UnexpectedTokenError
{
	pub found: Option<TokenType>,
	pub expected: TokenFlags,
}
