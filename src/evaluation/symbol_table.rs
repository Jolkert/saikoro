use std::collections::HashMap;
use crate::{parsing, tokenization::TokenStream, ParsingError, Node};

pub struct SymbolTable(HashMap<String, Node>);

impl SymbolTable {
 pub fn new() -> Self {
	SymbolTable(HashMap::<String, Node>::new())
 }

 pub fn insert(&mut self, k: &str, v: &str) -> Result<(), ParsingError>{
	parsing::parse_tree_from(&mut TokenStream::new(v))
    .map(|subtree| {
      self.0.insert(k.to_string(), subtree);
    })
 }

 pub fn get(&self, k: &String) -> Option<Node>{
	self.0.get(k).cloned()
 }

}

impl Default for SymbolTable {
	fn default() -> Self {
		Self::new()
	}
}

impl TryFrom<HashMap<String, String>> for SymbolTable {

	type Error = ParsingError;

	fn try_from(str_map: HashMap<String, String>) -> Result<Self, Self::Error> {
		let mut result = SymbolTable::new();

		for (sym, exp) in str_map {
			result.insert(&sym, &exp)?
		}

		Ok(result)
	}
}
