//! Types used in the evaluation of dice expressions

mod operand;
mod roll_types;

pub use operand::*;
pub use roll_types::*;

use crate::{error::ParsingError, parsing::Node, RangeRng};
use std::{collections::HashMap, hash::Hash};

pub(super) fn evaluate_tree<R>(
	parse_tree: Node,
	rng: &mut R,
) -> Result<DiceEvaluation, ParsingError>
where
	R: RangeRng,
{
	let mut rolls = OrderedMap::<RollId, RollGroup>::new();

	let value = evaluate_node(parse_tree, rng, &mut rolls)?.value();

	Ok(DiceEvaluation {
		value,
		roll_groups: rolls.values().cloned().collect(),
	})
}
fn evaluate_node<R>(
	node: Node,
	rng: &mut R,
	rolls: &mut OrderedMap<RollId, RollGroup>,
) -> Result<Operand, ParsingError>
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
		),
		Node::ComparisonTernary {
			comp_op: comp_operator,
			dice_left,
			dice_right,
			compare_to,
		} => comp_operator.eval(
			evaluate_node(*dice_left, rng, rolls)?,
			evaluate_node(*dice_right, rng, rolls)?,
			evaluate_node(*compare_to, rng, rolls)?,
			rng,
		),
	};

	if let Operand::Roll { id, data } = &operand
	{
		rolls.insert(*id, data.clone());
	}

	Ok(operand)
}

#[derive(Debug, Clone)]
struct OrderedMap<K, V>
{
	map: HashMap<K, V>,
	insertion_order: Vec<K>,
}
impl<K, V> OrderedMap<K, V>
{
	fn new() -> Self
	{
		Self {
			map: HashMap::new(),
			insertion_order: Vec::new(),
		}
	}
}
impl<K: Eq + Hash + Clone, V> OrderedMap<K, V>
{
	fn insert(&mut self, k: K, v: V) -> Option<V>
	{
		if !self.map.contains_key(&k)
		{
			self.insertion_order.push(k.clone());
		}

		self.map.insert(k, v)
	}

	fn values(&self) -> impl Iterator<Item = &V>
	{
		self.insertion_order.iter().map(|key| &self.map[key])
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use crate::{
		parsing,
		test_helpers::{assert_approx_eq, RiggedRandom},
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

	fn eval_expect(input: &str) -> DiceEvaluation
	{
		eval_str(input).unwrap_or_else(|_| panic!("Could not evaluate `{input}`"))
	}
	fn eval_expect_rand<R: RangeRng>(input: &str, rand: &mut R) -> DiceEvaluation
	{
		eval_str_rand(input, rand).unwrap_or_else(|_| panic!("Could not evaluate `{input}`"))
	}

	fn eval_str(input: &str) -> Result<DiceEvaluation, ParsingError>
	{
		eval_str_rand(input, &mut thread_rng())
	}
	fn eval_str_rand<R: RangeRng>(input: &str, rand: &mut R)
		-> Result<DiceEvaluation, ParsingError>
	{
		let mut stream = TokenStream::new(input);
		let tree = parsing::parse_tree_from(&mut stream)?;
		evaluate_tree(tree, rand)
	}
}
