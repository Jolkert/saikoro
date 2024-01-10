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
	pub fn operand_type(&self) -> OperandType
	{
		match self
		{
			Self::Number(_) => OperandType::Number,
			Self::Roll { .. } => OperandType::Roll,
		}
	}

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

	pub fn approx_eq(&self, rhs: &Self) -> bool
	{
		f64::abs(self.value() - rhs.value()) < f64::EPSILON
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

#[cfg(test)]
mod tests
{
	pub use super::*;
	use crate::evaluation::Roll;

	#[test]
	fn approx_eq()
	{
		assert_ne!(0.3, 0.1 + 0.2);
		assert!(Operand::Number(0.3).approx_eq(&Operand::Number(0.1 + 0.2)));
	}

	#[test]
	fn operand_type()
	{
		assert_eq!(OperandType::Number, Operand::Number(1.0).operand_type());
		assert_eq!(
			OperandType::Roll,
			Operand::from(RollGroup::new(20, [13].map(Roll::new))).operand_type()
		);
	}

	mod ops
	{
		use super::*;

		#[test]
		fn add()
		{
			assert_eq!(
				Operand::Number(12.0) + Operand::Number(3.0),
				Operand::Number(15.0)
			);
		}
		#[test]
		fn sub()
		{
			assert_eq!(
				Operand::Number(12.0) - Operand::Number(3.0),
				Operand::Number(9.0)
			);
		}
		#[test]
		fn mul()
		{
			assert_eq!(
				Operand::Number(12.0) * Operand::Number(3.0),
				Operand::Number(36.0)
			);
		}
		#[test]
		fn div()
		{
			assert_eq!(
				Operand::Number(12.0) / Operand::Number(3.0),
				Operand::Number(4.0)
			);
		}
		#[test]
		fn rem()
		{
			assert_eq!(
				Operand::Number(12.0) % Operand::Number(3.0),
				Operand::Number(0.0)
			);
		}
	}
}
