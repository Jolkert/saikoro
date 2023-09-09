mod operators;
pub mod tokenization;
pub use operators::*;

use num_rational::Rational64 as r64;
use std::collections::VecDeque;
use tokenization::{InvalidTokenError, Token, TokenStream};

// TODO: better name -morgan 2023-09-03
#[derive(Debug, PartialEq)]
pub enum Node
{
	Number(r64),
	Operator(Operator),
}

pub fn rpn_queue_from(string: &str) -> Result<VecDeque<Node>, InvalidTokenError>
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
			Token::Operator(ref op_token) =>
			{
				// this is a *complete* mess -morgan 2023-09-04

				let operator = Operator::from_token(
					op_token,
					match previous
					{
						None | Some(Token::Operator(_)) => Valency::Unary,
						_ => Valency::Binary,
					},
				);

				push_operator_to_stack(operator, &mut operator_stack, &mut output_queue);
			}
			Token::Delimiter { is_open } =>
			{
				if is_open
				{
					match previous
					{
						Some(Token::Number(_)) | Some(Token::Delimiter { is_open: true }) =>
						{
							push_operator_to_stack(
								Operator::from_token(&OperatorToken::Multiply, Valency::Binary),
								&mut operator_stack,
								&mut output_queue,
							)
						}
						_ => (),
					}
				}
				else
				{
					loop
					{
						match operator_stack.pop()
						{
							Some(OpOrDelim::Operator(op)) =>
							{
								output_queue.push_back(Node::Operator(op))
							}
							_ => break,
						}
					}
				}
			}
		}

		previous = Some(token);
	}

	while let Some(OpOrDelim::Operator(op)) = operator_stack.pop()
	{
		print!("op!");
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
		match last
		{
			OpOrDelim::Operator(last_op) =>
			{
				if last_op.valency <= operator.valency
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
			_ => break,
		}
	}
	operator_stack.push(OpOrDelim::Operator(operator));
}
