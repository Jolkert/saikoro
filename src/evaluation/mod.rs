//! Types used in the evaluation of dice expressions

mod operand;
mod roll_types;
mod symbol_table;

use std::{collections::HashMap, hash::Hash};

pub use operand::*;
use rand::SeedableRng;
pub use roll_types::*;
pub use symbol_table::*;

use crate::{RangeRng, error::MissingSymbolError, parsing::Node};

// TODO: docs
pub fn eval_tree(
	parsed_tree: &Node,
	symbol_table: Option<&SymbolTable>,
) -> Result<DiceEvaluation, MissingSymbolError>
{
	eval_tree_with_rand(parsed_tree, &mut rand::rng(), symbol_table)
}

// TODO: docs
pub fn eval_tree_with_seed(
	parsed_tree: &Node,
	seed: u64,
	symbol_table: Option<&SymbolTable>,
) -> Result<DiceEvaluation, MissingSymbolError>
{
	let mut seeded_random = rand::rngs::StdRng::seed_from_u64(seed);
	eval_tree_with_rand(parsed_tree, &mut seeded_random, symbol_table)
}

// TODO: docs
pub fn eval_tree_with_rand<R>(
	parsed_tree: &Node,
	rng: &mut R,
	symbol_table: Option<&SymbolTable>,
) -> Result<DiceEvaluation, MissingSymbolError>
where
	R: RangeRng,
{
	if let Some(symbol_table) = symbol_table
	{
		evaluate_tree_internal(parsed_tree, rng, symbol_table)
	}
	else
	{
		evaluate_tree_internal(parsed_tree, rng, &SymbolTable::new())
	}
}

pub(super) fn evaluate_tree_internal<R>(
	parse_tree: &Node,
	rng: &mut R,
	symbol_table: &SymbolTable,
) -> Result<DiceEvaluation, MissingSymbolError>
where
	R: RangeRng,
{
	let mut rolls = OrderedMap::<RollId, RollGroup>::new();

	let value = evaluate_node(parse_tree, rng, &mut rolls, symbol_table)?.value();

	Ok(DiceEvaluation {
		value,
		roll_groups: rolls.values().cloned().collect(),
	})
}
fn evaluate_node<R>(
	node: &Node,
	rng: &mut R,
	rolls: &mut OrderedMap<RollId, RollGroup>,
	symbol_table: &SymbolTable,
) -> Result<Operand, MissingSymbolError>
where
	R: RangeRng,
{
	let operand = match node
	{
		Node::Leaf(n) => Operand::Number(*n),
		Node::Unary { operator, argument } =>
		{
			operator.eval(evaluate_node(argument, rng, rolls, symbol_table)?, rng)
		}
		Node::Binary {
			operator,
			left,
			right,
		} => operator.eval(
			evaluate_node(left, rng, rolls, symbol_table)?,
			evaluate_node(right, rng, rolls, symbol_table)?,
			rng,
		),
		Node::ComparisonTernary {
			comp_op: comp_operator,
			dice_left,
			dice_right,
			compare_to,
		} => comp_operator.eval(
			evaluate_node(dice_left, rng, rolls, symbol_table)?,
			evaluate_node(dice_right, rng, rolls, symbol_table)?,
			evaluate_node(compare_to, rng, rolls, symbol_table)?,
			rng,
		),
		Node::Symbolic(s) => evaluate_node(
			symbol_table
				.get(s)
				.ok_or_else(|| MissingSymbolError::from(s.clone()))?,
			rng,
			rolls,
			symbol_table,
		)?,
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
		error::SaikoroError,
		parsing,
		test_helpers::{RiggedRandom, assert_approx_eq},
		tokenization::TokenStream,
	};

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

	fn eval_str(input: &str) -> Result<DiceEvaluation, SaikoroError>
	{
		eval_str_rand(input, &mut rand::rng())
	}
	fn eval_str_rand<R: RangeRng>(input: &str, rand: &mut R)
	-> Result<DiceEvaluation, SaikoroError>
	{
		let mut stream = TokenStream::new(input);
		let tree = parsing::parse_tree_from(&mut stream)?;
		Ok(evaluate_tree_internal(&tree, rand, &SymbolTable::new())?) // todo: add support for a symbol table in this function
	}
}
