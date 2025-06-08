use std::{marker::PhantomData, str::FromStr};

use serde::{
	Deserialize, Deserializer,
	de::{MapAccess, Visitor},
};

use crate::{error::ParsingError, parsing::Node};

impl FromStr for Node
{
	type Err = ParsingError;

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		crate::parsing::parse_string(s).map(|tree| tree.head)
	}
}

// yoinked from https://serde.rs/string-or-struct.html
// -morgan 2025-06-07
pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	T: Deserialize<'de> + FromStr<Err = ParsingError>,
	D: Deserializer<'de>,
{
	// This is a Visitor that forwards string types to T's `FromStr` impl and
	// forwards map types to T's `Deserialize` impl. The `PhantomData` is to
	// keep the compiler from complaining about T being an unused generic type
	// parameter. We need T in order to know the Value type for the Visitor
	// impl.
	struct StringOrStruct<T>(PhantomData<fn() -> T>);

	impl<'de, T> Visitor<'de> for StringOrStruct<T>
	where
		T: Deserialize<'de> + FromStr<Err = ParsingError>,
	{
		type Value = T;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result
		{
			formatter.write_str("string or map")
		}

		fn visit_str<E>(self, value: &str) -> Result<T, E>
		where
			E: serde::de::Error,
		{
			Ok(FromStr::from_str(value).unwrap())
		}

		fn visit_map<M>(self, map: M) -> Result<T, M::Error>
		where
			M: MapAccess<'de>,
		{
			// `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
			// into a `Deserializer`, allowing it to be used as the input to T's
			// `Deserialize` implementation. T then deserializes itself using
			// the entries from the map visitor.
			Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
		}
	}

	deserializer.deserialize_any(StringOrStruct(PhantomData))
}
