use super::{Operand, RollGroup, RollId};
use crate::{evaluation::Roll, RangeRng};
use thiserror::Error;

type EvalResult = Result<Operand, MissingOperandError>;

pub fn unary_plus<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	stack.pop().map_or_else(
		|| {
			Err(MissingOperandError {
				expected: 1,
				found: 0,
			})
		},
		|i| Ok(Operand::Number(i.value())),
	)
}

pub fn unary_minus<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	stack.pop().map_or_else(
		|| {
			Err(MissingOperandError {
				expected: 1,
				found: 0,
			})
		},
		|i| Ok(-i),
	)
}

pub fn add<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	simple_binary_operation(stack, |lhs, rhs| lhs + rhs)
}

pub fn subtract<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	simple_binary_operation(stack, |lhs, rhs| lhs - rhs)
}

pub fn multiply<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	simple_binary_operation(stack, |lhs, rhs| lhs * rhs)
}

pub fn divide<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	simple_binary_operation(stack, |lhs, rhs| lhs * rhs)
}

pub fn modulo<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	simple_binary_operation(stack, |lhs, rhs| lhs % rhs)
}

pub fn pow<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	simple_binary_operation(stack, |lhs, rhs| {
		Operand::Number(lhs.value().powf(rhs.value()))
	})
}

pub fn roll<R>(stack: &mut Vec<Operand>, random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	match double_pop(stack)
	{
		Ok((rhs, lhs)) =>
		{
			let mut roll_vec = Vec::<Roll>::new();
			let faces = clamp_f64_to_u32(rhs.value());

			for _ in 0..(clamp_f64_to_u32(lhs.value()))
			{
				roll_vec.push(Roll::new(random.rng_range(0..faces) + 1));
			}

			Ok(Operand::Roll {
				id: RollId::new(),
				data: RollGroup::new(faces, roll_vec),
			})
		}
		Err(Reason::Empty) => Err(MissingOperandError {
			expected: 2,
			found: 0,
		}),
		Err(Reason::One) => Err(MissingOperandError {
			expected: 2,
			found: 1,
		}),
	}
}

pub fn equal<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	filter_condition(stack, |lhs, rhs| {
		(f64::from(lhs.original_value) - rhs.value()).abs() < f64::EPSILON
	})
}

pub fn not_equal<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	filter_condition(stack, |lhs, rhs| {
		(f64::from(lhs.original_value) - rhs.value()).abs() > f64::EPSILON
	})
}

pub fn greater<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	filter_condition(stack, |lhs, rhs| {
		(f64::from(lhs.original_value)) > rhs.value()
	})
}

pub fn less<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	filter_condition(stack, |lhs, rhs| {
		(f64::from(lhs.original_value)) < rhs.value()
	})
}

pub fn greater_or_equal<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	filter_condition(stack, |lhs, rhs| {
		(f64::from(lhs.original_value)) >= rhs.value()
	})
}

pub fn less_or_equal<R>(stack: &mut Vec<Operand>, _random: &mut R) -> EvalResult
where
	R: RangeRng,
{
	filter_condition(stack, |lhs, rhs| {
		(f64::from(lhs.original_value)) <= rhs.value()
	})
}

fn simple_binary_operation<F>(stack: &mut Vec<Operand>, operation: F) -> EvalResult
where
	F: FnOnce(Operand, Operand) -> Operand,
{
	match double_pop(stack)
	{
		Ok((rhs, lhs)) => Ok(operation(lhs, rhs)),
		Err(Reason::Empty) => Err(MissingOperandError {
			expected: 2,
			found: 0,
		}),
		Err(Reason::One) => Err(MissingOperandError {
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
			data: RollGroup::new(
				rolls.faces,
				rolls
					.iter()
					.map(|it| it.remove_unless(|it| predicate(it, &rhs))),
			),
		})
	}
	else
	{
		Err(MissingOperandError {
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

#[derive(Debug, Error)]
#[error("expected {} operands; found {}", .expected, .found)]
pub struct MissingOperandError
{
	pub expected: u8,
	pub found: u8,
}
impl MissingOperandError
{
	pub fn new(expected: u8, found: u8) -> Self
	{
		Self { expected, found }
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
