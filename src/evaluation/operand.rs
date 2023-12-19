use std::ops;

use super::{DiceRoll, RollId};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Operand
{
	Number(f64),
	Roll
	{
		id: RollId,
		data: DiceRoll,
	},
}
impl Operand
{
	pub fn value(&self) -> f64
	{
		match self
		{
			Self::Number(n) => *n,
			Self::Roll { data: r, id: _ } => f64::from(r.total()),
		}
	}
}
impl ops::Neg for Operand
{
	type Output = Self;
	fn neg(self) -> Self::Output
	{
		Self::Number(-self.value())
	}
}

impl ops::Add for Operand
{
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output
	{
		Self::Number(self.value() + rhs.value())
	}
}
impl ops::Sub for Operand
{
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output
	{
		self + (-rhs)
	}
}
impl ops::Mul for Operand
{
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output
	{
		Self::Number(self.value() * rhs.value())
	}
}
impl ops::Div for Operand
{
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output
	{
		self * Self::Number(1.0 / rhs.value())
	}
}

impl ops::Rem for Operand
{
	type Output = Self;
	fn rem(self, rhs: Self) -> Self::Output
	{
		Self::Number(self.value() % rhs.value())
	}
}
