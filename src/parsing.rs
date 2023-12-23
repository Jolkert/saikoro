mod operators;
pub mod tokenization;
pub use operators::*;

use crate::Error;
use std::collections::VecDeque;
use tokenization::{Token, TokenStream};

// TODO: better name -morgan 2023-09-03
#[derive(Debug, PartialEq)]
pub enum Node
{
	Number(f64),
	Operator(Operator),
}
impl From<f64> for Node
{
	fn from(value: f64) -> Self
	{
		Self::Number(value)
	}
}
impl From<Operator> for Node
{
	fn from(value: Operator) -> Self
	{
		Self::Operator(value)
	}
}

pub fn rpn_queue_from(string: &str) -> Result<VecDeque<Node>, Error>
{
	let stream = TokenStream::new(string);
	let mut output_queue = VecDeque::<Node>::new();
	let mut operator_stack = Vec::<OpOrDelim>::new();

	let mut previous: Option<Token> = None;
	for token in stream
	{
		let token = token?;
		match token
		{
			Token::Number(num) =>
			{
				output_queue.push_back(Node::Number(num));
			}
			Token::Operator(op_token) =>
			{
				let operator =
					Operator::from_token(
						op_token,
						match previous
						{
							None
							| Some(Token::Operator(_) | Token::Delimiter { is_open: true }) => Valency::Unary,
							_ => Valency::Binary,
						},
					);

				push_operator_to_stack(operator, &mut operator_stack, &mut output_queue);
			}
			Token::Delimiter { is_open } =>
			{
				if is_open
				{
					if let Some(Token::Number(_) | Token::Delimiter { is_open: true }) = previous
					{
						push_operator_to_stack(
							Operator::from_token(OpToken::Multiply, Valency::Binary),
							&mut operator_stack,
							&mut output_queue,
						);
					}
				}
				else
				{
					while let Some(OpOrDelim::Operator(op)) = operator_stack.pop()
					{
						output_queue.push_back(Node::Operator(op));
					}
				}
			}
		}

		previous = Some(token);
	}

	while let Some(OpOrDelim::Operator(op)) = operator_stack.pop()
	{
		output_queue.push_back(Node::Operator(op));
	}

	Ok(output_queue)
}

fn push_operator_to_stack(
	operator: Operator,
	operator_stack: &mut Vec<OpOrDelim>,
	output_queue: &mut VecDeque<Node>,
)
{
	while let Some(last) = operator_stack.last()
	{
		if let OpOrDelim::Operator(last_op) = last
			&& last_op.valency <= operator.valency
			&& last_op.priority >= (operator.priority + operator.associativity)
		{
			if let Some(OpOrDelim::Operator(it)) = operator_stack.pop()
			{
				output_queue.push_back(Node::Operator(it));
			}
		}
		else
		{
			break;
		}
	}
	operator_stack.push(OpOrDelim::Operator(operator));
}
