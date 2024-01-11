mod operand;
mod roll_types;

pub use operand::*;
pub use roll_types::*;

use crate::{errors::EvaluationError, parsing::Node, RangeRng};
use std::collections::HashMap;

pub(super) fn evaluate_tree<R>(
	parse_tree: Node,
	rng: &mut R,
) -> Result<DiceEvaluation, EvaluationError>
where
	R: RangeRng,
{
	let mut rolls = HashMap::<RollId, RollGroup>::new();

	let value = evaluate_node(parse_tree, rng, &mut rolls)?.value();

	Ok(DiceEvaluation {
		value,
		roll_groups: rolls.values().cloned().collect(),
	})
}
fn evaluate_node<R>(
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
		Node::Unary { operator, argument } =>
		{
			operator.eval(evaluate_node(*argument, rng, rolls)?, rng)
		}
		Node::Binary {
			operator,
			left,
			right,
		} => operator.eval(
			evaluate_node(*left, rng, rolls)?,
			evaluate_node(*right, rng, rolls)?,
			rng,
		)?,
	};

	if let Operand::Roll { id, data } = &operand
	{
		rolls.insert(*id, data.clone());
	}

	Ok(operand)
}

#[cfg(test)]
mod tests
{
	use super::*;
	use crate::{
		parsing,
		test_helpers::{assert_approx_eq, flip_result, RiggedRandom},
		tokenization::TokenStream,
	};
	use rand::thread_rng;

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
		eval_str(input).unwrap_or_else(|_| panic!("Could not evaluate `{input}`"))
	}
	fn eval_expect_rand<R: RangeRng>(input: &str, rand: &mut R) -> DiceEvaluation
	{
		eval_str_rand(input, rand).unwrap_or_else(|_| panic!("Could not evaluate `{input}`"))
	}

	fn eval_expect_err(input: &str) -> EvaluationError
	{
		flip_result(eval_str(input))
			.unwrap_or_else(|_| panic!("Unexpected successful evaluation of `{input}`"))
	}

	fn eval_str(input: &str) -> Result<DiceEvaluation, EvaluationError>
	{
		eval_str_rand(input, &mut thread_rng())
	}
	fn eval_str_rand<R: RangeRng>(
		input: &str,
		rand: &mut R,
	) -> Result<DiceEvaluation, EvaluationError>
	{
		let mut stream = TokenStream::new(input);
		let tree = parsing::parse_tree_from(&mut stream)?;
		evaluate_tree(tree, rand)
	}
}
