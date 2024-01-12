use super::{RollGroup, RollId};
use std::{fmt::Display, ops};

/// An enum representing the two variants of [`Operand`]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OperandType
{
	Number,
	Roll,
}
impl Display for OperandType
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(
			f,
			"{}",
			match self
			{
				Self::Number => "Number",
				Self::Roll => "Roll",
			}
		)
	}
}

/// An enum corresponding to the two types of operands that can be used as arguments in operator functions
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
	/// Returns [`OperandType::Number`] if the [`Operand`] is a [`Number`][`Operand::Number`] variant,
	/// and [`OperandType::Roll`] if the [`Operand`] is a [`Roll`][`Operand::Roll`] variant
	/// # Examples
	/// ```rust
	/// # use saikoro::evaluation::{Operand, OperandType};
	/// let operand = Operand::Number(5.0);
	/// assert_eq!(operand.operand_type(), OperandType::Number)
	/// ```
	pub fn operand_type(&self) -> OperandType
	{
		match self
		{
			Self::Number(_) => OperandType::Number,
			Self::Roll { .. } => OperandType::Roll,
		}
	}

	/// Returns the numerical value of the [`Operand`]. This is the underlying [`f64`] of a
	/// [`Number`][`Operand::Number`] variant or the total of a [`Roll`][`Operand::Roll`] variant
	/// # Examples
	/// ```rust
	/// # use saikoro::evaluation::{Operand, Roll, RollGroup};
	/// let number = Operand::Number(3.0);
	/// assert_eq!(number.value(), 3.0);
	///
	/// let roll = Operand::from(RollGroup::new(6, [Roll::new(5), Roll::new(6)]));
	/// assert_eq!(roll.value(), 11.0);
	/// ```
	pub fn value(&self) -> f64
	{
		match self
		{
			Self::Number(n) => *n,
			Self::Roll { data, .. } => f64::from(data.total()),
		}
	}

	/// The same as [`value`][`Operand::value``], but consumes `self`  
	/// (see [`value`][`Operand::value`] for details)
	pub fn into_value(self) -> f64
	{
		match self
		{
			Self::Number(n) => n,
			Self::Roll { data, .. } => f64::from(data.total()),
		}
	}

	/// Returns a new [`Number`][`Operand::Number`] variant with with the [`value`][`Operand::value`]
	/// of `self`
	/// # Examples
	/// ```rust
	/// # use saikoro::evaluation::{Operand, Roll, RollGroup};
	/// let operand = Operand::from(RollGroup::new(6, [Roll::new(4), Roll::new(2)]));
	/// assert_eq!(operand.to_number(), Operand::Number(6.0));
	/// ```
	#[must_use]
	pub fn to_number(&self) -> Self
	{
		Self::Number(self.value())
	}

	/// The same as [`to_number`][`Operand::to_number``], but consumes `self`  
	/// (see [`to_number`][`Operand::to_number`] for details)
	#[must_use]
	pub fn into_number(self) -> Self
	{
		Self::Number(self.into_value())
	}

	/// Returns whether or not the two values are within [`f64::EPSILON`] of one another
	/// # Examples
	/// ```rust
	/// # use saikoro::evaluation::Operand;
	/// // floating point arithmetic means that 0.1 + 0.2 != 0.3
	/// assert_ne!(0.1 + 0.2, 0.3);
	///
	/// let sum = Operand::Number(0.1) + Operand::Number(0.2);
	///
	/// // by extension, Operand::Number(0.1) + Operand::Number(0.2) != Operand::Number(0.3)
	/// assert_ne!(sum, Operand::Number(0.3));
	///
	/// // however, approx_eq will return true, as they are within f64::EPSILON distance of one another
	/// assert!(sum.approx_eq(&Operand::Number(0.3)));
	/// ```
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
