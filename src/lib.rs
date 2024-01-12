//! A parser and evaluator for dice notation expression
//! # Basic Usage example
//! ```rust
//! # fn main() -> Result<(), saikoro::error::EvaluationError> {
//! // roll for fireball damage
//! let damage = saikoro::evaluate("8d6")?;
//! println!("Fireball deals {} fire damage", damage.value);
//! # Ok(())}
//! ```

pub mod error;
pub mod evaluation;
pub mod operator;
mod parsing;
mod tokenization;

use error::EvaluationError;
use evaluation::DiceEvaluation;
use rand::{Rng, RngCore, SeedableRng};
use std::ops::Range;
use tokenization::TokenStream;

/// Evaluates a string in format similar to [Standard Dice Notation](https://en.wikipedia.org/wiki/Dice_notation)
/// evaluated with [`rand::thread_rng`]. Equivalent to [`eval_with_rand`] called with `&mut rand::thread_rng()`
/// as the second parameter
/// # Examples
/// ```rust
/// # fn main() -> Result<(), saikoro::error::EvaluationError> {
/// let evaluation = saikoro::evaluate("2d6")?;
/// let final_value = evaluation.value;
/// // the result of rolling 2d6 will be between 2 and 12
/// assert!(final_value >= 2.0 && final_value <= 12.0);
/// # Ok(())
/// # }
/// ```
/// # Errors
/// An error variant will be returned if the expression is unable to be parsed, or the evaluation function
/// produces an error
pub fn evaluate(input: &str) -> Result<DiceEvaluation, EvaluationError>
{
	eval_with_rand(input, &mut rand::thread_rng())
}

/// A utility wrapper function for seeding a dice roll with the given u64 as the seed
/// (see [`saikoro::eval_with_rand`][`eval_with_rand`] for more information)
pub fn eval_with_seed(input: &str, seed: u64) -> Result<DiceEvaluation, EvaluationError>
{
	let mut seeded_random = rand::rngs::StdRng::seed_from_u64(seed);
	eval_with_rand(input, &mut seeded_random)
}

/// Evaluates a string in format similar to [Standard Dice Notation](https://en.wikipedia.org/wiki/Dice_notation)
/// evaluated with the given [`RangeRng`]
/// # Examples
/// ```rust
/// # fn main() -> Result<(), saikoro::error::EvaluationError>
/// # {
/// use rand::{rngs::StdRng, SeedableRng};
///
/// // this seed will generate a 4 and a 5 from the first two rolls
/// let mut seeded = StdRng::seed_from_u64(2024);
/// let evaluation = saikoro::eval_with_rand("2d6", &mut seeded)?;
/// assert_eq!(evaluation.value, 9.0);
/// # Ok(())
/// # }
/// ```
/// # Errors
/// An error variant will be returned if the expression is unable to be parsed, or the evaluation function
/// produces an error
/// #
/// # Rng Notes
/// Importantly, the evaluation function for dice rolls will generate a number in the range [0, faces)
/// (note exclusive upper bound) and add 1 to the value, so if the [`RangeRng::rng_range`] function will
/// always return the same value regardless of provided range, (eg. for testing purposes) the value produced
/// from dice rolls may be 1 more than expected.
/// # See Also
/// For simply seeding a roll with a u64 seed, see [`saikoro::eval_with_seed`][`eval_with_seed`]
/// [`saikoro::RangeRng`][`RangeRng`]
pub fn eval_with_rand<R>(input: &str, rand: &mut R) -> Result<DiceEvaluation, EvaluationError>
where
	R: RangeRng,
{
	evaluation::evaluate_tree(
		parsing::parse_tree_from(&mut TokenStream::new(input))?,
		rand,
	)
}

/// A utility trait for allowing flexibility for testing or rigging saikoro's random number generation.
/// All implementers of [`rand::RngCore`] (i.e. all RNGs from the [`rand`] therefore ones one is likely
/// to use) get an implementation of this trait for free, so most will not need to implement this trait,
/// but it is availale publicly for those who do
/// # Examples
/// ```rust
/// # use saikoro::RangeRng;
/// use std::collections::VecDeque;
/// # use std::ops::Range;
///
/// struct RiggedRng
/// {
///     // we store the rigged numbers in a queue
///     roll_queue: VecDeque<u32>
/// }
/// impl RangeRng for RiggedRng
/// {
///     // "generating a random number" is just popping the next number from the queue
///     fn rng_range(&mut self, range: Range<u32>) -> u32
///     {
///         self.roll_queue.pop_front().expect("Queue was empty!")
///     }
/// }
///
/// fn main()
/// {
///     // the first three numbers produced from the generator should be 4, 1, 2
///     let mut rigged_rng = RiggedRng {
///         roll_queue: VecDeque::from([4, 1, 2])
///     };
///     assert_eq!(rigged_rng.rng_range(0..6), 4);
///     assert_eq!(rigged_rng.rng_range(0..6), 1);
///     assert_eq!(rigged_rng.rng_range(0..6), 2);
/// }
/// ```
/// # Note to Implementers
/// In practice, when used by [`saikoro::eval_with_rand`][`crate::eval_with_rand`],
/// the [`rng_range`][`RangeRng::rng_range`] function will generate a number in the range [0, faces)
/// (note exclusive upper bound) and add 1 to the value, so if the [`rng_range`][`RangeRng::rng_range`]
/// function will always return the same value regardless of provided range, (eg. for testing purposes)
/// the value produced from dice rolls may be 1 more than expected.
pub trait RangeRng
{
	/// Generates a random number within the bounds of the [`Range`]
	fn rng_range(&mut self, range: Range<u32>) -> u32;
}
impl<T: RngCore> RangeRng for T
{
	#[doc(hidden)]
	fn rng_range(&mut self, range: Range<u32>) -> u32
	{
		self.gen_range(range)
	}
}

#[cfg(test)]
pub(crate) mod test_helpers
{
	use crate::{eval_with_rand, RangeRng};
	use std::collections::VecDeque;

	pub struct RiggedRandom
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

	pub fn flip_result<T, E>(result: Result<T, E>) -> Result<E, T>
	{
		match result
		{
			Ok(ok) => Err(ok),
			Err(err) => Ok(err),
		}
	}

	macro_rules! assert_approx_eq {
		($left: expr, $right: expr) => {
			match (&$left, &$right)
			{
				(left_val, right_val) =>
				{
					if !(f64::abs(*left_val - *right_val) < f64::EPSILON)
					{
						std::panic!("assertion that `left` approx equals `right` failed\nleft: {}\nright: {}", &*left_val, &*right_val);
					}
				}
			}
		};
	}
	pub(crate) use assert_approx_eq;

	#[test]
	fn rigged_random_test()
	{
		let rolls = eval_with_rand("3d6", &mut RiggedRandom::new([3, 5, 2])).unwrap();

		assert_approx_eq!(rolls.value, 10.0);
		assert_eq!(
			vec![3, 5, 2],
			rolls
				.ungrouped_rolls()
				.map(|it| it.original_value)
				.collect::<Vec<_>>()
		);
	}
}
