use super::{
	function::{self, OperatorError},
	OpToken, Operator,
};
use crate::{
	errors::BadOperandError,
	evaluation::{Operand, OperandType},
	RangeRng,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BinaryOperator
{
	pub token: OpToken, // if we add an operator that cant be binary, we have to change this -morgan 2024-01-04
	pub binding_power: BindingPower,
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
				Op::Equals
				| Op::NotEquals
				| Op::GreaterThan
				| Op::LessThan
				| Op::GreaterOrEqual
				| Op::LessOrEqual => BindingPower::new(5, 6),
				Op::Power => BindingPower::new(10, 9),
				Op::Dice => BindingPower::new(11, 12),
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BindingPower
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
