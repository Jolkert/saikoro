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
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		Ok(lhs + rhs)
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0, // this isnt really true fix this -morgan 2023-09-19
		})
	}
}

pub fn subtract(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		Ok(rhs - lhs)
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

pub fn multiply(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		Ok(rhs * lhs)
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

pub fn divide(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		Ok(lhs / rhs)
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

pub fn modulo(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		Ok(lhs % rhs)
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

pub fn pow(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		Ok(Item::Number(lhs.value().powf(rhs.value())))
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

pub fn roll(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
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
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

pub fn equal(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(filter_condition(lhs, |r| {
			r.value as f64 == rhs.value()
		})))
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

pub fn not_equal(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(filter_condition(lhs, |r| {
			r.value as f64 != rhs.value()
		})))
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

pub fn greater(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(filter_condition(lhs, |r| {
			r.value as f64 > rhs.value()
		})))
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

pub fn less(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(filter_condition(lhs, |r| {
			(r.value as f64) < rhs.value()
		})))
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

pub fn greater_or_equal(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(filter_condition(lhs, |r| {
			(r.value as f64) >= rhs.value()
		})))
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

pub fn less_or_equal(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(filter_condition(lhs, |r| {
			(r.value as f64) <= rhs.value()
		})))
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

fn filter_condition<F>(rolls: RollSet, predicate: F) -> RollSet
where
	F: Fn(Roll) -> bool,
{
	RollSet(
		rolls
			.0
			.iter()
			.map(|it| it.remove_unless(&predicate))
			.collect(),
	)
}

fn double_pop<T>(vec: &mut Vec<T>) -> Option<(T, T)>
{
	if let (Some(first), Some(second)) = (vec.pop(), vec.pop())
	{
		Some((first, second))
	}
	else
	{
		None
	}
}
