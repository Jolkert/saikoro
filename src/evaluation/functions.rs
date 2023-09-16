use super::{InvalidOperandError, Item, RollSet};
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
