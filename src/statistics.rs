use std::{cmp::Ordering, collections::HashMap, ops::RangeInclusive};

use crate::evaluation::{DiceEvaluation, RollGroup};

impl RollGroup
{
	/// Returns a [`PopulationData`] representing all possible values the expression which produced
	/// this [`RollGroup`] could have produced. Used for getting staistical information about the
	/// [`RollGroup`]
	pub fn population_data(&self) -> PopulationData
	{
		PopulationData {
			count: self.len() as u32,
			faces: self.faces,
		}
	}

	/// Returns the mean (average) of the [`RollGroup`] with rolls marked as "removed" excluded. For
	/// a calculation that includes removed rolls, see [`mean_raw`][RollGroup::mean_raw]. The
	/// *population mean* see [`RollGroup::population_data`] and
	/// [`PopulationData::mean`]
	pub fn mean(&self) -> f64
	{
		f64::from(self.total())
			/ f64::from(self.iter().filter(|it| !it.is_removed()).count() as u32)
	}

	/// Returns the mean (average) of the [`RollGroup`] including rolls marked as "removed". For a
	/// calculation that excludes removed rolls, see [`mean`][RollGroup::mean]
	pub fn mean_raw(&self) -> f64
	{
		f64::from(self.iter().map(|it| it.original_value).sum::<u32>())
			/ f64::from(self.len() as u32)
	}

	/// Returns the [z-score](https://en.wikipedia.org/wiki/Standard_score) of the
	/// [`RollGroup`] (see [`PopulationData::z_score`] for details)
	pub fn z_score(&self) -> f64
	{
		self.population_data().z_score(self.total())
	}

	/// Returns whether or not all rolls, including removed rolls, are their maximum value
	pub fn is_max_roll(&self) -> bool
	{
		self.iter().all(|roll| roll.original_value >= self.faces)
	}
	/// Returns whether or not all rolls, including removed rolls, are their minimum value (1)
	pub fn is_min_roll(&self) -> bool
	{
		self.iter().all(|roll| roll.original_value <= 1)
	}
}

impl DiceEvaluation
{
	/// Returns the mean (average) [z-score](https://en.wikipedia.org/wiki/Standard_score) of all
	/// [`RollGroups`][`RollGroup`] in the [`DiceEvaluation`] (see [`RollGroup::z_score`] for
	/// details)
	#[must_use]
	pub fn mean_z_score(&self) -> f64
	{
		self.roll_groups.iter().map(RollGroup::z_score).sum::<f64>()
			/ f64::from(self.roll_groups.len() as u32)
	}

	/// Returns the mean (average) [z-score](https://en.wikipedia.org/wiki/Standard_score) of all
	/// [`RollGroups`][`RollGroup`] in the [`DiceEvaluation`] scaled to a value in the interval [-1,
	/// 1] by dividing the result of [`Self::mean_z_score`] by the average
	/// [`PopulationData::min_z_score`] or [`PopulationData::max_z_score`] (depending on whether
	/// [`Self::mean_z_score`] is positive or negative)
	#[must_use]
	pub fn mean_z_score_normalized(&self) -> f64
	{
		let mean_z_score = self.mean_z_score();
		let scale_factor_key = match mean_z_score.partial_cmp(&0.0)
		{
			Some(Ordering::Less) => |group: &RollGroup| -group.population_data().min_z_score(),
			Some(Ordering::Greater) => |group: &RollGroup| group.population_data().max_z_score(),

			// early return 0.0 if mean z score is 0 (or NaN but the NaN case should never happen)
			// -morgan 2024-05-24
			_ => return 0.0,
		};

		let scale_factor = self.roll_groups.iter().map(scale_factor_key).sum::<f64>()
			/ f64::from(self.roll_groups.len() as u32);

		mean_z_score / scale_factor
	}
}

/// Represents information on the overall "population" of possible dice rolls which could result
/// from a particular amount of a particular type of dice
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PopulationData
{
	count: u32,
	faces: u32,
}
impl PopulationData
{
	/// Returns the mean (average) value of the population
	/// # Examples
	/// ```rust
	/// # use saikoro::evaluation::{Roll, RollGroup};
	/// # fn main() {
	/// // the actual roll values are unimportant, just that there are two (2) of them
	/// let rolls: [Roll; 2] = get_roll_values();
	/// // roll_group represents a roll of 2d6
	/// let roll_group = RollGroup::new(6, rolls);
	///
	/// // average value of 2d6 is 7
	/// assert_eq!(roll_group.population_data().mean(), 7.0);
	/// # }
	/// # fn get_roll_values() -> [Roll; 2] {
	/// # [1, 2].map(Roll::new)
	/// # }
	/// ```
	pub fn mean(&self) -> f64
	{
		f64::from(self.count) * f64::from(self.faces + 1) / 2.0
	}

	/// Returns the standard deviation of the population from the mean. (to get that mean, see
	/// [`mean`][PopulationData::mean])
	/// # Examples
	/// ```rust
	/// # use saikoro::evaluation::{Roll, RollGroup};
	/// # fn main() {
	/// // the actual roll values are unimportant, just that there are two (2) of them
	/// let rolls: [Roll; 2] = get_roll_values();
	/// // roll_group represents a roll of 2d6
	/// let roll_group = RollGroup::new(6, rolls);
	///
	/// // standard deviation from the mean of 2d6 is about 2.42
	/// assert_eq!(to_two_decimal_places(roll_group.population_data().stdev()), 2.42);
	/// # }
	/// # fn get_roll_values() -> [Roll; 2] {
	/// # [1, 2].map(Roll::new)
	/// # }
	/// #
	/// # fn to_two_decimal_places(val: f64) -> f64 {
	/// # (val * 100.0).round() / 100.0
	/// # }
	/// ```
	#[must_use]
	pub fn stdev(&self) -> f64
	{
		let mean = self.mean();
		let possibility_counts = self.ways_to_make_all_results();
		let variance =
			self.possible_rolls()
				.map(|value| {
					(f64::from(value) - mean).powi(2)
						* possibility_counts[(value - self.count) as usize]
				})
				.sum::<f64>() / self.population_size();

		variance.sqrt()
	}

