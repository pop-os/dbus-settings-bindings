// SPDX-License-Identifier: MPL-2.0

use futures_util::{
	Stream,
	stream::{self, StreamExt},
};
use zbus::{Connection, fdo::DBusProxy, names::OwnedBusName};

#[derive(Clone, Debug)]
pub enum Event {
	Remove(OwnedBusName),
	Add(OwnedBusName),
}

/// Helper to list mpris players on DBus bus, and watch for addition/removal.
///
/// Uses `org.freedesktop.DBus` to watch for clients that claim names starting with
/// `org.mpris.MediaPlayer2.`
pub struct Enumerator {
	proxy: DBusProxy<'static>,
}

impl Enumerator {
	pub async fn new(connection: &Connection) -> zbus::Result<Self> {
		Ok(Self {
			proxy: DBusProxy::builder(connection)
				.path("/org/freedesktop/DBus")?
				.build()
				.await?,
		})
	}

	/// Returns a stream that is signalled when an mpris client is added or removed
	pub async fn receive_changes(
		&self,
	) -> zbus::Result<impl Stream<Item = zbus::Result<Event>> + Unpin> {
		let stream = self.proxy.receive_name_owner_changed().await?;
		Ok(stream
			.filter_map(|signal| {
				Box::pin(async move {
					let args = match signal.args() {
						Ok(args) => args,
						Err(err) => {
							return Some(stream::iter(Some(Err(err)).into_iter().chain(None)));
						}
					};
					if args.name().contains("org.mpris.MediaPlayer2.") {
						let remove = args
							.old_owner
							.as_ref()
							.map(|_| Ok(Event::Remove(args.name().to_owned().into())));
						let add = args
							.new_owner
							.as_ref()
							.map(|_| Ok(Event::Add(args.name().to_owned().into())));
						Some(stream::iter(remove.into_iter().chain(add)))
					} else {
						None
					}
				})
			})
			.flatten())
	}

	/// Get names of all mpris players currently on the bus
	pub async fn players(&self) -> zbus::Result<Vec<OwnedBusName>> {
		let mut players = self.proxy.list_names().await?;
		players.retain(|name| name.starts_with("org.mpris.MediaPlayer2."));
		Ok(players)
	}
}
