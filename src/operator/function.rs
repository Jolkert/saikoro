// i promise i want to consume all of these thanks -morgan 2024-01-14
#![allow(clippy::needless_pass_by_value)]
use crate::{
	evaluation::{Operand, Roll, RollGroup},
	RangeRng,
};

pub fn unary_plus<R: RangeRng>(operand: Operand, _random: &mut R) -> Operand
{
	operand.into_number()
}
pub fn unary_minus<R: RangeRng>(operand: Operand, _random: &mut R) -> Operand
{
	-operand
}
pub fn unary_dice<R: RangeRng>(operand: Operand, random: &mut R) -> Operand
{
	let faces = clamp_f64_to_u32(operand.into_value());
	let roll = Roll::new(random.rng_range(0..faces) + 1);

	Operand::from(RollGroup::new(faces, [roll]))
}

pub fn add<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> Operand
{
	lhs + rhs
}
pub fn subtract<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> Operand
{
	lhs - rhs
}
pub fn multiply<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> Operand
{
	lhs * rhs
}
pub fn divide<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> Operand
{
	lhs / rhs
}
pub fn modulo<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> Operand
{
	lhs % rhs
}
pub fn power<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> Operand
{
	Operand::Number(lhs.into_value().powf(rhs.into_value()))
}
pub fn dice<R: RangeRng>(lhs: Operand, rhs: Operand, random: &mut R) -> Operand
{
	Operand::from(dice_roll(
		clamp_f64_to_u32(lhs.value()),
		clamp_f64_to_u32(rhs.value()),
		random,
	))
}

pub fn eq_roll_comp<R: RangeRng>(
	dice_lhs: Operand,
	dice_rhs: Operand,
	compare_to: Operand,
	random: &mut R,
) -> Operand
{
	Operand::from(roll_compare(
		clamp_f64_to_u32(dice_lhs.value()),
		clamp_f64_to_u32(dice_rhs.value()),
		compare_to,
		|l, r| f64::from(l.original_value).approx_eq(r.value()),
		random,
	))
}
pub fn ne_roll_comp<R: RangeRng>(
	dice_lhs: Operand,
	dice_rhs: Operand,
	compare_to: Operand,
	random: &mut R,
) -> Operand
{
	Operand::from(roll_compare(
		clamp_f64_to_u32(dice_lhs.value()),
		clamp_f64_to_u32(dice_rhs.value()),
		compare_to,
		|l, r| !f64::from(l.original_value).approx_eq(r.value()),
		random,
	))
}
pub fn greater_roll_comp<R: RangeRng>(
	dice_lhs: Operand,
	dice_rhs: Operand,
	compare_to: Operand,
	random: &mut R,
) -> Operand
{
	Operand::from(roll_compare(
		clamp_f64_to_u32(dice_lhs.value()),
		clamp_f64_to_u32(dice_rhs.value()),
		compare_to,
		|l, r| f64::from(l.original_value) > r.value(),
		random,
	))
}
pub fn less_roll_comp<R: RangeRng>(
	dice_lhs: Operand,
	dice_rhs: Operand,
	compare_to: Operand,
	random: &mut R,
) -> Operand
{
	Operand::from(roll_compare(
		clamp_f64_to_u32(dice_lhs.value()),
		clamp_f64_to_u32(dice_rhs.value()),
		compare_to,
		|l, r| f64::from(l.original_value) < r.value(),
		random,
	))
}
pub fn greater_eq_roll_comp<R: RangeRng>(
	dice_lhs: Operand,
	dice_rhs: Operand,
	compare_to: Operand,
	random: &mut R,
) -> Operand
{
	Operand::from(roll_compare(
		clamp_f64_to_u32(dice_lhs.value()),
		clamp_f64_to_u32(dice_rhs.value()),
		compare_to,
		|l, r| f64::from(l.original_value) >= r.value(),
		random,
	))
}
pub fn less_eq_roll_comp<R: RangeRng>(
	dice_lhs: Operand,
	dice_rhs: Operand,
	compare_to: Operand,
	random: &mut R,
) -> Operand
{
	Operand::from(roll_compare(
		clamp_f64_to_u32(dice_lhs.value()),
		clamp_f64_to_u32(dice_rhs.value()),
		compare_to,
		|l, r| f64::from(l.original_value) <= r.value(),
		random,
	))
}

fn roll_compare<F, R>(
	count: u32,
	faces: u32,
	rhs: Operand,
	predicate: F,
	random: &mut R,
) -> RollGroup
where
	F: Fn(&Roll, &Operand) -> bool,
	R: RangeRng,
{
	let roll = dice_roll(count, faces, random);
	comparison(roll, rhs, predicate)
}

fn comparison<F>(lhs: RollGroup, rhs: Operand, predicate: F) -> RollGroup
where
	F: Fn(&Roll, &Operand) -> bool,
{
	let mut lhs = lhs;
	for roll in &mut lhs
	{
		roll.remove_unless(|it| predicate(it, &rhs));
	}
	lhs
}

fn dice_roll<R: RangeRng>(count: u32, faces: u32, random: &mut R) -> RollGroup
{
	let values = (0..count).map(|_| Roll::new(random.rng_range(0..faces) + 1));
	RollGroup::new(faces, values)
}

#[allow(clippy::cast_sign_loss)]
fn clamp_f64_to_u32(value: f64) -> u32
{
	value.clamp(f64::from(u32::MIN), f64::from(u32::MAX)) as u32
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

#[cfg(test)]
mod tests
{
	use crate::{evaluation::Operand, test_helpers::RiggedRandom};

	#[test]
	fn add()
	{
		assert!(super::add(
			Operand::Number(12.0),
			Operand::Number(3.0),
			&mut rand::thread_rng(),
		)
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
		.approx_eq(&Operand::Number(12.0)));
	}
}
