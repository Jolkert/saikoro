use super::TokenType;
use crate::tokenization::TOKEN_TYPES;
use std::{fmt::Display, ops};

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

impl ops::BitOr<Self> for TokenType
{
	type Output = TokenFlags;
	fn bitor(self, rhs: Self) -> Self::Output
	{
		TokenFlags(self as u8 | rhs as u8)
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	#[test]
	fn bit_or()
	{
		assert_eq!(
			TokenFlags(0b0_0011),
			TokenType::Number | TokenType::Operator
		);

		assert_eq!(
			TokenFlags(0b0_1011),
			TokenFlags(0b0_0011) | TokenType::OpenDelimiter
		);

		assert_eq!(TokenFlags(0b1111), TokenFlags(0b1100) | TokenFlags(0b0011));
	}
}
