use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct DiceRoll
{
	pub rolls: Box<[Roll]>,
	pub faces: u32,
}
impl DiceRoll
{
	pub fn new(faces: u32, rolls: Vec<Roll>) -> Self
	{
		Self {
			rolls: rolls.into_boxed_slice(),
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
	pub fn iter(&self) -> std::slice::Iter<Roll>
	{
		self.rolls.iter()
	}
}
impl PartialEq for DiceRoll
{
	fn eq(&self, other: &Self) -> bool
	{
		self.total() == other.total()
	}
}
impl PartialOrd for DiceRoll
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
