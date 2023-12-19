use crate::Error;

pub use super::operators::OperatorToken;

use lazy_static::lazy_static;
use maplit::hashmap;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
	static ref REGEX_MAP: HashMap<TokenType, Regex> = hashmap! {
		TokenType::Number => Regex::new(r"\d+(\.\d+)?").unwrap(),
		TokenType::Operator => Regex::new(r"[\+\-\*\/%^dD]|==|!=|>=|<=|>|<").unwrap(),
		TokenType::Delimiter => Regex::new(r"[\(\)]").unwrap(),
		TokenType::Whitespace => Regex::new(r"\s+").unwrap()
	};
}

#[derive(Debug, PartialEq)]
pub enum Token
{
	Number(f64),
	Operator(OperatorToken),
	Delimiter
	{
		is_open: bool,
	},
}

#[derive(PartialEq, Eq, Hash)]
enum TokenType
{
	// TODO: find a better way -morgan 2023-09-02
	Number,
	Operator,
	Delimiter,
	Whitespace,
}

pub struct TokenStream<'a>
{
	string: &'a str,
	current_index: usize,
}

impl<'a> TokenStream<'a>
{
	pub fn new(string: &'a str) -> Self
	{
		TokenStream {
			string,
			current_index: 0,
		}
	}
}
impl<'a> Iterator for TokenStream<'a>
{
	type Item = Result<Token, Error>;
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.current_index >= self.string.len()
		{
			return None;
		}

		for pair in REGEX_MAP.iter()
		{
			if let Some(mtch) = pair.1.find_at(self.string, self.current_index)
			{
				if mtch.start() != self.current_index
				{
					continue;
				}

				self.current_index += mtch.as_str().len();

				// TODO: make this nicer
				return match pair.0
				{
					TokenType::Number => Some(
						mtch.as_str()
							.parse::<f64>()
							.map_or_else(|_| Err(Error::InvalidToken), |n| Ok(Token::Number(n))),
					),
					TokenType::Operator =>
					{
						Some(mtch.as_str().parse::<OperatorToken>().map_or_else(
							|_| Err(Error::InvalidToken),
							|op| Ok(Token::Operator(op)),
						))
					}
					TokenType::Delimiter => Some(Ok(Token::Delimiter {
						is_open: mtch.as_str() == "(",
					})),
					TokenType::Whitespace => self.next(),
				};
			}
		}

		None
	}
}
