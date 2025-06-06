use std::collections::HashMap;

use crate::{parsing, tokenization::TokenStream, Node, ParsingError};

#[derive(Debug, Clone)]
pub struct SymbolTable(HashMap<String, Node>);

impl SymbolTable
{
	pub fn new() -> Self
	{
		Self(HashMap::<String, Node>::new())
	}

	pub fn insert(&mut self, k: &str, v: &str) -> Result<(), ParsingError>
	{
		parsing::parse_tree_from(&mut TokenStream::new(v)).map(|subtree| {
			self.0.insert(k.to_string(), subtree);
		})
	}

	pub fn get(&self, k: &String) -> Option<Node>
	{
		self.0.get(k).cloned()
	}
}

impl Default for SymbolTable
{
	fn default() -> Self
	{
		Self::new()
	}
}

impl TryFrom<HashMap<String, String>> for SymbolTable
{
	type Error = ParsingError;

	fn try_from(str_map: HashMap<String, String>) -> Result<Self, Self::Error>
	{
		let mut result = Self::new();

		for (sym, exp) in str_map
		{
			result.insert(&sym, &exp)?;
		}

		Ok(result)
	}
}
