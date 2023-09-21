pub mod functions;
mod operand;
mod roll_types;
pub use operand::*;
pub use roll_types::*;

use crate::{
	parsing::{self, Node},
	Error,
};

pub fn eval_string(input: &str) -> Result<f64, Error>
{
	let mut rpn_queue = parsing::rpn_queue_from(input)?;
	let mut eval_stack = Vec::<Operand>::new();

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
					eval_stack.push(result);
				}
			}
		}
		else
		{
			break;
		}
	}

	match eval_stack.pop()
	{
		Some(i) => Ok(i.value()),
		// this needs to change later
		None => Err(Error::MissingOperand {
			expected: 0,
			found: 0,
		}),
	}
}