	fn population_size(&self) -> f64
	{
		f64::from(self.faces).powf(f64::from(self.count))
	}

	fn possible_rolls(&self) -> RangeInclusive<u32>
	{
		self.count..=(self.faces * self.count)
	}

	fn ways_to_make_all_results(&self) -> Vec<f64>
	{
		let mut amortization_table = HashMap::with_capacity(
			(((self.faces * self.count) - self.count + 1) * self.count) as usize,
		);

		self.possible_rolls()
			.map(|roll| self.ways_to_make(roll, &mut amortization_table))
			.collect()
	}

	fn ways_to_make(&self, total: u32, amortization_table: &mut HashMap<(u32, u32), f64>) -> f64
	{
		Self::ways_to_make_rec(total, self.count, self.faces, amortization_table)
	}

	fn ways_to_make_rec(
		total: u32,
		count: u32,
		faces: u32,

		amortization_table: &mut HashMap<(u32, u32), f64>,
	) -> f64
	{
		amortization_table
			.get(&(total, count))
			.copied()
			.unwrap_or_else(|| {
				if count <= 1
				{
					(1..=faces).contains(&total).into()
				}
				else
				{
					let possibilities = (1..=faces)
						.filter_map(|final_roll| {
							total.checked_sub(final_roll).map(|previous_sum| {
								Self::ways_to_make_rec(
									previous_sum,
									count - 1,
									faces,
									amortization_table,
								)
							})
						})
						.sum::<f64>();

					amortization_table.insert((total, count), possibilities);
					possibilities
				}
			})
	}

	/// The minimum value in the population (equivalent to the number of dice rolled, as the minimum
	/// value occurs when all rolls are 1)
	pub fn min(&self) -> u32
	{
		self.count
	}
	/// The maximum value in the population (equivalent to the number of dice rolled times the
	/// number of faces per die, as the maximum value occurs when all rolls are the maximum)
	pub fn max(&self) -> u32
	{
		self.faces * self.count
	}

	/// Returns the [z-score](https://en.wikipedia.org/wiki/Standard_score) of the given `value` that is, its
	/// distance from the mean (see: [`mean`][PopulationData::mean]) in units of
	/// standard deviation (see: [`stdev`][PopulationData::stdev])
	/// # Examples
	/// ```rust
	/// # use saikoro::evaluation::{Roll, RollGroup};
	/// # fn main() {
	/// let roll_group = RollGroup::new(6, [4, 5].map(Roll::new));
	/// assert_eq!(to_two_decimal_places(roll_group.z_score()), 0.83)
	/// // roll_group total == 9
	/// // population mean == 7
	/// // population standard deviation ~= 2.42
	/// // z-score ~= (9 - 7) / 2.42 == 0.83
	/// # }
	/// # fn to_two_decimal_places(val: f64) -> f64 {
	/// # (val * 100.0).round() / 100.0
	/// # }
	/// ```
	#[must_use]
	pub fn z_score(&self, value: u32) -> f64
	{
		(f64::from(value) - self.mean()) / self.stdev()
	}

	/// Returns the [z-score](https://en.wikipedia.org/wiki/Standard_score) of the minimum value of the population
	#[must_use]
	pub fn min_z_score(&self) -> f64
	{
		self.z_score(self.min())
	}

	/// Returns the [z-score](https://en.wikipedia.org/wiki/Standard_score) of the maximum value of the population
	#[must_use]
	pub fn max_z_score(&self) -> f64
	{
		self.z_score(self.max())
	}

	/// Returns a tuple of the [z-scores](https://en.wikipedia.org/wiki/Standard_score) of the minimum and maximum
	/// values of the population respectively. It is preferable to use this function if you will (or
	/// even might) need both values. This is because it only calculates the standard deviation
	/// once, avoiding any performance hits that caluclation might incur. The actual calucations
	/// beyond the standard deviation are realtively cheap even if one is thrown away
	#[must_use]
	pub fn min_max_z_score(&self) -> (f64, f64)
	{
		let stdev = self.stdev();
		(
			(f64::from(self.min()) - self.mean()) / stdev,
			(f64::from(self.max()) - self.mean()) / stdev,
		)
	}
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod test
{
	use super::*;
	use crate::{evaluation::Roll, test_helpers::assert_approx_eq};

	#[test]
	fn mean()
	{
		assert_approx_eq!(7.0, PopulationData::new(2, 6).mean());
	}

	#[test]
	fn stdev()
	{
		assert_approx_eq!(2.41522945769824, PopulationData::new(2, 6).stdev());
	}

	#[test]
	fn z_score()
	{
		assert_approx_eq!(
			2.07019667802706,
			RollGroup::new(6, [6, 6].map(Roll::new)).z_score(),
			1e-14
		);
	}

	#[test]
	fn mean_z_score()
	{
		let groups = [
			RollGroup::new(6, [5, 3].map(Roll::new)),
			RollGroup::new(4, [3, 4].map(Roll::new)),
		];

		let evaluation = DiceEvaluation {
			value: groups.iter().map(|group| f64::from(group.total())).sum(),
			roll_groups: groups.into(),
		};

		assert_approx_eq!(0.839475199836382, evaluation.mean_z_score());
	}

	impl PopulationData
	{
		pub fn new(count: u32, faces: u32) -> Self
		{
			Self { count, faces }
		}
	}
}
