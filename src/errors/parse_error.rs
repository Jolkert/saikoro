use super::TokenizationError;
use crate::operator::{OpToken, UnaryDirection, UnaryOperator};
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy)]
pub enum ParsingError
{
	#[error("{}", .0)]
	Tokenization(#[from] TokenizationError),
	#[error("{}", .0)]
	InvalidOperator(#[from] InvalidOperatorError),
	#[error("{}", .0)]
	UnaryWrongDirection(#[from] UnaryWrongDirectionError),
}

// this actually shouldn't be possible at the moment? at least not until there's a postfix operator
// covering future bases isnt a bad idea though cause ! will probably come at some point
// morgan 2024-01-10
#[derive(Debug, Error, Clone, Copy)]
#[error("Expected {} operator, found {:?}", .expected_direction, .operator)]
pub struct UnaryWrongDirectionError
{
	pub operator: UnaryOperator,
	pub expected_direction: UnaryDirection,
}

#[derive(Debug, Error, Clone, Copy)]
#[error("Failed to convert {} to unary operator!", .0)]
pub struct InvalidOperatorError(pub OpToken);
