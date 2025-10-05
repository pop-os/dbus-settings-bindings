// SPDX-License-Identifier: MPL-2.0

pub mod connection;

use self::connection::Connection;
use crate::interface::settings::{SettingsProxy, connection::ConnectionSettingsProxy};
use std::ops::Deref;
use zbus::Result;

#[derive(Debug)]
pub struct NetworkManagerSettings<'a>(SettingsProxy<'a>);

impl<'a> Deref for NetworkManagerSettings<'a> {
	type Target = SettingsProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> NetworkManagerSettings<'a> {
	pub async fn new(connection: &'a zbus::Connection) -> Result<NetworkManagerSettings<'a>> {
		SettingsProxy::new(connection).await.map(Self)
	}

	pub async fn list_connections(&'a self) -> Result<Vec<Connection<'a>>> {
		let connections = self.0.list_connections().await?;
		let mut out = Vec::with_capacity(connections.len());
		for connection in connections {
			let connection = ConnectionSettingsProxy::builder(self.0.inner().connection())
				.path(connection)?
				.build()
				.await?;
			out.push(connection.into());
		}
		Ok(out)
	}
}
