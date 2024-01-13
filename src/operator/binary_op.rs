use super::{
	function::{self, OperatorError},
	OpToken, Operator,
};
use crate::{
	error::BadOperandError,
	evaluation::{Operand, OperandType},
	RangeRng,
};

/// Represents an operator which takes two [`Operand`]s as its arguments
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BinaryOperator
{
	pub token: OpToken, // if we add an operator that cant be binary, we have to change this -morgan 2024-01-04
	pub(crate) binding_power: BindingPower,
}
impl BinaryOperator
{
	fn eval_fn<R: RangeRng>(
		self,
	) -> impl Fn(Operand, Operand, &mut R) -> Result<Operand, OperatorError>
	{
		match self.token
		{
			OpToken::Plus => function::add,
			OpToken::Minus => function::subtract,
			OpToken::Multiply => function::multiply,
			OpToken::Divide => function::divide,
			OpToken::Modulus => function::modulo,
			OpToken::Power => function::power,
			OpToken::Dice => function::dice,
			OpToken::Equals => function::equal,
			OpToken::NotEquals => function::not_equal,
			OpToken::GreaterThan => function::greater,
			OpToken::LessThan => function::less,
			OpToken::GreaterOrEqual => function::greater_or_equal,
			OpToken::LessOrEqual => function::less_or_equal,
		}
	}

	/// Evaluates the given [`Operand`]s over the operator's evaluation function using the provided
	/// [`RangeRng`] where applicable
	/// # Examples
	/// ```rust
	/// # use saikoro::{error::BadOperandError, evaluation::Operand, operator::{BinaryOperator, OpToken}};
	/// # fn main() -> Result<(), BadOperandError> {
	/// let add = BinaryOperator::from(OpToken::Plus);
	/// // Passing a RangeRng is required even when it isn't used
	/// let result = add.eval(Operand::Number(1.0), Operand::Number(2.0), &mut rand::thread_rng())?;
	/// assert_eq!(result, Operand::Number(3.0));
	/// # Ok(())}
	/// ```
	/// # Errors
	/// Returns an error variant if either of the operands is invalid for the operator (e.g. using
	/// a [`Number`][`Operand::Number`] variant as the left-hand side of a comparison filter operator)
	pub fn eval<R: RangeRng>(
		&self,
		lhs: Operand,
		rhs: Operand,
		random: &mut R,
	) -> Result<Operand, BadOperandError>
	{
		self.eval_fn()(lhs, rhs, random).map_err(|err| match err
		{
			OperatorError::NumberComparisonLhs(lhs) => BadOperandError {
				operator: Operator::from(*self),
				argument_pos: 0,
				expected: OperandType::Roll,
				found: lhs,
			},
		})
	}
}
impl From<OpToken> for BinaryOperator
{
	fn from(token: OpToken) -> Self
	{
		use OpToken as Op;
		Self {
			token,
			binding_power: match token
			{
				Op::Plus | Op::Minus => BindingPower::new(1, 2),
				Op::Multiply | Op::Divide | Op::Modulus => BindingPower::new(3, 4),
				Op::Power => BindingPower::new(6, 5),
				Op::Equals
				| Op::NotEquals
				| Op::GreaterThan
				| Op::LessThan
				| Op::GreaterOrEqual
				| Op::LessOrEqual => BindingPower::new(9, 10),
				Op::Dice => BindingPower::new(11, 12),
			},
		}
	}
}

// Not redundant because it gets re-exported in saikoro::operator -morgan 2024-01-12
#[allow(clippy::redundant_pub_crate)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct BindingPower
{
	pub left: u8,
	pub right: u8,
}
impl BindingPower
{
	pub const fn new(left: u8, right: u8) -> Self
	{
		Self { left, right }
	}
}
