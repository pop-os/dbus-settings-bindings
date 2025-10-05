// SPDX-License-Identifier: MPL-2.0
pub mod id;
pub mod ordering;
pub mod playlist;

use crate::{
	bindings::playlist::PlaylistsProxy,
	error::{Error, Result},
};
use std::ops::Deref;
use zbus::{Connection, names::OwnedBusName};

pub struct Playlists {
	proxy: PlaylistsProxy<'static>,
}

impl Playlists {
	/// Creates a new instance of the `org.mpris.MediaPlayer2.Playlists` interface.
	pub async fn new(connection: &Connection, name: OwnedBusName) -> Result<Self> {
		PlaylistsProxy::builder(connection)
			.destination(name)?
			.build()
			.await
			.map(Self::from)
			.map_err(Error::from)
	}
}

impl Deref for Playlists {
	type Target = PlaylistsProxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}

impl From<PlaylistsProxy<'static>> for Playlists {
	fn from(proxy: PlaylistsProxy<'static>) -> Self {
		Self { proxy }
	}
}
