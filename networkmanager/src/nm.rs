// SPDX-License-Identifier: MPL-2.0

use crate::{
	active_connection::ActiveConnection,
	device::Device,
	interface::{
		active_connection::ActiveConnectionProxy,
		device::DeviceProxy,
		enums::{ConnectivityState, State},
		NetworkManagerProxy,
	},
	settings::{connection::Connection, NetworkManagerSettings},
};
use zbus::{zvariant::ObjectPath, Result};

#[derive(Debug)]
pub struct NetworkManager<'a>(NetworkManagerProxy<'a>);

impl<'a> NetworkManager<'a> {
	pub async fn new(connection: &'a zbus::Connection) -> Result<NetworkManager<'a>> {
		NetworkManagerProxy::new(connection).await.map(Self)
	}

	pub async fn activate_connection(
		&self,
		connection: &'a Connection<'a>,
		device: &'a Device<'a>,
	) -> Result<ActiveConnection<'a>> {
		let connection = connection.path();
		let device = device.path();
		let specific_object = ObjectPath::from_static_str("/").unwrap();
		let active_connection_path = self
			.0
			.activate_connection(connection, device, &specific_object)
			.await?;
		ActiveConnectionProxy::builder(self.0.connection())
			.path(active_connection_path)?
			.build()
			.await
			.map(ActiveConnection::from)
	}

	pub async fn connectivity(&self) -> Result<ConnectivityState> {
		self.0.connectivity().await.map(ConnectivityState::from)
	}

	pub async fn check_connectivity(&self) -> Result<ConnectivityState> {
		self.0
			.check_connectivity()
			.await
			.map(ConnectivityState::from)
	}

	pub async fn deactivate_connection(&self, connection: &'a ActiveConnection<'a>) -> Result<()> {
		self.0.deactivate_connection(connection.path()).await
	}

	pub async fn devices(&self) -> Result<Vec<Device<'a>>> {
		let devices = self.0.get_all_devices().await?;
		let mut out = Vec::with_capacity(devices.len());
		for device in devices {
			let device = DeviceProxy::builder(self.0.connection())
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
			let device = DeviceProxy::builder(self.0.connection())
				.path(device)?
				.build()
				.await?;
			out.push(device.into());
		}
		Ok(out)
	}

	pub async fn state(&self) -> Result<State> {
		self.0.state().await.map(State::from)
	}

	pub async fn settings(&'a self) -> Result<NetworkManagerSettings<'a>> {
		NetworkManagerSettings::new(self.0.connection()).await
	}
}
