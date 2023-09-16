use super::{InvalidOperandError, Item};

fn unary_plus(stack: &mut Vec<Item>) -> Result<Item, InvalidOperandError>
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

fn unary_minus(stack: &mut Vec<Item>) -> Result<Item, InvalidOperandError>
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

fn add(stack: &mut Vec<Item>) -> Result<Item, InvalidOperandError>
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

fn subtract(stack: &mut Vec<Item>) -> Result<Item, InvalidOperandError>
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

fn multiply(stack: &mut Vec<Item>) -> Result<Item, InvalidOperandError>
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

fn divide(stack: &mut Vec<Item>) -> Result<Item, InvalidOperandError>
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

fn modulo(stack: &mut Vec<Item>) -> Result<Item, InvalidOperandError>
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
