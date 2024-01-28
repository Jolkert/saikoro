use crate::evaluation::{DiceEvaluation, RollGroup};

impl RollGroup
{
	fn expression(&self) -> DiceExpression
	{
		DiceExpression {
			count: self.len() as u32,
			faces: self.faces,
		}
	}

	pub fn mean(&self) -> f64
	{
		f64::from(self.total())
			/ f64::from(self.iter().filter(|it| !it.is_removed()).count() as u32)
	}

	pub fn population_mean(&self) -> f64
	{
		self.expression().mean()
	}
	pub fn population_stdev(&self) -> f64
	{
		self.expression().stdev()
	}

	pub fn z_score(&self) -> f64
	{
		let population = self.expression();
		(f64::from(self.total()) - population.mean()) / population.stdev()
	}

	pub fn is_max_roll(&self) -> bool
	{
		self.iter().all(|roll| roll.original_value >= self.faces)
	}
	pub fn is_min_roll(&self) -> bool
	{
		self.iter().all(|roll| roll.original_value <= 1)
	}
}

struct RollPopulationIter
{
	faces: u32,
	vec: Vec<u32>,
}
impl RollPopulationIter
{
	fn new(count: u32, faces: u32) -> Self
	{
		let mut starting_state = vec![1; count as usize];
		starting_state[0] = 0;

		Self {
			faces,
			vec: starting_state,
		}
	}
}
impl Iterator for RollPopulationIter
{
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item>
	{
		for i in 0..self.vec.len()
		{
			let new_val = self.vec[i] + 1;
			if new_val > self.faces && i >= self.vec.len() - 1
			{
				return None;
			}
			self.vec[i] = if new_val <= self.faces { new_val } else { 1 };
			if new_val <= self.faces
			{
				break;
			}
		}

		Some(self.vec.iter().sum())
	}
}

impl DiceEvaluation
{
	pub fn mean_z_score(&self) -> f64
	{
		self.roll_groups.iter().map(RollGroup::z_score).sum::<f64>()
			/ f64::from(self.roll_groups.len() as u32)
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct DiceExpression
{
	count: u32,
	faces: u32,
}
impl DiceExpression
{
	fn mean(self) -> f64
	{
		f64::from(self.count) * f64::from(self.faces + 1) / 2.0
	}
	fn stdev(self) -> f64
	{
		let mean = self.mean();
		let variance =
			self.population_iter()
				.map(|val| f64::powi(f64::from(val) - mean, 2))
				.sum::<f64>() / f64::from(self.population_size());

		f64::sqrt(variance)
	}
	fn population_size(self) -> u32
	{
		self.faces.pow(self.count)
	}

	fn population_iter(self) -> impl Iterator<Item = u32>
	{
		RollPopulationIter::new(self.count, self.faces)
	}
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod test
{
	use super::*;
	use crate::{evaluation::Roll, test_helpers::assert_approx_eq};

	#[test]
	fn population_iter()
	{
		assert_eq!(
			RollPopulationIter::new(1, 6).collect::<Vec<_>>(),
			vec![1, 2, 3, 4, 5, 6]
		);

		assert_eq!(
			RollPopulationIter::new(2, 4).collect::<Vec<_>>(),
			vec![2, 3, 4, 5, 3, 4, 5, 6, 4, 5, 6, 7, 5, 6, 7, 8]
		);

		assert_eq!(
			sorted(RollPopulationIter::new(2, 6).collect::<Vec<_>>()),
			vec![
				2, 3, 3, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 9, 9,
				9, 9, 10, 10, 10, 11, 11, 12
			]
		);

		assert_eq!(
			sorted(RollPopulationIter::new(3, 4).collect::<Vec<_>>()),
			vec![
				3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7,
				7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 10,
				10, 10, 10, 10, 10, 11, 11, 11, 12
			]
		);
	}

	fn sorted<T: Ord>(mut vec: Vec<T>) -> Vec<T>
	{
		vec.sort_unstable();
		vec
	}

	#[test]
	fn mean()
	{
		assert_approx_eq!(7.0, DiceExpression::new(2, 6).mean());
	}

	#[test]
	fn stdev()
	{
		assert_approx_eq!(2.41522945769824, DiceExpression::new(2, 6).stdev());
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

	impl DiceExpression
	{
		pub fn new(count: u32, faces: u32) -> Self
		{
			Self { count, faces }
		}
	}
}
