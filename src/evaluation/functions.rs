//
#![allow(clippy::unnecessary_wraps)]

use crate::{
	evaluation::{Operand, Roll, RollGroup},
	RangeRng,
};

type OperatorResult = Result<Operand, OperatorError>;

pub(crate) fn unary_plus<R: RangeRng>(operand: Operand, _random: &mut R) -> Operand
{
	operand.into_number()
}
pub(crate) fn unary_minus<R: RangeRng>(operand: Operand, _random: &mut R) -> Operand
{
	-operand
}
pub(crate) fn unary_dice<R: RangeRng>(operand: Operand, random: &mut R) -> Operand
{
	let faces = clamp_f64_to_u32(operand.into_value());
	let roll = Roll::new(random.rng_range(0..faces) + 1);

	Operand::from(RollGroup::new(faces, [roll]))
}

pub(crate) fn add<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> OperatorResult
{
	Ok(lhs + rhs)
}
pub(crate) fn subtract<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> OperatorResult
{
	Ok(lhs - rhs)
}
pub(crate) fn multiply<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> OperatorResult
{
	Ok(lhs * rhs)
}
pub(crate) fn divide<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> OperatorResult
{
	Ok(lhs / rhs)
}
pub(crate) fn modulo<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> OperatorResult
{
	Ok(lhs % rhs)
}
pub(crate) fn power<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> OperatorResult
{
	Ok(Operand::Number(lhs.into_value().powf(rhs.into_value())))
}
pub(crate) fn dice<R: RangeRng>(lhs: Operand, rhs: Operand, random: &mut R) -> OperatorResult
{
	let mut roll_vec = Vec::<Roll>::new();
	let faces = clamp_f64_to_u32(rhs.into_value());

	for _ in 0..(clamp_f64_to_u32(lhs.into_value()))
	{
		roll_vec.push(Roll::new(random.rng_range(0..faces) + 1));
	}

	Ok(Operand::from(RollGroup::new(faces, roll_vec)))
}

#[allow(clippy::needless_pass_by_value)] // i do actually want to consume this thanks -morgan 2024-01-08
pub(crate) fn comparison<F>(lhs: Operand, rhs: Operand, predicate: F) -> OperatorResult
where
	F: Fn(&Roll, &Operand) -> bool,
{
	if let Operand::Roll { id, data } = lhs
	{
		Ok(Operand::Roll {
			data: RollGroup::new(
				data.faces,
				data.iter()
					.map(|roll| roll.remove_unless(|it| predicate(it, &rhs))),
			),
			id,
		})
	}
	else
	{
		Err(OperatorError::NumberComparisonLhs(lhs))
	}
}
pub(crate) fn equal<R: RangeRng>(lhs: Operand, rhs: Operand, _rng: &mut R) -> OperatorResult
{
	comparison(lhs, rhs, |l, r| {
		(f64::from(l.original_value) - r.value()).abs() < f64::EPSILON
	})
}
pub(crate) fn not_equal<R: RangeRng>(lhs: Operand, rhs: Operand, _rng: &mut R) -> OperatorResult
{
	comparison(lhs, rhs, |l, r| {
		(f64::from(l.original_value) - r.value()).abs() > f64::EPSILON
	})
}
pub(crate) fn greater<R: RangeRng>(lhs: Operand, rhs: Operand, _rng: &mut R) -> OperatorResult
{
	comparison(lhs, rhs, |l, r| f64::from(l.original_value) > r.value())
}
pub(crate) fn less<R: RangeRng>(lhs: Operand, rhs: Operand, _rng: &mut R) -> OperatorResult
{
	comparison(lhs, rhs, |l, r| f64::from(l.original_value) < r.value())
}
pub(crate) fn greater_or_equal<R: RangeRng>(
	lhs: Operand,
	rhs: Operand,
	_rng: &mut R,
) -> OperatorResult
{
	comparison(lhs, rhs, |l, r| f64::from(l.original_value) >= r.value())
}
pub(crate) fn less_or_equal<R: RangeRng>(lhs: Operand, rhs: Operand, _rng: &mut R)
	-> OperatorResult
{
	comparison(lhs, rhs, |l, r| f64::from(l.original_value) <= r.value())
}

