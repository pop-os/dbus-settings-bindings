// SPDX-License-Identifier: MPL-2.0
use crate::error::{Error, Result};
use serde::{
	de::{self, Deserialize, Visitor},
	ser::{Serialize, Serializer},
};
use std::{
	fmt::{self, Display},
	str::FromStr,
};
use zvariant::{Signature, Type, Value};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlaylistOrdering {
	/// Alphabetical ordering by name, ascending.
	Alphabetical,
	/// Ordering by creation date, oldest first.
	CreationDate,
	/// Ordering by last modified date, oldest first.
	ModifiedDate,
	/// Ordering by date of last playback, oldest first.
	LastPlayDate,
	/// A user-defined ordering.
	UserDefined,
}

impl Type for PlaylistOrdering {
	const SIGNATURE: &'static Signature = String::SIGNATURE;
}

impl<'a> TryFrom<Value<'a>> for PlaylistOrdering {
	type Error = Error;

	fn try_from(value: Value<'a>) -> Result<Self> {
		match value {
			Value::Str(value) => Self::from_str(&value),
			_ => Err(Error::IncorrectValue {
				wanted: "Str",
				actual: value
					.try_to_owned()
					.map_err(|e| Error::Zbus(zbus::Error::Variant(e)))?,
			}),
		}
	}
}

impl<'a> From<PlaylistOrdering> for Value<'a> {
	fn from(ordering: PlaylistOrdering) -> Self {
		Value::Str(ordering.to_string().into())
	}
}

impl FromStr for PlaylistOrdering {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		match s.to_lowercase().trim() {
			"alphabetical" => Ok(Self::Alphabetical),
			"created" => Ok(Self::CreationDate),
			"modified" => Ok(Self::ModifiedDate),
			"played" => Ok(Self::LastPlayDate),
			"user" => Ok(Self::UserDefined),
			_ => Err(Error::InvalidEnum {
				got: s.to_string(),
				expected: &["Alphabetical", "Created", "Modified", "Played", "User"],
			}),
		}
	}
}

impl Display for PlaylistOrdering {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Alphabetical => "Alphabetical",
				Self::CreationDate => "Created",
				Self::ModifiedDate => "Modified",
				Self::LastPlayDate => "Played",
				Self::UserDefined => "User",
			}
		)
	}
}

impl Serialize for PlaylistOrdering {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(self.to_string().as_str())
	}
}

impl<'de> Deserialize<'de> for PlaylistOrdering {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: de::Deserializer<'de>,
	{
		deserializer.deserialize_str(PlaylistOrderingVisitor)
	}
}

struct PlaylistOrderingVisitor;

impl Visitor<'_> for PlaylistOrderingVisitor {
	type Value = PlaylistOrdering;

	fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_str("a string")
	}

	fn visit_str<E>(self, s: &str) -> std::result::Result<Self::Value, E>
	where
		E: de::Error,
	{
		PlaylistOrdering::from_str(s).map_err(de::Error::custom)
	}
}
