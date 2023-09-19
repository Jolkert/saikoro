// this is probably *not* best practice but until i
// figure something else out -morgan 2023-09-19
#[derive(Debug)]
pub enum Error
{
	InvalidToken,
	InvalidOperand,
	MissingOperand
	{
		expected: u32,
		found: u32,
	},
}
