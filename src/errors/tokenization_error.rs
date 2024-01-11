use crate::tokenization::{TokenFlags, TokenType};
use thiserror::Error;

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
