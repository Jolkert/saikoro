use crate::{
	error::{ParsingError, TokenizationError, UnaryWrongDirectionError, UnexpectedTokenError},
	operator::{BinaryOperator, OpToken, UnaryDirection, UnaryOperator},
	tokenization::{Token, TokenStream, TokenType},
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

pub fn parse_tree_from(stream: &mut TokenStream) -> Result<Node, ParsingError>
{
	parse_min_power(stream, 0)
}
fn parse_min_power(stream: &mut TokenStream, min_power: u8) -> Result<Node, ParsingError>
{
	let mut lhs = match stream
		.expect(TokenType::Number | TokenType::Operator | TokenType::OpenDelimiter)?
	{
		Token::Number(n) => Node::Leaf(n),
		Token::Operator(op_token) =>
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
			Node::Unary {
				operator,
				argument: Box::new(parse_min_power(stream, operator.binding_power)?),
			}
		}
		Token::OpenDelimiter =>
		{
			let value = parse_min_power(stream, 0)?;
			stream.consume_expecting(TokenType::CloseDelimiter)?;
			value
		}
		_ => unreachable!("stream.expect should prevent this branch from ever occuring"),
	};

	while let Some(peeked) = stream.peek()
	{
		let op = match peeked
		{
			Ok(Token::Operator(op)) => Ok(BinaryOperator::from(*op)),
			Ok(Token::OpenDelimiter) =>
			{
				let rhs = parse_min_power(stream, 0)?;
				lhs = Node::Binary {
					operator: OpToken::Multiply.into(),
					left: Box::new(lhs),
					right: Box::new(rhs),
				};
				break;
			}
			Ok(Token::CloseDelimiter) => break,
			result =>
			{
				let token = result.clone()?;
				Err(TokenizationError::from(UnexpectedTokenError {
					found: Some(token.token_type()),
					expected: TokenType::Operator.into(),
				}))
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

#[cfg(test)]
mod tests
{
	use super::*;
	use crate::{test_helpers::flip_result, tokenization::TokenStream};

	#[test]
	fn single_token()
	{
		assert_eq!(Node::Leaf(4.0), expect_tree("4"));

		assert!(matches!(
			expect_err_tree("+"),
			ParsingError::Tokenization(TokenizationError::UnexpectedToken(_))
		));
	}

	#[test]
	fn simple_expression()
	{
		let expected = Node::Binary {
			operator: OpToken::Plus.into(),
			left: Box::new(Node::Leaf(1.0)),
			right: Box::new(Node::Leaf(2.0)),
		};
		assert_eq!(expected, expect_tree("1+2"));
	}

	#[test]
	fn complex_expression()
	{
		use Node::{Binary, Leaf, Unary};
		let expected = Binary {
			operator: OpToken::Plus.into(),
			left: Box::new(Binary {
				operator: OpToken::Minus.into(),
				left: Box::new(Binary {
					operator: OpToken::Plus.into(),
					left: Box::new(Binary {
						operator: OpToken::GreaterThan.into(),
						left: Box::new(Binary {
							operator: OpToken::Dice.into(),
							left: Box::new(Leaf(2.0)),
							right: Box::new(Leaf(6.0)),
						}),
						right: Box::new(Leaf(3.0)),
					}),
					right: Box::new(Binary {
						operator: OpToken::Divide.into(),
						left: Box::new(Binary {
							operator: OpToken::Multiply.into(),
							left: Box::new(Binary {
								operator: OpToken::Dice.into(),
								left: Box::new(Leaf(8.0)),
								right: Box::new(Leaf(6.0)),
							}),
							right: Box::new(Binary {
								operator: OpToken::Power.into(),
								left: Box::new(Leaf(3.0)),
								right: Box::new(Leaf(4.0)),
							}),
						}),
						right: Box::new(Leaf(2.0)),
					}),
				}),
				right: Box::new(Leaf(3.0)),
			}),
			right: Box::new(Unary {
				operator: OpToken::Minus.try_into().unwrap(),
				argument: Box::new(Leaf(1.0)),
			}),
		};
		assert_eq!(expected, expect_tree("2d6 > 3 + 8d6 * 3^4 / 2 - 3 + -1"));
	}

	#[test]
	fn invalid_unary()
	{
		assert!(matches!(
			expect_err_tree("/4"),
			ParsingError::InvalidOperator(_)
		));
	}

	#[test]
	fn unmatched_paren()
	{
		assert!(parse_tree_from(&mut TokenStream::new("(2")).is_err());
	}

	#[test]
	fn juxtaposition_multiplication()
	{
		let two_by_three = Node::Binary {
			operator: OpToken::Multiply.into(),
			left: Box::new(Node::Leaf(2.0)),
			right: Box::new(Node::Leaf(3.0)),
		};

		assert_eq!(two_by_three, expect_tree("2 * 3)"),);
		assert_eq!(two_by_three, expect_tree("2(3))"),);
		assert_eq!(two_by_three, expect_tree("(2)(3)"),);
	}

	fn expect_tree(input: &str) -> Node
	{
		parse_tree_from(&mut TokenStream::new(input))
			.unwrap_or_else(|_| panic!("Could not parse `{input}`"))
	}
	fn expect_err_tree(input: &str) -> ParsingError
	{
		flip_result(parse_tree_from(&mut TokenStream::new(input)))
			.unwrap_or_else(|_| panic!("Unexpected successful parse of `{input}`"))
	}
}
