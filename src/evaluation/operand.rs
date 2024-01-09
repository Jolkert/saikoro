use super::{RollGroup, RollId};
use std::ops;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OperandType
{
	Number,
	Roll,
}

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
			Self::Roll { data, .. } => f64::from(data.total()),
		}
	}
	pub fn into_value(self) -> f64
	{
		match self
		{
			Self::Number(n) => n,
			Self::Roll { data, .. } => f64::from(data.total()),
		}
	}

	#[must_use]
	pub fn to_number(&self) -> Self
	{
		Self::Number(self.value())
	}
	#[must_use]
	pub fn into_number(self) -> Self
	{
		Self::Number(self.into_value())
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
		self + -rhs
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
		Self::Number(self.value() / rhs.value())
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

impl From<RollGroup> for Operand
{
	fn from(value: RollGroup) -> Self
	{
		Self::Roll {
			id: RollId::new(),
			data: value,
		}
	}
}
