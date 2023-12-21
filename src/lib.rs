// TODO: please turn this off before you ever consider this even close to finished because that
// would be incredibly stupid of you. im just sick of seeing 30+ warnings when im still not done
// implementing everything >:( -morgan 2023-09-17
#![allow(dead_code)]
#![feature(let_chains)]

pub mod error;
pub mod evaluation;
pub mod parsing;

use std::ops::Range;

pub use error::Error;
use rand::{
	rngs::{StdRng, ThreadRng},
	Rng,
};

pub trait SaikoroRandom
{
	fn rng_range(&mut self, range: Range<u32>) -> u32;
}

impl SaikoroRandom for ThreadRng
{
	fn rng_range(&mut self, range: Range<u32>) -> u32
	{
		self.gen_range(range)
	}
}

impl SaikoroRandom for StdRng
{
	fn rng_range(&mut self, range: Range<u32>) -> u32
	{
		self.gen_range(range)
	}
}

#[cfg(test)]
mod tests
{
	use crate::evaluation::{self, Operand, Roll};
	use crate::parsing::tokenization::{OpToken, Token, TokenStream};
	use crate::parsing::{self, Node, Operator, Valency};
	use crate::SaikoroRandom;
	use std::collections::VecDeque;

	#[test]
	fn tokenization_test()
	{
		let actual_tokens = token_vec_from("4>=5");
		let expected = vec![
			Token::from(4.0),
			Token::from(OpToken::GreaterOrEqual),
			Token::from(5.0),
		];

		assert_eq!(actual_tokens, expected);

		let actual_tokens = token_vec_from("2+7*3");
		let expected = vec![
			Token::from(2.0),
			Token::from(OpToken::Plus),
			Token::from(7.0),
			Token::from(OpToken::Multiply),
			Token::from(3.0),
		];

		assert_eq!(actual_tokens, expected);
	}

	#[test]
	fn basic_parse_queue_test()
	{
		let output = parsing::rpn_queue_from("2+7*3").unwrap();
		let expected: VecDeque<Node> = vec![
			Node::from(2.0),
			Node::from(7.0),
			Node::from(3.0),
			Node::from(Operator::from_token(OpToken::Multiply, Valency::Binary)),
			Node::from(Operator::from_token(OpToken::Plus, Valency::Binary)),
		]
		.into();

		assert_eq!(output, expected);
	}

	#[test]
	fn whitespace_test()
	{
		let without_whitespace = token_vec_from("17+9-3");
		let with_whitespace = token_vec_from("17 + 9 - 3");

		assert_eq!(without_whitespace, with_whitespace);
	}

	#[test]
	fn eval_fn_test()
	{
		let plus = Operator::from_token(OpToken::Plus, Valency::Binary);
		let mut item_stack = vec![Operand::Number(2.0), Operand::Number(5.0)];
		let result = plus.eval(&mut item_stack, &mut rand::thread_rng());

		assert_eq!(result.unwrap(), Operand::Number(7.0));
	}

	#[test]
	fn roll_type_test()
	{
		let mut roll = Roll::new(3);
		assert!(!roll.is_removed());

		roll = roll.remove_unless(|it| it.original_value < 2);
		assert!(roll.is_removed());
	}

	#[test]
	fn basic_eval_test()
	{
		let result = evaluation::eval_string("2+3*5").unwrap();
		assert!(result.value.approx_eq(17.0));

		let result =
			evaluation::eval_with_random("2d6 + 5", &mut RiggedRandom::new([4, 2])).unwrap();
		assert!(result.value.approx_eq(11.0));

		let result = evaluation::eval_string("2^3^3").unwrap();
		assert!(result.value.approx_eq(134_217_728.0));

		let result = evaluation::eval_string("5 - 4 - 2").unwrap();
		assert!(result.value.approx_eq(-1.0));

		let result = evaluation::eval_string("10d4");
		// 	assert each individual roll in range [1, 4]. not confusing looking at all i promise -morgan 2023-09-27
		assert!(
			result.is_ok_and(|it| it.rolls.iter().all(|it| it
				.iter()
				.all(|it| it.original_value >= 1 && it.original_value <= 4)))
		);

		let result = evaluation::eval_string("2d6 + 3d6 * 6d6");
		// assert 3 DiceRolls & all in range [1, 6]. again, very intuitive test code i promise -morgan 2023-09-27
		assert!(result.is_ok_and(|it| it.rolls.len() == 3
			&& it.rolls.iter().all(|it| it
				.iter()
				.all(|it| it.original_value >= 1 && it.original_value <= 6))));
	}

	#[test]
	fn comparison_operator_test()
	{
		let mut rigged = RiggedRandom::new([3, 4, 6, 1, 1]);
		let result = evaluation::eval_with_random("5d6 > 3", &mut rigged).unwrap();
		let rolls = result
			.rolls
			.first()
			.unwrap()
			.rolls
			.iter()
			.filter_map(Roll::value)
			.collect::<Vec<_>>();

		assert_eq!(rolls, vec![4, 6]);
	}

	#[test]
	fn rigged_random_test()
	{
		let mut rigged = RiggedRandom::new([3, 5, 2]);
		let result = evaluation::eval_with_random("3d6", &mut rigged).unwrap();
		let rolls = result
			.rolls
			.first()
			.unwrap()
			.rolls
			.iter()
			.map(|it| it.original_value)
			.collect::<Vec<_>>();

		assert_eq!(rolls, vec![3, 5, 2]);
	}

	fn token_vec_from(string: &str) -> Vec<Token>
	{
		TokenStream::new(string).map(Result::unwrap).collect()
	}

	trait ApproxEq
	{
		fn approx_eq(self, rhs: Self) -> bool;
	}
	impl ApproxEq for f64
	{
		fn approx_eq(self, rhs: Self) -> bool
		{
			Self::abs(self - rhs) < Self::EPSILON
		}
	}

	struct RiggedRandom
	{
		pub roll_queue: VecDeque<u32>,
	}
	impl RiggedRandom
	{
		pub fn new<I>(values: I) -> Self
		where
			I: IntoIterator<Item = u32>,
		{
			Self {
				roll_queue: values.into_iter().map(|it| it.saturating_sub(1)).collect(),
			}
		}

		pub fn push(&mut self, value: u32)
		{
			self.roll_queue.push_back(value.saturating_sub(1));
		}
		pub fn push_all<I>(&mut self, values: I)
		where
			I: Iterator<Item = u32>,
		{
			for value in values
			{
				self.push(value);
			}
		}

		fn pop(&mut self) -> u32
		{
			self.roll_queue.pop_front().expect("roll queue empty!")
		}
	}
	impl SaikoroRandom for RiggedRandom
	{
		fn rng_range(&mut self, range: std::ops::Range<u32>) -> u32
		{
			self.pop().clamp(range.start, range.end - 1)
		}
	}
}
