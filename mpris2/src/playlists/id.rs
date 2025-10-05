// SPDX-License-Identifier: MPL-2.0
use serde::{Deserialize, Serialize};
use std::{
	cmp::{Ord, Ordering, PartialOrd},
	fmt::{self, Display},
	ops::Deref,
};
use zvariant::{ObjectPath, OwnedObjectPath, Type, Value};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Type, Serialize, Deserialize, Value)]
pub struct PlaylistId(OwnedObjectPath);

impl PlaylistId {
	pub fn into_inner(self) -> OwnedObjectPath {
		self.0
	}

	pub fn into_static_path(self) -> ObjectPath<'static> {
		self.0.into_inner().into_owned()
	}
}

impl Deref for PlaylistId {
	type Target = OwnedObjectPath;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> AsRef<ObjectPath<'a>> for PlaylistId {
	fn as_ref(&self) -> &ObjectPath<'a> {
		&self.0
	}
}

impl PartialOrd for PlaylistId {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PlaylistId {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.as_str().cmp(other.0.as_str())
	}
}

impl Display for PlaylistId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0.as_str())
	}
}
