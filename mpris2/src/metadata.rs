// SPDX-License-Identifier: MPL-2.0
use crate::error::{Error, Result};
use std::{
	collections::HashMap,
	fmt,
	ops::{Deref, DerefMut},
};
use time::{Duration, OffsetDateTime};
use zbus::zvariant::{OwnedObjectPath, Value as ZValue};

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
	inner: HashMap<String, MetadataValue>,
}

impl Metadata {
	/// `xesam:album`: The track artist(s).
	pub fn album(&self) -> Option<String> {
		self.inner
			.get("xesam:album")
			.cloned()
			.and_then(|v| v.try_into_string().ok())
	}

	/// `xesam:artist`: The track artist(s).
	pub fn artists(&self) -> Option<Vec<String>> {
		self.inner
			.get("xesam:artist")
			.cloned()
			.and_then(|artists| artists.try_into_array().ok())
			.map(|artists| {
				artists
					.into_iter()
					.filter_map(|v| v.try_into_string().ok())
					.collect()
			})
	}

	/// `xesam:asText`: The track lyrics.
	pub fn lyrics(&self) -> Option<String> {
		self.inner
			.get("xesam:asText")
			.cloned()
			.and_then(|v| v.try_into_string().ok())
	}

	/// `xesam:albumArtist`: The album artist(s).
	pub fn album_artists(&self) -> Option<Vec<String>> {
		self.inner
			.get("xesam:albumArtist")
			.cloned()
			.and_then(|artists| artists.try_into_array().ok())
			.map(|artists| {
				artists
					.into_iter()
					.filter_map(|v| v.try_into_string().ok())
					.collect()
			})
	}

	/// `xesam:audioBPM`: The speed of the music, in beats per minute.
	pub fn bpm(&self) -> Option<u64> {
		self.inner
			.get("xesam:audioBPM")
			.cloned()
			.and_then(|v| v.try_into_uint().ok())
	}

	/// `xesam:autoRating`: An automatically-generated rating, based on things such as how often it has been played.
	/// This should be in the range 0.0 to 1.0.
	pub fn auto_rating(&self) -> Option<f64> {
		self.inner
			.get("xesam:autoRating")
			.cloned()
			.and_then(|v| v.try_into_double().ok())
	}

	/// `xesam:composer`: The composer(s) of the track.
	pub fn composer(&self) -> Option<Vec<String>> {
		self.inner
			.get("xesam:composer")
			.cloned()
			.and_then(|artists| artists.try_into_array().ok())
			.map(|artists| {
				artists
					.into_iter()
					.filter_map(|v| v.try_into_string().ok())
					.collect()
			})
	}

	/// `xesam:contentCreated`: When the track was created. Usually only the year component will be useful.
	pub fn created(&self) -> Option<OffsetDateTime> {
		self.inner
			.get("xesam:contentCreated")
			.cloned()
			.and_then(|v| v.try_into_date().ok())
	}

	/// `xesam:discNumber`: The disc number on the album that this track is from.
	pub fn disc_number(&self) -> Option<u64> {
		self.inner
			.get("xesam:discNumber")
			.cloned()
			.and_then(|v| v.try_into_uint().ok())
	}

	/// `xesam:firstUsed`: When the track was first played.
	pub fn first_played(&self) -> Option<OffsetDateTime> {
		self.inner
			.get("xesam:firstUsed")
			.cloned()
			.and_then(|v| v.try_into_date().ok())
	}

	/// `xesam:genre`: The genre(s) of the track.
	pub fn genre(&self) -> Option<Vec<String>> {
		self.inner
			.get("xesam:genre")
			.cloned()
			.and_then(|artists| artists.try_into_array().ok())
			.map(|artists| {
				artists
					.into_iter()
					.filter_map(|v| v.try_into_string().ok())
					.collect()
			})
	}

	/// `xesam:lastUsed`: When the track was last played.
	pub fn last_played(&self) -> Option<OffsetDateTime> {
		self.inner
			.get("xesam:lastUsed")
			.cloned()
			.and_then(|v| v.try_into_date().ok())
	}

