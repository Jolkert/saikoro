use std::{cmp::Ordering, ops};

use num_rational::Rational64 as r64;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Item
{
	Number(r64),
	Roll(RollSet),
}
impl Item
{
	fn value(&self) -> r64
	{
		match self
		{
			Self::Number(n) => *n,
			Self::Roll(r) => r.total(),
		}
	}
}
impl ops::Neg for Item
{
	type Output = Self;
	fn neg(self) -> Self::Output
	{
		Item::Number(-self.value())
	}
}

impl ops::Add for Item
{
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output
	{
		Item::Number(self.value() + rhs.value())
	}
}
impl ops::Sub for Item
{
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output
	{
		self + (-rhs)
	}
}
impl ops::Mul for Item
{
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output
	{
		Item::Number(self.value() * rhs.value())
	}
}
impl ops::Div for Item
{
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output
	{
		self * Item::Number(r64::from_integer(1) / rhs.value())
	}
}

impl ops::Rem for Item
{
	type Output = Self;
	fn rem(self, rhs: Self) -> Self::Output
	{
		Item::Number(self.value() % rhs.value())
	}
}

#[derive(Eq)]
pub struct RollSet(Vec<Roll>);
impl RollSet
{
	pub fn total(&self) -> r64
	{
		r64::from_integer(self.0.iter().map(|it| it.value).sum())
	}
}
impl PartialEq for RollSet
{
	fn eq(&self, other: &Self) -> bool
	{
		self.total() == other.total()
	}
}
impl Ord for RollSet
{
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.total().cmp(&other.total())
	}
}
impl PartialOrd for RollSet
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		self.total().partial_cmp(&other.total())
	}
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Roll
{
	pub value: i64,
	pub faces: u64,
}

#[derive(Debug)]
struct InvalidOperandError();

mod functions;
