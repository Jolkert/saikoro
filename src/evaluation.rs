pub mod functions;
mod operand;
mod roll_types;
use std::collections::HashMap;

pub use operand::*;
pub use roll_types::*;

use crate::{
	parsing::{self, tokenization::TokenizationError, Node},
	RangeRng,
};
use rand::thread_rng;
use thiserror::Error;

use self::functions::MissingOperandError;

#[derive(Debug)]
pub struct DiceEvaluation
{
	pub value: f64,
	pub rolls: Box<[DiceRoll]>, // this really isnt necessary
}

pub fn eval_string(input: &str) -> Result<DiceEvaluation, EvaluationError>
{
	eval_with_random(input, &mut thread_rng())
}

pub fn eval_with_random<R>(input: &str, random: &mut R) -> Result<DiceEvaluation, EvaluationError>
where
	R: RangeRng,
{
	let mut rpn_queue = parsing::rpn_queue_from(input)?;
	let mut eval_stack = Vec::<Operand>::new();
	let mut roll_list = HashMap::<RollId, DiceRoll>::new();

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
				rolls: roll_list.values().cloned().collect(),
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
