use crate::{
	evaluation::{Operand, Roll, RollGroup, RollId},
	RangeRng,
};
use thiserror::Error;

#[derive(Debug, Error, Copy, Clone)]
#[error("Expected dice roll on left side of comparison")]
pub struct FilterNumberError;

type EvalResult = Result<Operand, FilterNumberError>;

pub fn unary_plus<R: RangeRng>(operand: &Operand, _random: &mut R) -> Operand
{
	Operand::Number(operand.value())
}
pub fn unary_minus<R: RangeRng>(operand: &Operand, _random: &mut R) -> Operand
{
	-operand
}
pub fn unary_dice<R: RangeRng>(operand: &Operand, random: &mut R) -> Operand
{
	let faces = clamp_f64_to_u32(operand.value());
	let roll = Roll::new(random.rng_range(0..faces) + 1);

	Operand::Roll {
		id: RollId::new(),
		data: RollGroup::new(faces, [roll]),
	}
}

pub fn add<R: RangeRng>(lhs: &Operand, rhs: &Operand, _random: &mut R) -> EvalResult
{
	Ok(lhs + rhs)
}
pub fn subtract<R: RangeRng>(lhs: &Operand, rhs: &Operand, _random: &mut R) -> EvalResult
{
	Ok(lhs - rhs)
}
pub fn multiply<R: RangeRng>(lhs: &Operand, rhs: &Operand, _random: &mut R) -> EvalResult
{
	Ok(lhs * rhs)
}
pub fn divide<R: RangeRng>(lhs: &Operand, rhs: &Operand, _random: &mut R) -> EvalResult
{
	Ok(lhs / rhs)
}
pub fn modulo<R: RangeRng>(lhs: &Operand, rhs: &Operand, _random: &mut R) -> EvalResult
{
	Ok(lhs % rhs)
}
pub fn power<R: RangeRng>(lhs: &Operand, rhs: &Operand, _random: &mut R) -> EvalResult
{
	Ok(Operand::Number(lhs.value().powf(rhs.value())))
}
pub fn dice<R: RangeRng>(lhs: &Operand, rhs: &Operand, random: &mut R) -> EvalResult
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

pub fn comparison<F>(lhs: &Operand, rhs: &Operand, predicate: F) -> EvalResult
where
	F: Fn(&Roll, &Operand) -> bool,
{
	if let Operand::Roll { id, data } = lhs
	{
		Ok(Operand::Roll {
			id: *id,
			data: RollGroup::new(
				data.faces,
				data.iter()
					.map(|roll| roll.remove_unless(|it| predicate(it, rhs))),
			),
		})
	}
	else
	{
		Err(FilterNumberError)
	}
}
pub fn equal<R: RangeRng>(lhs: &Operand, rhs: &Operand, _rng: &mut R) -> EvalResult
{
	comparison(lhs, rhs, |l, r| {
		(f64::from(l.original_value) - r.value()).abs() < f64::EPSILON
	})
}
pub fn not_equal<R: RangeRng>(lhs: &Operand, rhs: &Operand, _rng: &mut R) -> EvalResult
{
	comparison(lhs, rhs, |l, r| {
		(f64::from(l.original_value) - r.value()).abs() > f64::EPSILON
	})
}
pub fn greater<R: RangeRng>(lhs: &Operand, rhs: &Operand, _rng: &mut R) -> EvalResult
{
	comparison(lhs, rhs, |l, r| f64::from(l.original_value) > r.value())
}
pub fn less<R: RangeRng>(lhs: &Operand, rhs: &Operand, _rng: &mut R) -> EvalResult
{
	comparison(lhs, rhs, |l, r| f64::from(l.original_value) < r.value())
}
pub fn greater_or_equal<R: RangeRng>(lhs: &Operand, rhs: &Operand, _rng: &mut R) -> EvalResult
{
	comparison(lhs, rhs, |l, r| f64::from(l.original_value) >= r.value())
}
pub fn less_or_equal<R: RangeRng>(lhs: &Operand, rhs: &Operand, _rng: &mut R) -> EvalResult
{
	comparison(lhs, rhs, |l, r| f64::from(l.original_value) <= r.value())
}

#[allow(clippy::cast_sign_loss)]
fn clamp_f64_to_u32(value: f64) -> u32
{
	value.clamp(f64::from(u32::MIN), f64::from(u32::MAX)) as u32
}
