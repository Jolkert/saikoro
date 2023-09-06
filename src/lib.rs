mod parsing;

#[cfg(test)]
mod tests
{
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
		let output = parsing::rpn_queue_from("2+7*3");

		assert_eq!(output[0], Node::Number(2.0));
		assert_eq!(output[1], Node::Number(7.0));
		assert_eq!(output[2], Node::Number(3.0));
		assert_eq!(
			output[3],
			Node::Operator(Operator {
				priority: OperatorToken::Multiply.priority(),
				valency: 2,
				associativity: Associativity::Left,
			})
		);
		assert_eq!(
			output[4],
			Node::Operator(Operator {
				priority: OperatorToken::Plus.priority(),
				valency: 2,
				associativity: Associativity::Left,
			})
		);
	}
}
