pub mod evaluation;
pub mod parsing;
pub use evaluation::{eval_with_random, evaluate};

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
pub(crate) mod test_helpers
{
	use crate::{evaluation::eval_with_random, RangeRng};
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
		let rolls = eval_with_random("3d6", &mut RiggedRandom::new([3, 5, 2])).unwrap();

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
