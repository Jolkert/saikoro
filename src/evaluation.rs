pub mod functions;
mod operand;
mod roll_types;
use std::rc::Rc;

pub use operand::*;
pub use roll_types::*;

use crate::{
	parsing::{self, Node},
	Error,
};

#[derive(Debug)]
pub struct DiceEvaluation
{
	pub value: f64,
	pub rolls: Box<[Rc<DiceRoll>]>, // this really isnt necessary
}

pub fn eval_string(input: &str) -> Result<DiceEvaluation, Error>
{
	let mut rpn_queue = parsing::rpn_queue_from(input)?;
	let mut eval_stack = Vec::<Operand>::new();
	let mut roll_list = Vec::<Rc<DiceRoll>>::new();

	loop
	{
		if let Some(current) = rpn_queue.pop_front()
		{
			match current
			{
				Node::Number(n) => eval_stack.push(Operand::Number(n)),
				Node::Operator(op) =>
				{
					let result = op.eval(&mut eval_stack)?;
					match result
					{
						Some(Operand::Number(n)) => eval_stack.push(Operand::Number(n)),
						Some(Operand::Roll(r)) =>
						{
							roll_list.push(r.clone());
							eval_stack.push(Operand::Roll(r));
						}
						None => (),
					}
				}
			}
		}
		else
		{
			break;
		}
	}

	if let Some(operand) = eval_stack.pop()
	{
		Ok(DiceEvaluation {
			value: operand.value(),
			rolls: roll_list.into_boxed_slice(),
		})
	}
	else
	{
		// this needs to change later
		Err(Error::MissingOperand {
			expected: 0,
			found: 0,
		})
	}
}
