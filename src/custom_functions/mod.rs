use crate::{RangeRng, evaluation::Operand};

pub struct SaikoroFunction
{
	name: String,
	num_args: u8,
	function: fn(Vec<Operand>, &mut Box<dyn RangeRng>) -> Operand,
}

// TODO: proc macro for generating `SaikoroFunction`s from plain rust functions
