#![allow(missing_docs)]

mod evaluation_error;
mod parse_error;
mod tokenization_error;

pub use evaluation_error::*;
pub use parse_error::*;
pub use tokenization_error::*;

#[derive(Debug, thiserror::Error, Clone)]
pub enum SaikoroError
{
	#[error("{}", .0)]
	Evaluation(#[from] MissingSymbolError),

	#[error("{}", .0)]
	Parsing(#[from] ParsingError),
}