	/// `xesam:lyricist`: The lyricist(s) of the track.
	pub fn lyricist(&self) -> Option<Vec<String>> {
		self.inner
			.get("xesam:lyricist")
			.cloned()
			.and_then(|artists| artists.try_into_array().ok())
			.map(|artists| {
				artists
					.into_iter()
					.filter_map(|v| v.try_into_string().ok())
					.collect()
			})
	}

	/// `xesam:title`: The track title.
	pub fn title(&self) -> Option<String> {
		self.inner
			.get("xesam:title")
			.cloned()
			.and_then(|v| v.try_into_string().ok())
	}

	/// `xesam:trackNumber`: The track number on the album that this track is from.
	pub fn track_number(&self) -> Option<u64> {
		self.inner
			.get("xesam:trackNumber")
			.cloned()
			.and_then(|v| v.try_into_uint().ok())
	}

	/// `xesam:url`: The location of the media file.
	pub fn url(&self) -> Option<String> {
		self.inner
			.get("xesam:url")
			.cloned()
			.and_then(|v| v.try_into_string().ok())
	}

	/// `xesam:useCount`: The number of times the track has been played.
	pub fn use_count(&self) -> Option<u64> {
		self.inner
			.get("xesam:useCount")
			.cloned()
			.and_then(|v| v.try_into_uint().ok())
	}

	/// `xesam:userRating`: The user's rating of the track.
	pub fn user_rating(&self) -> Option<f64> {
		self.inner
			.get("xesam:userRating")
			.cloned()
			.and_then(|v| v.try_into_double().ok())
	}

	/// `mpris:trackid`: D-Bus path: A unique identity for this track within the context of an MPRIS object (eg: tracklist).
	pub fn track_id(&self) -> Option<OwnedObjectPath> {
		self.inner
			.get("mpris:trackid")
			.cloned()
			.and_then(|v| v.try_into_string().ok())
			.and_then(|path| OwnedObjectPath::try_from(path).ok())
	}

	/// `mpris:length`: The length of the track in microseconds.
	pub fn length(&self) -> Option<Duration> {
		self.inner
			.get("mpris:length")
			.cloned()
			.and_then(|v| match &v {
				MetadataValue::Int(i) => Some(*i),
				MetadataValue::UInt(u) => Some(*u as i64),
				MetadataValue::Str(s) => s.parse().ok(),
				_ => None,
			})
			.map(Duration::microseconds)
	}

	/// `mpris:artUrl`: The location of an image representing the track or album.
	/// Clients should not assume this will continue to exist when the media player stops giving out the URL.
	pub fn art_url(&self) -> Option<String> {
		self.inner
			.get("mpris:artUrl")
			.cloned()
			.and_then(|v| v.try_into_string().ok())
	}
}

