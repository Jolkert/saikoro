use std::collections::HashMap;

use crate::{Node, ParsingError, parsing, tokenization::TokenStream};

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

	pub fn get(&self, k: &String) -> Option<&Node>
	{
		self.0.get(k)
	}
}

impl Default for SymbolTable
{
	fn default() -> Self
	{
		Self::new()
	}
}

impl From<HashMap<String, Node>> for SymbolTable
{
	fn from(value: HashMap<String, Node>) -> Self
	{
		Self(value)
	}
}

impl TryFrom<HashMap<String, String>> for SymbolTable
{
	type Error = ParsingError;

	fn try_from(str_map: HashMap<String, String>) -> Result<Self, Self::Error>
	{
		str_map
			.into_iter()
			.map(|(symbol, expr)| {
				Ok((
					symbol,
					parsing::parse_tree_from(&mut TokenStream::new(&expr))?,
				))
			})
			.collect::<Result<HashMap<_, _>, _>>()
			.map(Self::from)
	}
}
