// SPDX-License-Identifier: MPL-2.0
use crate::{
	bindings::{
		media_player::MediaPlayer2Proxy, player::PlayerProxy, playlist::PlaylistsProxy,
		track_list::TrackListProxy,
	},
	enumerator::Enumerator,
	error::{Error, Result},
	player::Player,
	playlists::Playlists,
	track_list::TrackList,
};
use std::ops::Deref;
use zbus::{Connection, names::OwnedBusName};

#[derive(Debug, Clone)]
pub struct MediaPlayer {
	proxy: MediaPlayer2Proxy<'static>,
}

impl MediaPlayer {
	/// Creates a new instance of the `org.mpris.MediaPlayer2` interface.
	pub async fn new(connection: &Connection, name: OwnedBusName) -> Result<Self> {
		MediaPlayer2Proxy::builder(connection)
			.destination(name)?
			.build()
			.await
			.map(Self::from)
			.map_err(Error::from)
	}

	/// Gets the names of all the MPRIS players that are available on the current session.
	pub async fn available_players(connection: &Connection) -> Result<Vec<OwnedBusName>> {
		Ok(Enumerator::new(connection).await?.players().await?)
	}

	/// Gets a new instance of all the MPRIS players that are available on the current session.
	pub async fn new_all(connection: &Connection) -> Result<Vec<Self>> {
		let players = Self::available_players(connection).await?;
		let mut instances = Vec::with_capacity(players.len());
		for player in players {
			instances.push(Self::new(connection, player).await?);
		}
		Ok(instances)
	}

	/// Returns an instance to the `org.mpris.MediaPlayer2.Player` interface of this object.
	pub async fn player(&self) -> Result<Player> {
		PlayerProxy::builder(self.proxy.inner().connection())
			.destination(self.proxy.inner().destination().to_owned())?
			.build()
			.await
			.map(Player::from)
			.map_err(Error::from)
	}

	/// Returns an instance to the `org.mpris.MediaPlayer2.TrackList` interface of this object,
	/// if a track list is available.
	pub async fn track_list(&self) -> Result<Option<TrackList>> {
		if self.proxy.has_track_list().await? {
			TrackListProxy::builder(self.proxy.inner().connection())
				.destination(self.proxy.inner().destination().to_owned())?
				.build()
				.await
				.map(TrackList::from)
				.map(Some)
				.map_err(Error::from)
		} else {
			Ok(None)
		}
	}

	/// Returns an instance to the `org.mpris.MediaPlayer2.Playlists` interface of this object,
	/// if a track list is available.
	pub async fn playlists(&self) -> Result<Option<Playlists>> {
		if self.proxy.has_track_list().await? {
			PlaylistsProxy::builder(self.proxy.inner().connection())
				.destination(self.proxy.inner().destination().to_owned())?
				.build()
				.await
				.map(Playlists::from)
				.map(Some)
				.map_err(Error::from)
		} else {
			Ok(None)
		}
	}
}

impl Deref for MediaPlayer {
	type Target = MediaPlayer2Proxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}

impl From<MediaPlayer2Proxy<'static>> for MediaPlayer {
	fn from(proxy: MediaPlayer2Proxy<'static>) -> Self {
		Self { proxy }
	}
}
