// SPDX-License-Identifier: MPL-2.0
use super::id::PlaylistId;
use serde::{Deserialize, Serialize};
use zbus::{Type, Variant};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Type, Variant, Serialize, Deserialize)]
pub struct Playlist((PlaylistId, String, String));

impl Playlist {
	pub fn id(&self) -> &PlaylistId {
		&self.0.0
	}

	pub fn name(&self) -> &str {
		&self.0.1
	}

	pub fn icon(&self) -> &str {
		&self.0.2
	}
}
