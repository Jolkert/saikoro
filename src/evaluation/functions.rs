use crate::evaluation::Roll;

use super::{DiceRoll, Operand, RollId};
use crate::Error;
use rand::prelude::*;

type EvalResult = Result<Operand, Error>;

pub fn unary_plus(stack: &mut Vec<Operand>) -> EvalResult
{
	stack.pop().map_or_else(
		|| {
			Err(Error::MissingOperand {
				expected: 1,
				found: 0,
			})
		},
		|i| Ok(Operand::Number(i.value())),
	)
}

pub fn unary_minus(stack: &mut Vec<Operand>) -> EvalResult
{
	stack.pop().map_or_else(
		|| {
			Err(Error::MissingOperand {
				expected: 1,
				found: 0,
			})
		},
		|i| Ok(-i),
	)
}

pub fn add(stack: &mut Vec<Operand>) -> EvalResult
{
	simple_binary_operation(stack, |lhs, rhs| lhs + rhs)
}

pub fn subtract(stack: &mut Vec<Operand>) -> EvalResult
{
	simple_binary_operation(stack, |lhs, rhs| lhs - rhs)
}

pub fn multiply(stack: &mut Vec<Operand>) -> EvalResult
{
	simple_binary_operation(stack, |lhs, rhs| lhs * rhs)
}

pub fn divide(stack: &mut Vec<Operand>) -> EvalResult
{
	simple_binary_operation(stack, |lhs, rhs| lhs * rhs)
}

pub fn modulo(stack: &mut Vec<Operand>) -> EvalResult
{
	simple_binary_operation(stack, |lhs, rhs| lhs % rhs)
}

pub fn pow(stack: &mut Vec<Operand>) -> EvalResult
{
	simple_binary_operation(stack, |lhs, rhs| {
		Operand::Number(lhs.value().powf(rhs.value()))
	})
}

pub fn roll(stack: &mut Vec<Operand>) -> EvalResult
{
	match double_pop(stack)
	{
		Ok((rhs, lhs)) =>
		{
			let mut roll_vec = Vec::<Roll>::new();
			let faces = clamp_f64_to_u32(rhs.value());

			for _ in 0..(clamp_f64_to_u32(lhs.value()))
			{
				roll_vec.push(Roll::new(rand::thread_rng().gen_range(0..faces) + 1));
			}

			Ok(Operand::Roll {
				id: RollId::new(),
				data: DiceRoll::new(faces, roll_vec),
			}) // oh god so many parens -morgan 2023-09-27
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

pub fn equal(stack: &mut Vec<Operand>) -> EvalResult
{
	filter_condition(stack, |lhs, rhs| {
		(f64::from(lhs.value) - rhs.value()).abs() < f64::EPSILON
	})
}

pub fn not_equal(stack: &mut Vec<Operand>) -> EvalResult
{
	filter_condition(stack, |lhs, rhs| {
		(f64::from(lhs.value) - rhs.value()).abs() > f64::EPSILON
	})
}

pub fn greater(stack: &mut Vec<Operand>) -> EvalResult
{
	filter_condition(stack, |lhs, rhs| (f64::from(lhs.value)) > rhs.value())
}

pub fn less(stack: &mut Vec<Operand>) -> EvalResult
{
	filter_condition(stack, |lhs, rhs| (f64::from(lhs.value)) < rhs.value())
}

pub fn greater_or_equal(stack: &mut Vec<Operand>) -> EvalResult
{
	filter_condition(stack, |lhs, rhs| (f64::from(lhs.value)) >= rhs.value())
}

pub fn less_or_equal(stack: &mut Vec<Operand>) -> EvalResult
{
	filter_condition(stack, |lhs, rhs| (f64::from(lhs.value)) <= rhs.value())
}

fn simple_binary_operation<F>(stack: &mut Vec<Operand>, operation: F) -> EvalResult
where
	F: FnOnce(Operand, Operand) -> Operand,
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

fn filter_condition<F>(stack: &mut Vec<Operand>, predicate: F) -> EvalResult
where
	F: Fn(&Roll, &Operand) -> bool,
{
	if let Ok((rhs, Operand::Roll { id, data: rolls })) = double_pop(stack)
	{
		Ok(Operand::Roll {
			id,
			data: DiceRoll {
				rolls: rolls
					.iter()
					.map(|it| it.remove_unless(|it| predicate(it, &rhs)))
					.collect(),
				..rolls
			},
		})
	}
	else
	{
		Err(Error::MissingOperand {
			expected: 2,
			found: 0,
		})
	}
}

fn double_pop<T>(vec: &mut Vec<T>) -> Result<(T, T), Reason>
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

#[allow(clippy::cast_sign_loss)]
fn clamp_f64_to_u32(value: f64) -> u32
{
	value.clamp(f64::from(u32::MIN), f64::from(u32::MAX)) as u32
}

enum Reason
{
	Empty,
	One,
}
