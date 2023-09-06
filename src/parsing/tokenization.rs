pub use super::operators::OperatorToken;

use lazy_static::lazy_static;
use maplit::hashmap;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
	static ref REGEX_MAP: HashMap<TokenType, Regex> = hashmap! {
		TokenType::Number => Regex::new(r"\d+(\.\d+)?").unwrap(),
		TokenType::Operator => Regex::new(r"[\+\-\*\/%^dD]|==|!=|>=|<=|>|<").unwrap(),
		TokenType::Delimiter => Regex::new(r"[\(\)]").unwrap()
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
	type Item = Token;
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
					TokenType::Number => Some(Token::Number(mtch.as_str().parse::<f64>().unwrap())),
					TokenType::Operator => Some(Token::Operator(
						mtch.as_str().parse::<OperatorToken>().unwrap(),
					)),
					TokenType::Delimiter => Some(Token::Delimiter {
						is_open: mtch.as_str() == "(",
					}),
				};
			}
		}

		None
	}
}
