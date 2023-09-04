mod parsing;

#[cfg(test)]
mod tests
{
	use crate::parsing::tokenization::{OperatorToken, Token, TokenStream};

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
}
