pub mod functions;
mod operand;
mod roll_types;

pub use operand::*;
pub use roll_types::*;

use crate::{
	parsing::{self, tokenization::TokenizationError, Node},
	RangeRng,
};
use functions::MissingOperandError;
use rand::thread_rng;
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
	eval_with_random(input, &mut thread_rng())
}

pub fn eval_with_random<R>(input: &str, random: &mut R) -> Result<DiceEvaluation, EvaluationError>
where
	R: RangeRng,
{
	let mut rpn_queue = parsing::rpn_queue_from(input)?;
	let mut eval_stack = Vec::<Operand>::new();
	let mut roll_list = HashMap::<RollId, RollGroup>::new();

	while let Some(current) = rpn_queue.pop_front()
	{
		match current
		{
			Node::Number(n) => eval_stack.push(Operand::Number(n)),
			Node::Operator(op) => match op.eval(&mut eval_stack, random)?
			{
				Operand::Number(n) => eval_stack.push(Operand::Number(n)),
				Operand::Roll { id, data } =>
				{
					roll_list.insert(id, data.clone());
					eval_stack.push(Operand::Roll { id, data });
				}
			},
		}
	}

	eval_stack.pop().map_or_else(
		|| {
			// this needs to change later
			Err(EvaluationError::from(MissingOperandError {
				expected: 0,
				found: 0,
			}))
		},
		|operand| {
			Ok(DiceEvaluation {
				value: operand.value(),
				roll_groups: roll_list.values().cloned().collect(),
			})
		},
	)
}

#[derive(Debug, Error)]
pub enum EvaluationError
{
	#[error("{}", .0)]
	Tokenization(#[from] TokenizationError),
	#[error("{}", .0)]
	MissingOperand(#[from] MissingOperandError),
}
