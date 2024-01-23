use crate::evaluation::RollGroup;

impl RollGroup
{
	fn expression(&self) -> DiceExpression
	{
		DiceExpression {
			count: self.len() as u32,
			faces: self.faces,
		}
	}

	pub fn pop_mean(&self) -> f64
	{
		self.expression().mean()
	}
	pub fn pop_stdev(&self) -> f64
	{
		self.expression().stdev()
	}

	pub fn z_score(&self) -> f64
	{
		let population = self.expression();
		(f64::from(self.total()) - population.mean()) / population.stdev()
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct DiceExpression
{
	count: u32,
	faces: u32,
}
impl DiceExpression
{
	pub fn mean(&self) -> f64
	{
		f64::from(self.count) * f64::from(self.faces + 1) / 2.0
	}
	pub fn stdev(&self) -> f64
	{
		let mean = self.mean();
		let variance =
			self.population_iter()
				.map(|val| f64::powi(f64::from(val) - mean, 2))
				.sum::<f64>() / f64::from(self.population_size());

		f64::sqrt(variance)
	}
	pub fn population_size(&self) -> u32
	{
		self.faces.pow(self.count)
	}

	fn population_iter(&self) -> impl Iterator<Item = u32>
	{
		RollPopulationIter::new(self.count, self.faces)
	}
}

#[cfg(test)]
mod test
{
	use super::*;
	use crate::{evaluation::Roll, test_helpers::assert_approx_eq};

	#[test]
	fn population_iter()
	{
		assert_eq!(
			RollPopulationIter::new(2, 4).collect::<Vec<_>>(),
			vec![2, 3, 4, 5, 3, 4, 5, 6, 4, 5, 6, 7, 5, 6, 7, 8]
		);

		let mut two_d_six = RollPopulationIter::new(2, 6).collect::<Vec<_>>();
		two_d_six.sort();

		assert_eq!(
			two_d_six,
			vec![
				2, 3, 3, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 9, 9,
				9, 9, 10, 10, 10, 11, 11, 12
			]
		)
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
		assert_approx_eq!(2.41522945769824, DiceExpression::new(2, 6).stdev());
	}

	#[test]
	fn z_score()
	{
		let absolute_error =
			(RollGroup::new(6, [6, 6].map(Roll::new)).z_score() - 2.07019667802706).abs();
		assert!(absolute_error < 1e-14);
	}

	impl DiceExpression
	{
		pub fn new(count: u32, faces: u32) -> Self
		{
			Self { count, faces }
		}
	}
}
