// TODO: please turn this off before you ever consider this even close to finished because that
// would be incredibly stupid of you. im just sick of seeing 30+ warnings when im still not done
// implementing everything >:( -morgan 20203-09-17
#![allow(dead_code)]
mod evaluation;
mod parsing;

#[cfg(test)]
mod tests
{
	use crate::parsing::tokenization::{OperatorToken, Token, TokenStream};
	use crate::parsing::{self, Associativity, Node, Operator, Priority, Valency};

	#[test]
	fn bsaic_tokenization_test()
	{
		let mut stream = TokenStream::new("4>=5");
		assert_eq!(stream.next().unwrap().unwrap(), Token::Number(4.0));
		assert_eq!(
			stream.next().unwrap().unwrap(),
			Token::Operator(OperatorToken::GreaterOrEqual)
		);
		assert_eq!(stream.next().unwrap().unwrap(), Token::Number(5.0));
		assert!(stream.next().is_none());
	}

	#[test]
	fn tokenization_test()
	{
		let mut stream = TokenStream::new("2+7*3");
		assert_eq!(stream.next().unwrap().unwrap(), Token::Number(2.0));
		assert_eq!(
			stream.next().unwrap().unwrap(),
			Token::Operator(OperatorToken::Plus)
		);
		assert_eq!(stream.next().unwrap().unwrap(), Token::Number(7.0));
		assert_eq!(
			stream.next().unwrap().unwrap(),
			Token::Operator(OperatorToken::Multiply)
		);
		assert_eq!(stream.next().unwrap().unwrap(), Token::Number(3.0));
	}

	#[test]
	fn basic_parse_queue_test()
	{
		let output = parsing::rpn_queue_from("2+7*3").unwrap();

		assert_eq!(output[0], Node::Number(2.0));
		assert_eq!(output[1], Node::Number(7.0));
		assert_eq!(output[2], Node::Number(3.0));
		assert_eq!(
			output[3],
			Node::Operator(Operator {
				priority: Priority::MULTIPLICITIVE,
				valency: Valency::Binary,
				associativity: Associativity::Left,
			})
		);
		assert_eq!(
			output[4],
			Node::Operator(Operator {
				priority: Priority::ADDITIVE,
				valency: Valency::Binary,
				associativity: Associativity::Left,
			})
		);
	}

	#[test]
	fn whitespace_test()
	{
		let stream_no_whitespace = TokenStream::new("17+9-3")
			.map(|t| t.unwrap())
			.collect::<Vec<Token>>();

		let stream_whitespace = TokenStream::new("17 + 9 - 3")
			.map(|t| t.unwrap())
			.collect::<Vec<Token>>();

		assert_eq!(stream_no_whitespace.len(), stream_whitespace.len());
		for token_pair in stream_no_whitespace.iter().zip(stream_whitespace.iter())
		{
			assert_eq!(token_pair.0, token_pair.1);
		}
	}
}
