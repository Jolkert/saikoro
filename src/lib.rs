#![feature(let_chains)]

pub mod evaluation;
pub mod parsing;

use rand::{Rng, RngCore};
use std::ops::Range;

pub trait RangeRng
{
	fn rng_range(&mut self, range: Range<u32>) -> u32;
}

impl<T: RngCore> RangeRng for T
{
	fn rng_range(&mut self, range: Range<u32>) -> u32
	{
		self.gen_range(range)
	}
}

#[cfg(test)]
mod tests
{
	use crate::evaluation::{self, DiceEvaluation, Operand, Roll};
	use crate::parsing::tokenization::{OpToken, Token, TokenStream};
	use crate::parsing::{self, BinaryOperator, Node};
	use crate::RangeRng;
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
	fn whitespace_test()
	{
		let without_whitespace = token_vec_from("17+9-3");
		let with_whitespace = token_vec_from("17 + 9 - 3");

		assert_eq!(without_whitespace, with_whitespace);
	}

	#[test]
	fn eval_fn_test()
	{
		let plus = BinaryOperator::from(OpToken::Plus);
		let result = plus.eval(
			Operand::Number(2.0),
			Operand::Number(5.0),
			&mut rand::thread_rng(),
		);

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
		assert!(eval_unwrap("2+3*5").value.approx_eq(17.0));

		assert!(eval_unwrap("(2+3)*5").value.approx_eq(25.0));

		assert!(eval_unwrap_with("2d6 + 5", &mut RiggedRandom::new([4, 2]))
			.value
			.approx_eq(11.0));

		assert!(eval_unwrap("2^3^3").value.approx_eq(134_217_728.0));

		assert!(eval_unwrap("5 - 4 - 2").value.approx_eq(-1.0));

		assert!(eval_unwrap("10d4")
			.ungrouped_rolls()
			.all(|roll| (1..=4).contains(&roll.original_value)));

		assert!(eval_unwrap("2d6 + 3d6 * 6d6")
			.ungrouped_rolls()
			.all(|roll| (1..=6).contains(&roll.original_value)));
	}

	#[test]
	fn comparison_operator_test()
	{
		let rolls = eval_unwrap_with("5d6 > 3", &mut RiggedRandom::new([3, 4, 6, 1, 1]))
			.ungrouped_rolls()
			.filter_map(Roll::value)
			.collect::<Vec<_>>();

		assert_eq!(rolls, vec![4, 6]);
	}

	#[test]
	fn rigged_random_test()
	{
		let rolls = eval_unwrap_with("3d6", &mut RiggedRandom::new([3, 5, 2]))
			.ungrouped_rolls()
			.map(|it| it.original_value)
			.collect::<Vec<_>>();

		assert_eq!(rolls, vec![3, 5, 2]);
	}

	#[test]
	fn tree_test()
	{
		use Node::{Binary, Leaf};

		let actual = parsing::parse_tree_from("2d6 > 3 + 8d6 * 3^4 / 2 - 3 + 1").unwrap();
		let expected = Binary {
			operator: OpToken::Plus.into(),
			left: Box::new(Binary {
				operator: OpToken::Minus.into(),
				left: Box::new(Binary {
					operator: OpToken::Plus.into(),
					left: Box::new(Binary {
						operator: OpToken::GreaterThan.into(),
						left: Box::new(Binary {
							operator: OpToken::Dice.into(),
							left: Box::new(Leaf(2.0)),
							right: Box::new(Leaf(6.0)),
						}),
						right: Box::new(Leaf(3.0)),
					}),
					right: Box::new(Binary {
						operator: OpToken::Divide.into(),
						left: Box::new(Binary {
							operator: OpToken::Multiply.into(),
							left: Box::new(Binary {
								operator: OpToken::Dice.into(),
								left: Box::new(Leaf(8.0)),
								right: Box::new(Leaf(6.0)),
							}),
							right: Box::new(Binary {
								operator: OpToken::Power.into(),
								left: Box::new(Leaf(3.0)),
								right: Box::new(Leaf(4.0)),
							}),
						}),
						right: Box::new(Leaf(2.0)),
					}),
				}),
				right: Box::new(Leaf(3.0)),
			}),
			right: Box::new(Leaf(1.0)),
		};
		assert_eq!(expected, actual);
	}

	#[test]
	pub fn unary_test()
	{
		let actual = parsing::parse_tree_from("-4+5").unwrap();
		let expected = Node::Binary {
			operator: OpToken::Plus.into(),
			left: Box::new(Node::Unary {
				operator: OpToken::Minus.try_into().unwrap(),
				argument: Box::new(Node::Leaf(4.0)),
			}),
			right: Box::new(Node::Leaf(5.0)),
		};
		assert_eq!(expected, actual);

		let actual = parsing::parse_tree_from("-2^5").unwrap();
		let expected = Node::Unary {
			operator: OpToken::Minus.try_into().unwrap(),
			argument: Box::new(Node::Binary {
				operator: OpToken::Power.into(),
				left: Box::new(Node::Leaf(2.0)),
				right: Box::new(Node::Leaf(5.0)),
			}),
		};
		assert_eq!(expected, actual);
	}

	#[test]
	pub fn tree_eval_test()
	{
		let actual = evaluation::evaluate("2 + 5 * 7").unwrap();
		let expected = 37.0;
		assert!((expected - actual.value).abs() < f64::EPSILON);
	}

	fn eval_unwrap(input: &str) -> DiceEvaluation
	{
		evaluation::evaluate(input).unwrap()
	}

	fn eval_unwrap_with<R: RangeRng>(input: &str, random: &mut R) -> DiceEvaluation
	{
		evaluation::eval_with_random(input, random).unwrap()
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

		fn pop(&mut self) -> u32
		{
			self.roll_queue.pop_front().expect("roll queue empty!")
		}
	}
	impl RangeRng for RiggedRandom
	{
		fn rng_range(&mut self, range: std::ops::Range<u32>) -> u32
		{
			self.pop().clamp(range.start, range.end - 1)
		}
	}
}
