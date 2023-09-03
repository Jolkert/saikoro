mod parsing;

#[cfg(test)]
mod tests
{
	use crate::parsing::tokenization::{Operator, Token, TokenStream};

	#[test]
	fn bsaic_tokenization_test()
	{
		let mut stream = TokenStream::new(String::from("4>=5"));
		assert_eq!(stream.next().unwrap(), Token::Number(4.0));
		assert_eq!(
			stream.next().unwrap(),
			Token::Operator(Operator::GreaterOrEqual)
		);
		assert_eq!(stream.next().unwrap(), Token::Number(5.0));
		assert!(stream.next().is_none());
	}
}
