use crate::evaluation::Roll;

use super::{Item, RollSet};
use crate::Error;
use rand::prelude::*;

type EvalResult = Result<Item, Error>;

pub fn unary_plus(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some(i) = stack.pop()
	{
		Ok(i)
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 1,
			found: 0,
		})
	}
}

pub fn unary_minus(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some(i) = stack.pop()
	{
		Ok(-i)
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 1,
			found: 0,
		})
	}
}

pub fn add(stack: &mut Vec<Item>) -> EvalResult
{
	simple_binary_operation(stack, |lhs, rhs| lhs + rhs)
}

pub fn subtract(stack: &mut Vec<Item>) -> EvalResult
{
	simple_binary_operation(stack, |lhs, rhs| lhs - rhs)
}

pub fn multiply(stack: &mut Vec<Item>) -> EvalResult
{
	simple_binary_operation(stack, |lhs, rhs| lhs * rhs)
}

pub fn divide(stack: &mut Vec<Item>) -> EvalResult
{
	simple_binary_operation(stack, |lhs, rhs| lhs * rhs)
}

pub fn modulo(stack: &mut Vec<Item>) -> EvalResult
{
	simple_binary_operation(stack, |lhs, rhs| lhs % rhs)
}

pub fn pow(stack: &mut Vec<Item>) -> EvalResult
{
	simple_binary_operation(stack, |lhs, rhs| {
		Item::Number(lhs.value().powf(rhs.value()))
	})
}

pub fn roll(stack: &mut Vec<Item>) -> EvalResult
{
	match double_pop(stack)
	{
		Ok((rhs, lhs)) =>
		{
			let mut roll_vec = Vec::<Roll>::new();
			let faces = rhs.value() as u64;

			for _ in 0..(lhs.value() as u64)
			{
				roll_vec.push(Roll {
					value: rand::thread_rng().gen_range(0..faces + 1),
					faces,
					removed: false,
				});
			}

			Ok(Item::Roll(RollSet(roll_vec)))
		}
		Err(Reason::Empty) => Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		}),
		Err(Reason::One) => Err(Error::MissingOperand {
			expected: 2,
			found: 1,
		}),
	}
}

pub fn equal(stack: &mut Vec<Item>) -> EvalResult
{
	filter_condition(stack, |lhs, rhs| (lhs.value as f64) == rhs.value())
}

pub fn not_equal(stack: &mut Vec<Item>) -> EvalResult
{
	filter_condition(stack, |lhs, rhs| (lhs.value as f64) != rhs.value())
}

pub fn greater(stack: &mut Vec<Item>) -> EvalResult
{
	filter_condition(stack, |lhs, rhs| (lhs.value as f64) > rhs.value())
}

pub fn less(stack: &mut Vec<Item>) -> EvalResult
{
	filter_condition(stack, |lhs, rhs| (lhs.value as f64) < rhs.value())
}

pub fn greater_or_equal(stack: &mut Vec<Item>) -> EvalResult
{
	filter_condition(stack, |lhs, rhs| (lhs.value as f64) >= rhs.value())
}

pub fn less_or_equal(stack: &mut Vec<Item>) -> EvalResult
{
	filter_condition(stack, |lhs, rhs| (lhs.value as f64) <= rhs.value())
}

fn simple_binary_operation<F>(stack: &mut Vec<Item>, operation: F) -> EvalResult
where
	F: FnOnce(Item, Item) -> Item,
{
	match double_pop(stack)
	{
		Ok((rhs, lhs)) => Ok(operation(lhs, rhs)),
		Err(Reason::Empty) => Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		}),
		Err(Reason::One) => Err(Error::MissingOperand {
			expected: 2,
			found: 1,
		}),
	}
}

fn filter_condition<F>(stack: &mut Vec<Item>, predicate: F) -> EvalResult
where
	F: Fn(Roll, &Item) -> bool,
{
	if let Ok((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(RollSet(
			lhs.0
				.iter()
				.map(|it| it.remove_unless(|it| predicate(it, &rhs)))
				.collect(),
		)))
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

fn double_pop<T>(vec: &mut Vec<T>) -> Result<(T, T), Reason> // bool: was_empty
{
	let double = (vec.pop(), vec.pop());
	if let (Some(first), Some(second)) = double
	{
		Ok((first, second))
	}
	else
	{
		Err(match double
		{
			(None, None) => Reason::Empty,
			_ => Reason::One,
		})
	}
}

enum Reason
{
	Empty,
	One,
}
