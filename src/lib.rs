mod parsing;

use num_rational::{Ratio, Rational64 as r64};

pub fn r64_from_f64(f: f64) -> Option<r64>
{
	if let Some(r) = Ratio::from_float(f)
	{
		if let (Ok(num), Ok(denom)) = (r.numer().try_into(), r.denom().try_into())
		{
			return Some(r64::new(num, denom));
		}
	}

	None
}

#[cfg(test)]
mod tests
{
	use crate::parsing::tokenization::{OperatorToken, Token, TokenStream};
	use crate::parsing::{self, Associativity, Node, Operator, Priority, Valency};

	use num_rational::Rational64 as r64;

	#[test]
	fn bsaic_tokenization_test()
	{
		let mut stream = TokenStream::new("4>=5");
		assert_eq!(
			stream.next().unwrap().unwrap(),
			Token::Number(r64::from_integer(4))
		);
		assert_eq!(
			stream.next().unwrap().unwrap(),
			Token::Operator(OperatorToken::GreaterOrEqual)
		);
		assert_eq!(
			stream.next().unwrap().unwrap(),
			Token::Number(r64::from_integer(5))
		);
		assert!(stream.next().is_none());
	}

	#[test]
	fn tokenization_test()
	{
		let mut stream = TokenStream::new("2+7*3");
		assert_eq!(
			stream.next().unwrap().unwrap(),
			Token::Number(r64::from_integer(2))
		);
		assert_eq!(
			stream.next().unwrap().unwrap(),
			Token::Operator(OperatorToken::Plus)
		);
		assert_eq!(
			stream.next().unwrap().unwrap(),
			Token::Number(r64::from_integer(7))
		);
		assert_eq!(
			stream.next().unwrap().unwrap(),
			Token::Operator(OperatorToken::Multiply)
		);
		assert_eq!(
			stream.next().unwrap().unwrap(),
			Token::Number(r64::from_integer(3))
		);
	}

	#[test]
	fn basic_parse_queue_test()
	{
		let output = parsing::rpn_queue_from("2+7*3").unwrap();

		assert_eq!(output[0], Node::Number(r64::from_integer(2)));
		assert_eq!(output[1], Node::Number(r64::from_integer(7)));
		assert_eq!(output[2], Node::Number(r64::from_integer(3)));
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
