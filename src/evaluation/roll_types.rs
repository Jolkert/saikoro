use std::cmp::Ordering;

#[derive(Debug)]
pub struct RollSet(pub Vec<Roll>);
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
