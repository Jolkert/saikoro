#![feature(assert_matches)]
#![feature(slice_pattern)]
mod parsing;

#[cfg(test)]
mod tests
{
	use std::assert_matches::assert_matches;

	use crate::parsing::tokenization::{OperatorToken, Token, TokenStream};
	use crate::parsing::{self, Associativity, Node, Operator};

	#[test]
	fn bsaic_tokenization_test()
	{
		let mut stream = TokenStream::new("4>=5");
		assert_eq!(stream.next().unwrap(), Token::Number(4.0));
		assert_eq!(
			stream.next().unwrap(),
			Token::Operator(OperatorToken::GreaterOrEqual)
		);
		assert_eq!(stream.next().unwrap(), Token::Number(5.0));
		assert!(stream.next().is_none());
	}

	#[test]
	fn tokenization_test()
	{
		let mut stream = TokenStream::new("2+7*3");
		assert_eq!(stream.next().unwrap(), Token::Number(2.0));
		assert_eq!(stream.next().unwrap(), Token::Operator(OperatorToken::Plus));
		assert_eq!(stream.next().unwrap(), Token::Number(7.0));
		assert_eq!(
			stream.next().unwrap(),
			Token::Operator(OperatorToken::Multiply)
		);
		assert_eq!(stream.next().unwrap(), Token::Number(3.0));
	}

	#[test]
	fn basic_parse_queue_test()
	{
		let mut output = parsing::rpn_queue_from("2+7*3");

		assert_matches!(
			output.make_contiguous(),
			[
				Node::Number(2.0),
				Node::Number(7.0),
				Node::Number(3.0),
				Node::Operator(Operator {
					priority: 1,
					valency: 2,
					associativity: Associativity::Left,
				}),
				Node::Operator(Operator {
					priority: 0,
					valency: 2,
					associativity: Associativity::Left,
				}),
			]
		)
	}
}
