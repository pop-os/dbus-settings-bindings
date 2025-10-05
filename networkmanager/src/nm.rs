// SPDX-License-Identifier: MPL-2.0

use crate::{
	active_connection::ActiveConnection,
	device::Device,
	interface::{
		NetworkManagerProxy,
		active_connection::ActiveConnectionProxy,
		device::DeviceProxy,
		enums::{NmConnectivityState, NmState},
	},
	settings::{NetworkManagerSettings, connection::Connection},
};
use std::ops::Deref;
use zbus::{Result, zvariant::ObjectPath};

#[derive(Debug)]
pub struct NetworkManager<'a>(NetworkManagerProxy<'a>);

impl<'a> Deref for NetworkManager<'a> {
	type Target = NetworkManagerProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> NetworkManager<'a> {
	pub async fn new(connection: &'a zbus::Connection) -> Result<NetworkManager<'a>> {
		NetworkManagerProxy::new(connection).await.map(Self)
	}

	/// Activate a connection profile for the given device.
	pub async fn activate_connection(
		&self,
		connection: &Connection<'_>,
		device: &Device<'_>,
	) -> Result<ActiveConnection<'a>> {
		let connection = connection.inner().path();
		let device = device.inner().path();
		self.activate_connection_by_paths(connection, device).await
	}

	/// Activate a connection profile for the given device by their object paths.
	pub async fn activate_connection_by_paths(
		&self,
		connection: &ObjectPath<'_>,
		device: &ObjectPath<'_>,
	) -> Result<ActiveConnection<'a>> {
		let specific_object = ObjectPath::from_static_str("/").unwrap();
		let active_connection_path = self
			.0
			.activate_connection(connection, device, &specific_object)
			.await?;
		ActiveConnectionProxy::builder(self.0.inner().connection())
			.path(active_connection_path)?
			.build()
			.await
			.map(ActiveConnection::from)
	}

	pub async fn active_connections(&self) -> Result<Vec<ActiveConnection<'a>>> {
		let active_connections = self.0.active_connections().await?;
		let mut out = Vec::with_capacity(active_connections.len());
		for active_connection in active_connections {
			let active_connection = ActiveConnectionProxy::builder(self.0.inner().connection())
				.path(active_connection)?
				.build()
				.await?;
			out.push(active_connection.into());
		}
		Ok(out)
	}

	pub async fn connectivity(&self) -> Result<NmConnectivityState> {
		self.0.connectivity().await.map(NmConnectivityState::from)
	}

	pub async fn check_connectivity(&self) -> Result<NmConnectivityState> {
		self.0
			.check_connectivity()
			.await
			.map(NmConnectivityState::from)
	}

	pub async fn deactivate_connection(&self, connection: &'a ActiveConnection<'a>) -> Result<()> {
		self.0
			.deactivate_connection(connection.inner().path())
			.await
	}

	pub async fn devices(&self) -> Result<Vec<Device<'a>>> {
		let devices = self.0.get_all_devices().await?;
		let mut out = Vec::with_capacity(devices.len());
		for device in devices {
			let device = DeviceProxy::builder(self.0.inner().connection())
				.path(device)?
				.build()
				.await?;
			out.push(device.into());
		}
		Ok(out)
	}

	pub async fn all_devices(&self) -> Result<Vec<Device<'a>>> {
		let devices = self.0.get_all_devices().await?;
		let mut out = Vec::with_capacity(devices.len());
		for device in devices {
			let device = DeviceProxy::builder(self.0.inner().connection())
				.path(device)?
				.build()
				.await?;
			out.push(device.into());
		}
		Ok(out)
	}

	pub async fn state(&self) -> Result<NmState> {
		self.0.state().await.map(NmState::from)
	}

	pub async fn settings(&'a self) -> Result<NetworkManagerSettings<'a>> {
		NetworkManagerSettings::new(self.0.inner().connection()).await
	}
}
