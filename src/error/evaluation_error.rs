use super::{ParsingError, TokenizationError};
use crate::{
	evaluation::{Operand, OperandType},
	operator::Operator,
};
use thiserror::Error;

/// An error representing any error that can occur while evaluating a dice string (including any
/// errors which can occur during parsing or tokenization)
#[derive(Debug, Error)]
pub enum EvaluationError
{
	#[error("{}", .0)]
	Tokenization(#[from] TokenizationError),
	#[error("{}", .0)]
	Parsing(#[from] ParsingError),
	#[error("{}", .0)]
	FilterNumber(#[from] BadOperandError),
}

/// An error representing an operand of the wrong type found while evauating an operator expression
#[derive(Debug, Error)]
#[error("Operator {:?} expected an operand of type {}, but found {:?} as argument {}", .operator, expected, .found, argument_pos)]
pub struct BadOperandError
{
	pub operator: Operator,
	pub argument_pos: u8,
	pub expected: OperandType,
	pub found: Operand,
}
