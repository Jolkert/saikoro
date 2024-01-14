use super::TokenizationError;
use crate::operator::{CompOperator, OpToken, UnaryDirection, UnaryOperator};
use thiserror::Error;

/// An error representing any error that can occur while parsing a dice string (including any
/// errors which can occur during tokenization)
#[derive(Debug, Error, Clone, Copy)]
pub enum ParsingError
{
	#[error("{}", .0)]
	Tokenization(#[from] TokenizationError),
	#[error("{}", .0)]
	InvalidOperator(#[from] InvalidOperatorError),
	#[error("{}", .0)]
	UnaryWrongDirection(#[from] UnaryWrongDirectionError),
	#[error("{}", .0)]
	UnmatchedCloseDelimiter(#[from] UnmatchedCloseDelimiterError),
	#[error("{}", .0)]
	UnmatchedComparison(#[from] UnmatchedComparisonError),
}

// this actually shouldn't be possible at the moment? at least not until there's a postfix operator
// covering future bases isnt a bad idea though cause ! will probably come at some point
// morgan 2024-01-10
/// An error representing a prefix operator in postfix position or vice-versa.
/// # Usage Notes
/// As of version 1.0, no postfix unary operators are yet implemented and therefore this error cannot
/// yet occur. However, there are plans to implement them, so this error exists preemptively.
#[derive(Debug, Error, Clone, Copy)]
#[error("Expected {} operator, found {:?}", .expected_direction, .operator)]
pub struct UnaryWrongDirectionError
{
	pub operator: UnaryOperator,
	pub expected_direction: UnaryDirection,
}

/// An error representing an operator that cannot be used as a unary operator
#[derive(Debug, Error, Clone, Copy)]
#[error("Failed to convert {} to unary operator!", .0)]
pub struct InvalidOperatorError(pub OpToken);

/// An error representing a found closing parenthesis without a matching open parenthesis
#[derive(Debug, Error, Clone, Copy)]
#[error("Found closing parenthesis not matching any open!")]
pub struct UnmatchedCloseDelimiterError;

/// An error representing a found comparison operator without a matching dice operator
#[derive(Debug, Error, Clone, Copy)]
#[error("Found comparison operator `{}` without a matching dice operator!", .0)]
pub struct UnmatchedComparisonError(pub CompOperator);
