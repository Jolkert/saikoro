use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct DiceRoll
{
	pub rolls: Box<[Roll]>,
	pub faces: u64,
}
impl DiceRoll
{
	pub fn new(faces: u64, rolls: Vec<Roll>) -> Self
	{
		DiceRoll {
			rolls: rolls.into_boxed_slice(),
			faces,
		}
	}
	pub fn total(&self) -> u64
	{
		self.rolls
			.iter()
			.filter(|it| !it.is_removed())
			.map(|it| it.value)
			.sum()
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
	pub value: u64,
	pub removed: bool,
}
impl Roll
{
	pub fn new(value: u64) -> Self
	{
		Roll {
			value,
			removed: false,
		}
	}

	fn is_removed(&self) -> bool
	{
		self.removed
	}

	fn remove(self) -> Self
	{
		Self {
			removed: true,
			..self
		}
	}

	pub fn remove_unless<F>(self, predicate: F) -> Self
	where
		F: FnOnce(&Self) -> bool,
	{
		if !predicate(&self)
		{
			self.remove()
		}
		else
		{
			self
		}
	}
}
