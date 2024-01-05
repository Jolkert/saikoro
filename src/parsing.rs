mod operators;
pub mod tokenization;
pub use operators::*;

use thiserror::Error;
use tokenization::{
	PeekableTokenStream, Token, TokenType, TokenizationError, UnexpectedTokenError,
};

#[derive(Debug, PartialEq)]
pub enum Node
{
	Binary
	{
		operator: BinaryOperator,
		left: Box<Self>,
		right: Box<Self>,
	},
	Unary
	{
		operator: UnaryOperator,
		argument: Box<Self>,
	},
	Leaf(f64),
}

pub fn parse_tree_from(input: &str) -> Result<Node, ParsingError>
{
	parse_min_power(&mut PeekableTokenStream::new(input), 0)
}
fn parse_min_power(stream: &mut PeekableTokenStream, min_power: u8) -> Result<Node, ParsingError>
{
	let mut lhs = match stream.next().transpose()?
	{
		Some(Token::Number(n)) => Ok(Node::Leaf(n)),
		Some(Token::Operator(op_token)) =>
		{
			let operator = UnaryOperator::try_from(op_token)?;
			if operator.direction != UnaryDirection::Prefix
			{
				return Err(UnaryWrongDirectionError {
					operator,
					expected_direction: UnaryDirection::Prefix,
				}
				.into());
			}
			Ok(Node::Unary {
				operator,
				argument: Box::new(parse_min_power(stream, operator.binding_power)?),
			})
		}
		Some(Token::OpenDelimiter) =>
		{
			let value = parse_min_power(stream, 0)?;
			stream.consume_expecting(TokenType::CloseDelimiter)?;
			Ok(value)
		}
		token => Err(UnexpectedTokenError {
			found: token.map(|it| it.token_type()),
			expected: TokenType::Number | TokenType::Operator,
		}),
	}?;

	while let Some(peeked) = stream.peek()
	{
		let op = match peeked
		{
			Ok(Token::Operator(op)) => Ok(BinaryOperator::from(*op)),
			Ok(Token::CloseDelimiter) => break,
			result =>
			{
				let token = result.clone()?;
				Err(UnexpectedTokenError {
					found: Some(token.token_type()),
					expected: TokenType::Operator.into(),
				})
			}
		}?;

		let binding_power = op.binding_power;
		if binding_power.left < min_power
		{
			break;
		}

		stream.consume()?;
		let rhs = parse_min_power(stream, binding_power.right)?;
		lhs = Node::Binary {
			operator: op,
			left: Box::new(lhs),
			right: Box::new(rhs),
		}
	}

	Ok(lhs)
}

#[derive(Debug, Error, Clone, Copy)]
pub enum ParsingError
{
	#[error("{}", .0)]
	Tokenization(#[from] TokenizationError),
	#[error("{}", .0)]
	InvalidOperator(#[from] InvalidOperatorError),
	#[error("{}", .0)]
	UnexpectedToken(#[from] UnexpectedTokenError),
	#[error("{}", .0)]
	UnaryWrongDirection(#[from] UnaryWrongDirectionError),
}

#[derive(Debug, Error, Clone, Copy)]
#[error("Expected {} operator, found {:?}", .expected_direction, .operator)]
pub struct UnaryWrongDirectionError
{
	pub operator: UnaryOperator,
	pub expected_direction: UnaryDirection,
}