#[allow(clippy::cast_sign_loss)]
fn clamp_f64_to_u32(value: f64) -> u32
{
	value.clamp(f64::from(u32::MIN), f64::from(u32::MAX)) as u32
}

// don't keep this public, its for internal use only to bubble up to BinaryOperator::eval
// it's in an enum in case there's a need for more error types in the future
// -morgan 2024-01-08
#[derive(Debug)]
pub(crate) enum OperatorError
{
	NumberComparisonLhs(Operand),
}

#[cfg(test)]
mod tests
{
	use crate::{
		evaluation::{Operand, Roll, RollGroup},
		test_helpers::RiggedRandom,
	};

	#[test]
	fn add()
	{
		assert!(super::add(
			Operand::Number(12.0),
			Operand::Number(3.0),
			&mut rand::thread_rng(),
		)
		.unwrap()
		.approx_eq(&Operand::Number(15.0)));
	}
	#[test]
	fn subtract()
	{
		assert!(super::subtract(
			Operand::Number(12.0),
			Operand::Number(3.0),
			&mut rand::thread_rng(),
		)
		.unwrap()
		.approx_eq(&Operand::Number(9.0)));
	}
	#[test]
	fn multiply()
	{
		assert!(super::multiply(
			Operand::Number(12.0),
			Operand::Number(3.0),
			&mut rand::thread_rng(),
		)
		.unwrap()
		.approx_eq(&Operand::Number(36.0)));
	}
	#[test]
	fn divide()
	{
		assert!(super::divide(
			Operand::Number(12.0),
			Operand::Number(3.0),
			&mut rand::thread_rng(),
		)
		.unwrap()
		.approx_eq(&Operand::Number(4.0)));
	}
	#[test]
	fn modulo()
	{
		assert!(super::modulo(
			Operand::Number(12.0),
			Operand::Number(3.0),
			&mut rand::thread_rng(),
		)
		.unwrap()
		.approx_eq(&Operand::Number(0.0)));
	}
	#[test]
	fn power()
	{
		assert!(super::power(
			Operand::Number(12.0),
			Operand::Number(3.0),
			&mut rand::thread_rng(),
		)
		.unwrap()
		.approx_eq(&Operand::Number(1728.0)));
	}
	#[test]
	fn dice()
	{
		assert!(super::dice(
			Operand::Number(4.0),
			Operand::Number(6.0),
			&mut RiggedRandom::new([4, 1, 5, 2]),
		)
		.unwrap()
		.approx_eq(&Operand::Number(12.0)));
	}
	#[test]
	fn equal()
	{
		assert!(super::equal(
			roll_operand(),
			Operand::Number(2.0),
			&mut rand::thread_rng()
		)
		.unwrap()
		.approx_eq(&Operand::Number(4.0)));
	}
	#[test]
	fn not_equal()
	{
		assert!(super::not_equal(
			roll_operand(),
			Operand::Number(2.0),
			&mut rand::thread_rng()
		)
		.unwrap()
		.approx_eq(&Operand::Number(11.0)));
	}
	#[test]
	fn greater()
	{
		assert!(super::greater(
			roll_operand(),
			Operand::Number(2.0),
			&mut rand::thread_rng()
		)
		.unwrap()
		.approx_eq(&Operand::Number(10.0)));
	}
	#[test]
	fn less()
	{
		assert!(super::less(
			roll_operand(),
			Operand::Number(2.0),
			&mut rand::thread_rng()
		)
		.unwrap()
		.approx_eq(&Operand::Number(1.0)));
	}
	#[test]
	fn greater_or_equal()
	{
		assert!(super::greater_or_equal(
			roll_operand(),
			Operand::Number(2.0),
			&mut rand::thread_rng()
		)
		.unwrap()
		.approx_eq(&Operand::Number(14.0)));
	}
	#[test]
	fn less_or_equal()
	{
		assert!(super::less_or_equal(
			roll_operand(),
			Operand::Number(2.0),
			&mut rand::thread_rng()
		)
		.unwrap()
		.approx_eq(&Operand::Number(5.0)));
	}

	fn roll_operand() -> Operand
	{
		Operand::from(RollGroup::new(6, [2, 4, 2, 6, 1].map(Roll::new)))
	}
}
