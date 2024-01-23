use crate::{
	error::{
		ParsingError, TokenizationError, UnaryWrongDirectionError, UnexpectedTokenError,
		UnmatchedCloseDelimiterError, UnmatchedComparisonError,
	},
	operator::{BinaryOperator, CompOperator, OpToken, UnaryDirection, UnaryOperator},
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
	ComparisonTernary
	{
		comp_op: CompOperator,
		dice_left: Box<Self>,
		dice_right: Box<Self>,
		compare_to: Box<Self>,
	},
	Leaf(f64),
}

pub fn parse_tree_from(stream: &mut TokenStream) -> Result<Node, ParsingError>
{
	parse_min_power(stream, 0, ParseContext::default())
}

// parsing function kinda has to be big. separating this out much further would not be great. at
// least for now -morgan 2024-01-22
#[allow(clippy::too_many_lines)]
fn parse_min_power(
	stream: &mut TokenStream,
	min_power: u8,
	context: ParseContext,
) -> Result<Node, ParsingError>
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
				argument: Box::new(parse_min_power(stream, operator.binding_power, context)?),
			}
		}
		Token::OpenDelimiter =>
		{
			let value = parse_min_power(stream, 0, context.expect_close_paren())?;
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
				let rhs = parse_min_power(stream, 0, context.expect_close_paren())?;
				lhs = Node::Binary {
					operator: OpToken::Multiply.into(),
					left: Box::new(lhs),
					right: Box::new(rhs),
				};
				break;
			}
			Ok(Token::CloseDelimiter) =>
			{
				if context.expecting_close_paren
				{
					break;
				}

				Err(ParsingError::from(UnmatchedCloseDelimiterError))
			}
			Ok(Token::ComparisonOperator(op)) =>
			{
				if context.expecting_comparison
				{
					break;
				}

				Err(ParsingError::from(UnmatchedComparisonError(*op)))
			}
			result =>
			{
				let token = result.clone()?;
				Err(TokenizationError::from(UnexpectedTokenError {
					found: Some(token.token_type()),
					expected: TokenType::Operator.into(),
				})
				.into())
			}
		}?;

		let binding_power = op.binding_power;
		if binding_power.left < min_power
		{
			break;
		}

		stream.consume()?; // consume current operator
		lhs = if op.token == OpToken::Dice
		{
			let rhs = parse_min_power(stream, binding_power.right, context.expect_comparison())?;
			if let Some(Ok(Token::ComparisonOperator(comp_op))) = stream.peek()
			{
				// deref now so mut borrow is possible next line -morgan 2024-01-14
				let comp_op = *comp_op;
				stream.consume_expecting(TokenType::ComparisonOperator)?;
				let compare_to = parse_min_power(stream, 0, context)?;
				Node::ComparisonTernary {
					comp_op,
					dice_left: Box::new(lhs),
					dice_right: Box::new(rhs),
					compare_to: Box::new(compare_to),
				}
			}
			else
			{
				// please stabalize if let chaining i beg you ;-; -morgan 2024-01-14
				Node::Binary {
					operator: op,
					left: Box::new(lhs),
					right: Box::new(rhs),
				}
			}
		}
		else
		{
			let rhs = parse_min_power(stream, binding_power.right, context)?;
			Node::Binary {
				operator: op,
				left: Box::new(lhs),
				right: Box::new(rhs),
			}
		}
	}

	Ok(lhs)
}

#[derive(Debug, Default, Copy, Clone)]
struct ParseContext
{
	expecting_comparison: bool,
	expecting_close_paren: bool,
}
impl ParseContext
{
	pub fn expect_close_paren(self) -> Self
	{
		Self {
			expecting_close_paren: true,
			..self
		}
	}

	pub fn expect_comparison(self) -> Self
	{
		Self {
			expecting_comparison: true,
			..self
		}
	}
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
					left: Box::new(Node::ComparisonTernary {
						comp_op: CompOperator::GreaterThan,
						dice_left: Box::new(Leaf(2.0)),
						dice_right: Box::new(Leaf(6.0)),
						compare_to: Box::new(Leaf(3.0)),
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
		assert_eq!(expected, expect_tree("(2d6 > 3) + 8d6 * 3^4 / 2 - 3 + -1"));
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
		expect_err_tree("(2");
		expect_err_tree("2)");
	}

	#[test]
	fn unmatched_comparison()
	{
		expect_err_tree("5 > 2");
		expect_err_tree("2d6 + 5 > 9");
	}

	#[test]
	fn juxtaposition_multiplication()
	{
		let two_by_three = Node::Binary {
			operator: OpToken::Multiply.into(),
			left: Box::new(Node::Leaf(2.0)),
			right: Box::new(Node::Leaf(3.0)),
		};

		assert_eq!(two_by_three, expect_tree("2 * 3"),);
		assert_eq!(two_by_three, expect_tree("2(3))"),);
		assert_eq!(two_by_three, expect_tree("(2)(3)"),);
	}

	fn expect_tree(input: &str) -> Node
	{
		parse_str(input).unwrap_or_else(|_| panic!("Could not parse `{input}`"))
	}
	fn expect_err_tree(input: &str) -> ParsingError
	{
		flip_result(parse_str(input))
			.unwrap_or_else(|_| panic!("Unexpected successful parse of `{input}`"))
	}

	fn parse_str(input: &str) -> Result<Node, ParsingError>
	{
		parse_tree_from(&mut TokenStream::new(input))
	}
}
