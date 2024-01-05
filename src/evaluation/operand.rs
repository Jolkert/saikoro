use super::{RollGroup, RollId};
use std::ops;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Operand
{
	Number(f64),
	Roll
	{
		id: RollId,
		data: RollGroup,
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
impl ops::Neg for &Operand
{
	type Output = Operand;
	fn neg(self) -> Self::Output
	{
		Operand::Number(-self.value())
	}
}

impl ops::Add for &Operand
{
	type Output = Operand;
	fn add(self, rhs: Self) -> Self::Output
	{
		Operand::Number(self.value() + rhs.value())
	}
}
impl ops::Sub for &Operand
{
	type Output = Operand;
	fn sub(self, rhs: Self) -> Self::Output
	{
		Operand::Number(self.value() - rhs.value())
	}
}
impl ops::Mul for &Operand
{
	type Output = Operand;
	fn mul(self, rhs: Self) -> Self::Output
	{
		Operand::Number(self.value() * rhs.value())
	}
}
impl ops::Div for &Operand
{
	type Output = Operand;
	fn div(self, rhs: Self) -> Self::Output
	{
		Operand::Number(self.value() / rhs.value())
	}
}

impl ops::Rem for &Operand
{
	type Output = Operand;
	fn rem(self, rhs: Self) -> Self::Output
	{
		Operand::Number(self.value() % rhs.value())
	}
}
