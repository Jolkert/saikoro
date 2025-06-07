#[derive(Debug, thiserror::Error, Clone)]
#[error("Could not find symbol {{{}}} in symbol table", .expected_symbol)]
pub struct MissingSymbolError
{
	expected_symbol: String,
}

impl From<String> for MissingSymbolError
{
	fn from(value: String) -> Self
	{
		Self {
			expected_symbol: value,
		}
	}
}
