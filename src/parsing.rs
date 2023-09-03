pub mod tokenization
{
	use lazy_static::lazy_static;
	use maplit::hashmap;
	use regex::Regex;
	use std::collections::HashMap;

	#[derive(Debug, PartialEq)]
	pub enum Token
	{
		Number(f64),
		Operator(Operator),
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

	#[derive(Debug, PartialEq, Eq)]
	pub enum Operator
	{
		Plus,
		Minus,
		Multiply,
		Divide,
		Modulus,
		Power,
		Dice,
		Equals,
		NotEquals,
		GreaterThan,
		LessThan,
		GreaterOrEqual,
		LessOrEqual,
	}
	impl Operator
	{
		fn from(str: &str) -> Option<Operator>
		{
			match str {
				"+" => Some(Self::Plus),
				"-" => Some(Self::Minus),
				"*" => Some(Self::Multiply),
				"/" => Some(Self::Divide),
				"%" => Some(Self::Modulus),
				"^" => Some(Self::Power),
				"d" | "D" => Some(Self::Dice),
				"==" => Some(Self::Equals),
				"!=" => Some(Self::NotEquals),
				">" => Some(Self::GreaterThan),
				"<" => Some(Self::LessThan),
				">=" => Some(Self::GreaterOrEqual),
				"<=" => Some(Self::LessOrEqual),
				_ => None,
			}
		}
	}

	pub struct TokenStream
	{
		string: String,
		current_index: usize,
	}
	impl TokenStream
	{
		pub fn new(string: String) -> TokenStream
		{
			TokenStream {
				string,
				current_index: 0,
			}
		}
	}
	lazy_static! {
		static ref REGEX_MAP: HashMap<TokenType, Regex> = hashmap! {
			TokenType::Number => Regex::new(r"\d+(\.\d+)?").unwrap(),
			TokenType::Operator => Regex::new(r"[\+\-\*\/%^dD]|==|!=|>=|<=|>|<").unwrap(),
			TokenType::Delimiter => Regex::new(r"[\(\)]").unwrap()
		};
	}
	impl Iterator for TokenStream
	{
		type Item = Token;
		fn next(&mut self) -> Option<Self::Item>
		{
			if self.current_index >= self.string.len() {
				return None;
			}

			for pair in REGEX_MAP.iter() {
				if let Some(mtch) = pair.1.find_at(&self.string, self.current_index) {
					if mtch.start() != self.current_index {
						continue;
					}

					self.current_index += mtch.as_str().len();

					// TODO: make this nicer
					return match pair.0 {
						TokenType::Number => {
							Some(Token::Number(mtch.as_str().parse::<f64>().unwrap()))
						}
						TokenType::Operator => {
							Some(Token::Operator(Operator::from(mtch.as_str()).unwrap()))
						}
						TokenType::Delimiter => Some(Token::Delimiter {
							is_open: mtch.as_str() == "(",
						}),
					};
				}
			}

			None
		}
	}
}
