use super::{ParsingError, TokenizationError};
use crate::{
	evaluation::{Operand, OperandType},
	operator::Operator,
};
use thiserror::Error;

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

#[derive(Debug, Error)]
#[error("Operator ")]
pub struct BadOperandError
{
	pub operator: Operator,
	pub argument_pos: u8,
	pub expected: OperandType,
	pub found: Operand,
}