impl Deref for Metadata {
	type Target = HashMap<String, MetadataValue>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Metadata {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl fmt::Display for Metadata {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{{")?;
		let mut iter = self.inner.iter().peekable();
		while let Some((k, v)) = iter.next() {
			if iter.peek().is_some() {
				write!(f, "{}: {}, ", k, v)?;
			} else {
				write!(f, "{}: {}", k, v)?;
			}
		}
		write!(f, "}}")
	}
}

impl<'a, V: Into<ZValue<'a>>> From<HashMap<String, V>> for Metadata {
	fn from(map: HashMap<String, V>) -> Self {
		Self {
			inner: map
				.into_iter()
				.map(|(k, v)| (k, MetadataValue::from(&v.into())))
				.collect(),
		}
	}
}

#[derive(Clone, PartialEq)]
pub enum MetadataValue {
	Str(String),
	Double(f64),
	Int(i64),
	UInt(u64),
	Bool(bool),
	Array(Vec<MetadataValue>),
	Dict(HashMap<String, MetadataValue>),
	__Unsupported,
}

impl MetadataValue {
	fn variant(&self) -> &'static str {
		match self {
			MetadataValue::Str(_) => "Str",
			MetadataValue::Double(_) => "Double",
			MetadataValue::Int(_) => "Int",
			MetadataValue::UInt(_) => "UInt",
			MetadataValue::Bool(_) => "Bool",
			MetadataValue::Array(_) => "Array",
			MetadataValue::Dict(_) => "Dict",
			MetadataValue::__Unsupported => "Unsupported",
		}
	}

	/// Tries to extract a string from the variant,
	/// returning an error if the variant is not a string.
	pub fn try_into_string(self) -> Result<String> {
		match self {
			MetadataValue::Str(s) => Ok(s),
			_ => Err(Error::IncorrectVariant {
				wanted: "Str",
				actual: self.variant(),
			}),
		}
	}

	/// Tries to extract a string from the variant,
	/// panicking if the variant is not a string.
	pub fn into_string(self) -> String {
		self.try_into_string()
			.unwrap_or_else(|err| panic!("{}", err))
	}

	/// Tries to extract a date/time from the variant,
	/// returning an error if the variant is not a date/time.
	pub fn try_into_date(self) -> Result<OffsetDateTime> {
		let variant = self.variant();
		match self {
			MetadataValue::Str(s) => {
				OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339).map_err(
					|_| Error::IncorrectVariant {
						wanted: "String (DateTime)",
						actual: variant,
					},
				)
			}
			_ => Err(Error::IncorrectVariant {
				wanted: "String (DateTime)",
				actual: variant,
			}),
		}
	}

	/// Tries to extract a date/time from the variant,
	/// panicking if the variant is not a date/time.
	pub fn into_date(self) -> OffsetDateTime {
		self.try_into_date().unwrap_or_else(|err| panic!("{}", err))
	}

	/// Tries to extract a double from the variant,
	/// returning an error if the variant is not a double.
	pub fn try_into_double(self) -> Result<f64> {
		match self {
			MetadataValue::Double(d) => Ok(d),
			_ => Err(Error::IncorrectVariant {
				wanted: "Double",
				actual: self.variant(),
			}),
		}
	}

	/// Tries to extract a double from the variant,
	/// panicking if the variant is not a double.
	pub fn into_double(self) -> f64 {
		self.try_into_double()
			.unwrap_or_else(|err| panic!("{}", err))
	}

	/// Tries to extract an integer from the variant,
	/// returning an error if the variant is not an integer.
	pub fn try_into_int(self) -> Result<i64> {
		match self {
			MetadataValue::Int(i) => Ok(i),
			_ => Err(Error::IncorrectVariant {
				wanted: "Int",
				actual: self.variant(),
			}),
		}
	}

	/// Tries to extract an integer from the variant,
	/// panicking if the variant is not an integer.
	pub fn into_int(self) -> i64 {
		self.try_into_int().unwrap_or_else(|err| panic!("{}", err))
	}

	/// Tries to extract an unsigned integer from the variant,
	/// returning an error if the variant is not an unsigned integer.
	pub fn try_into_uint(self) -> Result<u64> {
		match self {
			MetadataValue::UInt(u) => Ok(u),
			_ => Err(Error::IncorrectVariant {
				wanted: "UInt",
				actual: self.variant(),
			}),
		}
	}

	/// Tries to extract an unsigned integer from the variant,
	/// panicking if the variant is not an unsigned integer.
	pub fn into_uint(self) -> u64 {
		self.try_into_uint().unwrap_or_else(|err| panic!("{}", err))
	}

	/// Tries to extract a boolean from the variant,
	/// returning an error if the variant is not a boolean.
	pub fn try_into_bool(self) -> Result<bool> {
		match self {
			MetadataValue::Bool(b) => Ok(b),
			_ => Err(Error::IncorrectVariant {
				wanted: "Bool",
				actual: self.variant(),
			}),
		}
	}

	/// Tries to extract a boolean from the variant,
	/// panicking if the variant is not a boolean.
	pub fn into_bool(self) -> bool {
		self.try_into_bool().unwrap_or_else(|err| panic!("{}", err))
	}

	/// Tries to extract an array from the variant,
	/// returning an error if the variant is not an array.
	pub fn try_into_array(self) -> Result<Vec<MetadataValue>> {
		match self {
			MetadataValue::Array(a) => Ok(a),
			_ => Err(Error::IncorrectVariant {
				wanted: "Array",
				actual: self.variant(),
			}),
		}
	}

	/// Tries to extract an array from the variant,
	/// panicking if the variant is not an array.
	pub fn into_array(self) -> Vec<MetadataValue> {
		self.try_into_array()
			.unwrap_or_else(|err| panic!("{}", err))
	}

	/// Tries to extract a dictionary from the variant,
	/// returning an error if the variant is not a dictionary.
	/// The dictionary is returned as a map from string keys to values.
	pub fn try_into_dict(self) -> Result<HashMap<String, MetadataValue>> {
		match self {
			MetadataValue::Dict(d) => Ok(d),
			_ => Err(Error::IncorrectVariant {
				wanted: "Dict",
				actual: self.variant(),
			}),
		}
	}

	/// Tries to extract a dictionary from the variant,
	/// panicking if the variant is not a dictionary.
	/// The dictionary is returned as a map from string keys to values.
	pub fn into_dict(self) -> HashMap<String, MetadataValue> {
		self.try_into_dict().unwrap_or_else(|err| panic!("{}", err))
	}
}

impl<'a> From<&ZValue<'a>> for MetadataValue {
	fn from(value: &ZValue) -> Self {
		match value.try_clone() {
			Ok(ZValue::U8(u)) => Self::UInt(u as u64),
			Ok(ZValue::Bool(b)) => Self::Bool(b),
			Ok(ZValue::I16(i)) => Self::Int(i as i64),
			Ok(ZValue::U16(u)) => Self::UInt(u as u64),
			Ok(ZValue::I32(i)) => Self::Int(i as i64),
			Ok(ZValue::U32(u)) => Self::UInt(u as u64),
			Ok(ZValue::I64(i)) => Self::Int(i),
			Ok(ZValue::U64(u)) => Self::UInt(u),
			Ok(ZValue::F64(f)) => Self::Double(f),
			Ok(ZValue::Str(s)) => Self::Str(s.to_string()),
			Ok(ZValue::ObjectPath(path)) => Self::Str(path.to_string()),
			Ok(ZValue::Array(a)) => Self::Array(a.iter().map(|v| v.into()).collect()),
			Ok(ZValue::Dict(d)) => Self::Dict(
				HashMap::<String, ZValue>::try_from(d)
					.unwrap()
					.into_iter()
					.map(|(k, v)| (k, (&v).into()))
					.collect(),
			),
			Ok(ZValue::Value(value)) => Self::from(&*value),
			_ => Self::__Unsupported,
		}
	}
}

impl fmt::Debug for MetadataValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::__Unsupported => write!(f, "__Unsupported"),
			Self::Int(i) => write!(f, "{}", i),
			Self::UInt(u) => write!(f, "{}", u),
			Self::Double(d) => write!(f, "{}", d),
			Self::Str(s) => write!(f, "{}", s),
			Self::Bool(b) => write!(f, "{}", b),
			Self::Array(a) => write!(f, "{:?}", a),
			Self::Dict(d) => {
				let mut debug_struct = f.debug_struct("Dict");
				for (k, v) in d {
					debug_struct.field(k, &v);
				}
				debug_struct.finish()
			}
		}
	}
}

impl fmt::Display for MetadataValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::__Unsupported => write!(f, "__Unsupported"),
			Self::Int(i) => write!(f, "{}", i),
			Self::UInt(u) => write!(f, "{}", u),
			Self::Double(d) => write!(f, "{}", d),
			Self::Str(s) => write!(f, "\"{}\"", s),
			Self::Bool(b) => write!(f, "{}", b),
			Self::Array(a) => {
				write!(f, "[")?;
				let mut iter = a.iter().peekable();
				while let Some(value) = iter.next() {
					if iter.peek().is_some() {
						write!(f, "{}, ", value)?;
					} else {
						write!(f, "{}", value)?;
					}
				}
				write!(f, "]")
			}
			Self::Dict(d) => {
				write!(f, "{{")?;
				let mut iter = d.iter().peekable();
				while let Some((k, v)) = iter.next() {
					if iter.peek().is_some() {
						write!(f, "{}: {}, ", k, v)?;
					} else {
						write!(f, "{}: {}", k, v)?;
					}
				}
				write!(f, "}}")
			}
		}
	}
}
