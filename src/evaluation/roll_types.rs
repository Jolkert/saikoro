use std::cmp::Ordering;

#[derive(Debug)]
pub struct DiceRoll
{
	rolls: Box<[Roll]>,
	faces: u64,
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
			.filter(|it| !it.removed)
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
	fn new(value: u64) -> Self
	{
		Roll {
			value,
			removed: false,
		}
	}

	fn remove(self) -> Self
	{
		Roll {
			value: self.value,
			removed: true,
		}
	}

	pub fn remove_unless<F>(self, predicate: F) -> Self
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
