pub mod functions;

use std::{cmp::Ordering, ops};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Item
{
	Number(f64),
	Roll(RollSet),
}
impl Item
{
	fn value(&self) -> f64
	{
		match self
		{
			Self::Number(n) => *n,
			Self::Roll(r) => r.total() as f64,
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
		self * Item::Number(1.0 / rhs.value())
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

#[derive(Debug)]
pub struct RollSet(Vec<Roll>);
impl RollSet
{
	pub fn total(&self) -> u64
	{
		self.0
			.iter()
			.filter(|it| !it.removed)
			.map(|it| it.value)
			.sum()
	}
}
impl PartialEq for RollSet
{
	fn eq(&self, other: &Self) -> bool
	{
		self.total() == other.total()
	}
}
impl PartialOrd for RollSet
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		self.total().partial_cmp(&other.total())
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Roll
{
	pub value: u64,
	pub faces: u64,
	pub removed: bool,
}
impl Roll
{
	fn remove(self) -> Self
	{
		Roll {
			value: self.value,
			faces: self.faces,
			removed: true,
		}
	}

	fn remove_unless<F>(self, predicate: F) -> Self
	where
		F: FnOnce(Self) -> bool,
	{
		if !predicate(self)
		{
			self.remove()
		}
		else
		{
			self.clone()
		}
	}
}
