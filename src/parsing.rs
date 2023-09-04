use std::collections::VecDeque;
use std::ops;
use tokenization::{OperatorToken, Token, TokenStream};

// TODO: better name -morgan 2023-09-03
#[derive(Debug)]
pub enum Node
{
	Number(f64),
	Operator(Operator),
}
#[derive(Debug)]
pub struct Operator
{
	pub priority: u16,
	pub valency: u8,
	pub associativity: Associativity,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Associativity
{
	Left = 0,
	Right = 1,
}
impl ops::Add<Associativity> for u16
{
	type Output = u16;
	fn add(self, rhs: Associativity) -> Self::Output
	{
		self + if rhs == Associativity::Left { 0 } else { 1 }
	}
}

enum OpOrDelim
{
	Operator(Operator),
	Delimiter
	{
		is_open: bool,
	},
}
impl OpOrDelim
{
	fn is_operator(&self) -> bool
	{
		match self
		{
			Self::Operator(_) => true,
			_ => false,
		}
	}
}

pub fn rpn_queue_from(string: &str) -> VecDeque<Node>
{
	let stream = TokenStream::new(string);
	let mut output_queue: VecDeque<Node> = VecDeque::new();
	let mut operator_stack: Vec<OpOrDelim> = Vec::new();

	let mut previous: Option<Token> = None;
	for token in stream
	{
		match token
		{
			Token::Number(num) =>
			{
				output_queue.push_back(Node::Number(num));
			}
			Token::Operator(ref op_token) =>
			{
				// this is a *complete* mess -morgan 2023-09-04
				let operator = Operator {
					valency: match previous
					{
						None | Some(Token::Operator(_)) => 1,
						_ => 2,
					},
					priority: op_token.priority(),
					associativity: if *op_token == OperatorToken::Power
					{
						Associativity::Right
					}
					else
					{
						Associativity::Left
					},
				};
				push_operator_to_stack(operator, &mut operator_stack, &mut output_queue);
			}
			Token::Delimiter { is_open } =>
			{
				if is_open
				{
					match previous
					{
						Some(Token::Number(_)) | Some(Token::Delimiter { is_open: true }) =>
						{
							push_operator_to_stack(
								Operator {
									priority: OperatorToken::Multiply.priority(),
									valency: 2,
									associativity: Associativity::Left,
								},
								&mut operator_stack,
								&mut output_queue,
							)
						}
						_ => (),
					}
				}
				else
				{
					loop
					{
						match operator_stack.pop()
						{
							Some(OpOrDelim::Operator(op)) =>
							{
								output_queue.push_back(Node::Operator(op))
							}
							_ => break,
						}
					}
				}
			}
		}

		previous = Some(token);
	}

	while let Some(OpOrDelim::Operator(op)) = operator_stack.pop()
	{
		print!("op!");
		output_queue.push_back(Node::Operator(op));
	}

	output_queue
}
fn push_operator_to_stack(
	operator: Operator,
	stack: &mut Vec<OpOrDelim>,
	output_queue: &mut VecDeque<Node>,
)
{
	while let Some(front) = stack.last()
	{
		match front
		{
			OpOrDelim::Delimiter { is_open: _ } => break,
			OpOrDelim::Operator(last_op) =>
			{
				if last_op.valency <= operator.valency
					&& last_op.priority >= (operator.priority + operator.associativity)
				{
					if let Some(OpOrDelim::Operator(it)) = stack.pop()
					{
						output_queue.push_back(Node::Operator(it));
					}
				}
				else
				{
					break;
				}
			}
		}
	}
	stack.push(OpOrDelim::Operator(operator));
}

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

	#[derive(Debug, PartialEq, Eq)]
	pub enum OperatorToken
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
	impl OperatorToken
	{
		fn from(str: &str) -> Option<OperatorToken>
		{
			match str
			{
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

		pub fn priority(&self) -> u16
		{
			match self
			{
				Self::Plus | Self::Minus => 0,
				Self::Multiply | Self::Divide | Self::Modulus => 1,
				Self::Power => 2,
				Self::Dice => 3,
				Self::Equals
				| Self::NotEquals
				| Self::GreaterThan
				| Self::LessThan
				| Self::GreaterOrEqual
				| Self::LessOrEqual => 4,
			}
		}
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
	lazy_static! {
		static ref REGEX_MAP: HashMap<TokenType, Regex> = hashmap! {
			TokenType::Number => Regex::new(r"\d+(\.\d+)?").unwrap(),
			TokenType::Operator => Regex::new(r"[\+\-\*\/%^dD]|==|!=|>=|<=|>|<").unwrap(),
			TokenType::Delimiter => Regex::new(r"[\(\)]").unwrap()
		};
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
				if let Some(mtch) = pair.1.find_at(&self.string, self.current_index)
				{
					if mtch.start() != self.current_index
					{
						continue;
					}

					self.current_index += mtch.as_str().len();

					// TODO: make this nicer
					return match pair.0
					{
						TokenType::Number =>
						{
							Some(Token::Number(mtch.as_str().parse::<f64>().unwrap()))
						}
						TokenType::Operator =>
						{
							Some(Token::Operator(OperatorToken::from(mtch.as_str()).unwrap()))
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
