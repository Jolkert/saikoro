use crate::{
	evaluation::{Operand, Roll, RollGroup},
	RangeRng,
};

type EvalResult = Result<Operand, OperatorError>;

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

pub fn add<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> EvalResult
{
	Ok(lhs + rhs)
}
pub fn subtract<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> EvalResult
{
	Ok(lhs - rhs)
}
pub fn multiply<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> EvalResult
{
	Ok(lhs * rhs)
}
pub fn divide<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> EvalResult
{
	Ok(lhs / rhs)
}
pub fn modulo<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> EvalResult
{
	Ok(lhs % rhs)
}
pub fn power<R: RangeRng>(lhs: Operand, rhs: Operand, _random: &mut R) -> EvalResult
{
	Ok(Operand::Number(lhs.into_value().powf(rhs.into_value())))
}
pub fn dice<R: RangeRng>(lhs: Operand, rhs: Operand, random: &mut R) -> EvalResult
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
pub fn comparison<F>(lhs: Operand, rhs: Operand, predicate: F) -> EvalResult
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
pub fn equal<R: RangeRng>(lhs: Operand, rhs: Operand, _rng: &mut R) -> EvalResult
{
	comparison(lhs, rhs, |l, r| {
		(f64::from(l.original_value) - r.value()).abs() < f64::EPSILON
	})
}
pub fn not_equal<R: RangeRng>(lhs: Operand, rhs: Operand, _rng: &mut R) -> EvalResult
{
	comparison(lhs, rhs, |l, r| {
		(f64::from(l.original_value) - r.value()).abs() > f64::EPSILON
	})
}
pub fn greater<R: RangeRng>(lhs: Operand, rhs: Operand, _rng: &mut R) -> EvalResult
{
	comparison(lhs, rhs, |l, r| f64::from(l.original_value) > r.value())
}
pub fn less<R: RangeRng>(lhs: Operand, rhs: Operand, _rng: &mut R) -> EvalResult
{
	comparison(lhs, rhs, |l, r| f64::from(l.original_value) < r.value())
}
pub fn greater_or_equal<R: RangeRng>(lhs: Operand, rhs: Operand, _rng: &mut R) -> EvalResult
{
	comparison(lhs, rhs, |l, r| f64::from(l.original_value) >= r.value())
}
pub fn less_or_equal<R: RangeRng>(lhs: Operand, rhs: Operand, _rng: &mut R) -> EvalResult
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
pub enum OperatorError
{
	NumberComparisonLhs(Operand),
}
