pub mod functions;
mod operand;
mod roll_types;

pub use operand::*;
pub use roll_types::*;

use crate::{
	parsing::{self, tokenization::TokenizationError, BadOperandError, Node, ParsingError},
	RangeRng,
};
use std::{collections::HashMap, fmt::Display};
use thiserror::Error;

#[derive(Debug)]
pub struct DiceEvaluation
{
	pub value: f64,
	pub roll_groups: Box<[RollGroup]>,
}
impl DiceEvaluation
{
	pub fn ungrouped_rolls(&self) -> impl Iterator<Item = &Roll>
	{
		self.roll_groups.iter().flat_map(RollGroup::iter)
	}
}
impl Display for DiceEvaluation
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(
			f,
			"Total: {} [{}]",
			self.value,
			self.roll_groups
				.iter()
				.map(ToString::to_string)
				.collect::<Vec<_>>()
				.join(", ")
		)
	}
}

pub fn evaluate(input: &str) -> Result<DiceEvaluation, EvaluationError>
{
	eval_with_random(input, &mut rand::thread_rng())
}
pub fn eval_with_random<R>(input: &str, rng: &mut R) -> Result<DiceEvaluation, EvaluationError>
where
	R: RangeRng,
{
	let mut rolls = HashMap::<RollId, RollGroup>::new();

	let root = parsing::parse_tree_from(input)?;
	let value = eval_node(root, rng, &mut rolls)?.value();

	Ok(DiceEvaluation {
		value,
		roll_groups: rolls.values().cloned().collect(),
	})
}
fn eval_node<R>(
	node: Node,
	rng: &mut R,
	rolls: &mut HashMap<RollId, RollGroup>,
) -> Result<Operand, EvaluationError>
where
	R: RangeRng,
{
	let operand = match node
	{
		Node::Leaf(n) => Operand::Number(n),
		Node::Unary { operator, argument } => operator.eval(eval_node(*argument, rng, rolls)?, rng),
		Node::Binary {
			operator,
			left,
			right,
		} => operator.eval(
			eval_node(*left, rng, rolls)?,
			eval_node(*right, rng, rolls)?,
			rng,
		)?,
	};

	if let Operand::Roll { id, data } = &operand
	{
		rolls.insert(*id, data.clone());
	}

	Ok(operand)
}

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

#[cfg(test)]
mod tests
{
	use super::*;
	use crate::test_helpers::{assert_approx_eq, flip_result, RiggedRandom};

	#[test]
	fn deterministic_evaluation()
	{
		assert_approx_eq!(5.0, eval_expect("2 + 3").value);
	}

	#[test]
	fn dice_evaluation()
	{
		let evaluation = eval_expect_rand("2d6", &mut RiggedRandom::new([3, 6]));
		assert_approx_eq!(9.0, evaluation.value);
		assert_eq!(
			vec![3, 6],
			evaluation
				.ungrouped_rolls()
				.map(|it| it.original_value)
				.collect::<Vec<_>>()
		);
	}

	#[test]
	fn invalid_comparison()
	{
		assert!(matches!(
			eval_expect_err("2 > 1"),
			EvaluationError::FilterNumber(_)
		));
	}

	fn eval_expect(input: &str) -> DiceEvaluation
	{
		evaluate(input).unwrap_or_else(|_| panic!("Could not evaluate `{input}`"))
	}
	fn eval_expect_rand<R: RangeRng>(input: &str, rand: &mut R) -> DiceEvaluation
	{
		eval_with_random(input, rand).unwrap_or_else(|_| panic!("Could not evaluate `{input}`"))
	}

	fn eval_expect_err(input: &str) -> EvaluationError
	{
		flip_result(evaluate(input))
			.unwrap_or_else(|_| panic!("Unexpected successful evaluation of `{input}`"))
	}
}
