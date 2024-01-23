// TODO: i want to move this out of evaluation, but im not entirely sure where to put it -morgan
// 2024-01-10
use std::{cmp::Ordering, fmt::Display};

/// A group of [`Roll`]s and the number of faces on the dice they were originally rolled from
#[derive(Debug, Clone)]
pub struct RollGroup
{
	rolls: Box<[Roll]>,
	pub faces: u32,
}
impl RollGroup
{
	/// Creates a new [`RollGroup`] given the provided number of dice faces, and a collection
	/// of individual roll values
	pub fn new<I>(faces: u32, rolls: I) -> Self
	where
		I: IntoIterator<Item = Roll>,
	{
		Self {
			rolls: rolls.into_iter().collect(),
			faces,
		}
	}

	/// Gets the sum of all [`Roll`]s in the `self` [`RollGroup`], ignoring all [`Roll`]s whose
	/// values were filtered out. Returns 0 for an empty [`RollGroup`], or one where every [`Roll`]
	/// has been filtered out
	/// # Examples
	/// ```rust
	/// # use saikoro::evaluation::{Roll, RollGroup};
	/// let roll_group = RollGroup::new(6, [5, 3, 1].map(Roll::new));
	/// assert_eq!(roll_group.total(), 9);
	///
	/// let roll_group = RollGroup::new(6, [Roll::new(5), Roll::new(3).into_removed(), Roll::new(1)]);
	/// assert_eq!(roll_group.total(), 6);
	/// ```
	pub fn total(&self) -> u32
	{
		self.rolls
			.iter()
			.filter(|it| !it.is_removed())
			.map(|it| it.original_value)
			.sum::<u32>()
	}

	/// Returns the number of elements in the [`RollGroup`]
	/// # Examples
	/// ```rust
	/// # use saikoro::evaluation::{Roll, RollGroup};
	/// let roll_group = RollGroup::new(6, [4, 6, 1, 2].map(Roll::new));
	/// assert_eq!(roll_group.len(), 4);
	/// ```
	pub fn len(&self) -> usize
	{
		self.rolls.len()
	}

	/// Returns `true` if the [`RollGroup`] has a length of 0
	/// # Examples
	/// ```rust
	/// # use saikoro::evaluation::{Roll, RollGroup};
	/// let roll_group = RollGroup::new(6, [4, 6, 1, 2].map(Roll::new));
	/// assert!(!roll_group.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool
	{
		self.len() == 0
	}

	/// # Examples
	/// ```rust
	/// # use saikoro::evaluation::{Roll, RollGroup};
	/// let roll_group = &RollGroup::new(6, [4, 1, 2].map(Roll::new));
	/// let mut iter = roll_group.iter();
	/// assert_eq!(iter.next(), Some(&Roll::new(4)));
	/// assert_eq!(iter.next(), Some(&Roll::new(1)));
	/// assert_eq!(iter.next(), Some(&Roll::new(2)));
	/// assert_eq!(iter.next(), None);
	/// ```
	pub fn iter(&self) -> std::slice::Iter<Roll>
	{
		self.rolls.iter()
	}

	pub(crate) fn iter_mut(&mut self) -> std::slice::IterMut<Roll>
	{
		self.rolls.iter_mut()
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
impl<'a> IntoIterator for &'a mut RollGroup
{
	type Item = &'a mut Roll;
	type IntoIter = std::slice::IterMut<'a, Roll>;

	fn into_iter(self) -> Self::IntoIter
	{
		self.iter_mut()
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

/// A value representing an individual die roll with information on whether or not it should count
/// toward the value of its parent [`RollGroup`]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Roll
{
	pub original_value: u32,
	removed: bool,
}
impl Roll
{
	/// Creates a new [`Roll`] where the original value is the given [`u32`] and is considered
	/// "not removed" (i.e. its value counts toward its parent [`RollGroup`])
	pub fn new(value: u32) -> Self
	{
		Self {
			original_value: value,
			removed: false,
		}
	}

	/// Gets the value of the [`Roll`] as an [`Option`]. Returns [`None`] if the [`Roll`] has been
	/// removed, and otherwise returns the underlying value as [`Some(u32)`]
	/// # Examples
	/// ```rust
	/// # use saikoro::evaluation::Roll;
	/// let roll = Roll::new(15);
	/// assert!(roll.value().is_some_and(|val| val == 15));
	///
	/// let removed = roll.into_removed();
	/// assert!(removed.value().is_none());
	/// ```
	pub fn value(&self) -> Option<u32>
	{
		(!self.removed).then_some(self.original_value)
	}

	/// Returns a [`bool`] representing whether or not the [`Roll`] is removed (i.e. its value
	/// was filtered out and should not count toward the total value)
	pub fn is_removed(&self) -> bool
	{
		self.removed
	}

	/// Sets `self` to be removed, and returns it back to the caller
	#[must_use]
	pub fn into_removed(self) -> Self
	{
		Self {
			removed: true,
			..self
		}
	}

	pub fn remove(&mut self)
	{
		self.removed = true;
	}

	pub fn remove_unless<F>(&mut self, predicate: F)
	where
		F: FnOnce(&Self) -> bool,
	{
		if !predicate(self)
		{
			self.remove();
		}
	}

	/// calls [`into_removed`][`Roll::into_removed`] on `self` if it does **not** match the
	/// predicate, otherwise it simply acts as a no-op and returns `self` back
	#[must_use]
	pub fn into_removed_unless<F>(self, predicate: F) -> Self
	where
		F: FnOnce(&Self) -> bool,
	{
		if predicate(&self)
		{
			self
		}
		else
		{
			self.into_removed()
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

/// A value that can be used to uniquely identify a roll.
/// # Technical Details
/// Currently, [`RollId`] acts simply as a wrapper around a randomly generated [`u64`], and
/// *technically* has no guarantees against collisions, though they should be rare.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct RollId(u64);
impl RollId
{
	/// Generates a new [`RollId`] with a random value
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

#[cfg(test)]
mod tests
{
	use super::*;
	#[test]
	fn remove()
	{
		let roll = Roll::new(5).into_removed();
		assert_eq!(roll.original_value, 5);
		assert!(roll.is_removed());
	}

	#[test]
	fn remove_unless()
	{
		let should_retain = Roll::new(5).into_removed_unless(|it| it.original_value > 3);
		let should_remove = Roll::new(2).into_removed_unless(|it| it.original_value > 3);

		assert!(!should_retain.is_removed());
		assert!(should_remove.is_removed());
	}

	#[test]
	fn value()
	{
		let non_zero = Roll::new(3);
		let removed_non_zero = Roll::new(2).into_removed();
		let zero = Roll::new(0);
		let removed_zero = Roll::new(0).into_removed();

		assert!(non_zero.value().is_some_and(|val| val == 3));
		assert!(removed_non_zero.value().is_none());
		assert!(zero.value().is_some_and(|val| val == 0));
		assert!(removed_zero.value().is_none());
	}
}

#[derive(Debug)]
pub struct DiceEvaluation
{
	pub value: f64,
	pub roll_groups: Box<[RollGroup]>,
}
impl DiceEvaluation
{
	pub fn ungrouped_rolls(&self) -> impl Iterator<Item = &Roll>
	{
		self.roll_groups.iter().flat_map(RollGroup::iter)
	}
}
impl Display for DiceEvaluation
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(
			f,
			"Total: {} [{}]",
			self.value,
			self.roll_groups
				.iter()
				.map(ToString::to_string)
				.collect::<Vec<_>>()
				.join(", ")
		)
	}
}
