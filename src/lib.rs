// TODO: please turn this off before you ever consider this even close to finished because that
// would be incredibly stupid of you. im just sick of seeing 30+ warnings when im still not done
// implementing everything >:( -morgan 20203-09-17
#![allow(dead_code)]
mod evaluation;
mod parsing;

#[cfg(test)]
mod tests
{
	use crate::evaluation::Item;
	use crate::parsing::tokenization::{OperatorToken, Token, TokenStream};
	use crate::parsing::{self, Node, Operator, Valency};

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
			Node::Operator(Operator::from_token(
				OperatorToken::Multiply,
				Valency::Binary
			))
		);
		assert_eq!(
			output[4],
			Node::Operator(Operator::from_token(OperatorToken::Plus, Valency::Binary))
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

	#[test]
	fn eval_fn_test()
	{
		let plus = Operator::from_token(OperatorToken::Plus, Valency::Binary);
		let mut item_stack = vec![Item::Number(2.0), Item::Number(5.0)];
		let result = plus.eval(&mut item_stack);

		match result
		{
			Ok(i) => assert_eq!(i, Item::Number(7.0)),
			Err(_) => panic!(),
		}
	}
}
