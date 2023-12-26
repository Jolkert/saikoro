use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Clone)]
pub struct RollGroup
{
	rolls: Box<[Roll]>,
	pub faces: u32,
}
impl RollGroup
{
	pub fn new<I>(faces: u32, rolls: I) -> Self
	where
		I: IntoIterator<Item = Roll>,
	{
		Self {
			rolls: rolls.into_iter().collect(),
			faces,
		}
	}
	pub fn total(&self) -> u32
	{
		self.rolls
			.iter()
			.filter(|it| !it.is_removed())
			.map(|it| it.original_value)
			.sum::<u32>()
	}
	pub fn len(&self) -> usize
	{
		self.rolls.len()
	}
	pub fn is_empty(&self) -> bool
	{
		self.len() == 0
	}
	pub fn iter(&self) -> std::slice::Iter<Roll>
	{
		self.rolls.iter()
	}
}
impl Display for RollGroup
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(
			f,
			"{}d{}: [{}]",
			self.len(),
			self.faces,
			self.rolls
				.iter()
				.map(ToString::to_string)
				.collect::<Vec<_>>()
				.join(", ")
		)
	}
}
impl<'a> IntoIterator for &'a RollGroup
{
	type Item = &'a Roll;
	type IntoIter = std::slice::Iter<'a, Roll>;

	fn into_iter(self) -> Self::IntoIter
	{
		self.rolls.iter()
	}
}
impl PartialEq for RollGroup
{
	fn eq(&self, other: &Self) -> bool
	{
		self.total() == other.total()
	}
}
impl PartialOrd for RollGroup
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		self.total().partial_cmp(&other.total())
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Roll
{
	pub original_value: u32,
	removed: bool,
}
impl Roll
{
	pub fn new(value: u32) -> Self
	{
		Self {
			original_value: value,
			removed: false,
		}
	}

	pub fn value(&self) -> Option<u32>
	{
		(!self.removed).then_some(self.original_value)
	}

	pub fn is_removed(&self) -> bool
	{
		self.removed
	}

	#[must_use]
	pub fn remove(self) -> Self
	{
		Self {
			removed: true,
			..self
		}
	}

	#[must_use]
	pub fn remove_unless<F>(self, predicate: F) -> Self
	where
		F: FnOnce(&Self) -> bool,
	{
		if predicate(&self)
		{
			self
		}
		else
		{
			self.remove()
		}
	}
}
impl Display for Roll
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let wrap_str = self.is_removed().then_some("~~").unwrap_or_default();
		write!(f, "{}{}{}", wrap_str, self.original_value, wrap_str)
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct RollId(u64);
impl RollId
{
	pub fn new() -> Self
	{
		Self(rand::random())
	}
}
impl Default for RollId
{
	fn default() -> Self
	{
		Self::new()
	}
}
