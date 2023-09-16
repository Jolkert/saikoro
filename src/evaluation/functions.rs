use crate::evaluation::Roll;

use super::{InvalidOperandError, Item, RollSet};
use rand::prelude::*;

type EvalResult = Result<Item, InvalidOperandError>;

fn unary_plus(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some(i) = stack.pop()
	{
		Ok(i)
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn unary_minus(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some(i) = stack.pop()
	{
		Ok(-i)
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn add(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		Ok(lhs + rhs)
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn subtract(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		Ok(rhs - lhs)
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn multiply(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		Ok(rhs * lhs)
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn divide(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		Ok(lhs / rhs)
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn modulo(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		Ok(lhs % rhs)
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn pow(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		Ok(Item::Number(lhs.value().powf(rhs.value())))
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn roll(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, lhs)) = double_pop(stack)
	{
		let mut roll_vec = Vec::<Roll>::new();
		let faces = rhs.value() as u64;

		for i in 0..(lhs.value() as u64)
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
		Err(InvalidOperandError {})
	}
}

fn equal(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(RollSet(
			lhs.0
				.iter()
				.map(|it| it.remove_unless(|r| r.value as f64 == rhs.value()))
				.collect(),
		)))
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn not_equal(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(RollSet(
			lhs.0
				.iter()
				.map(|it| it.remove_unless(|r| r.value as f64 != rhs.value()))
				.collect(),
		)))
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn filter_greater(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(RollSet(
			lhs.0
				.iter()
				.map(|it| it.remove_unless(|r| r.value as f64 > rhs.value()))
				.collect(),
		)))
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn filter_less(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(RollSet(
			lhs.0
				.iter()
				.map(|it| it.remove_unless(|r| (r.value as f64) < rhs.value()))
				.collect(),
		)))
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn filter_greater_equal(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(RollSet(
			lhs.0
				.iter()
				.map(|it| it.remove_unless(|r| r.value as f64 >= rhs.value()))
				.collect(),
		)))
	}
	else
	{
		Err(InvalidOperandError {})
	}
}

fn filter_less_equal(stack: &mut Vec<Item>) -> EvalResult
{
	if let Some((rhs, Item::Roll(lhs))) = double_pop(stack)
	{
		Ok(Item::Roll(RollSet(
			lhs.0
				.iter()
				.map(|it| it.remove_unless(|r| r.value as f64 <= rhs.value()))
				.collect(),
		)))
	}
	else
	{
		Err(InvalidOperandError {})
	}
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
