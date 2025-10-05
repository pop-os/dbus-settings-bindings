// SPDX-License-Identifier: MPL-2.0

pub mod bluetooth;
pub mod tun;
pub mod wired;
pub mod wireguard;
pub mod wireless;

use crate::{
	active_connection::ActiveConnection,
	config::{ip4::Ipv4Config, ip6::Ipv6Config},
	interface::{
		active_connection::ActiveConnectionProxy,
		config::{ip4::Ipv4ConfigProxy, ip6::Ipv6ConfigProxy},
		device::{
			DeviceProxy, bluetooth::BluetoothDeviceProxy, tun::TunDeviceProxy,
			wired::WiredDeviceProxy, wireguard::WireGuardDeviceProxy,
			wireless::WirelessDeviceProxy,
		},
		enums::{DeviceCapabilities, DeviceState, DeviceType},
		settings::connection::ConnectionSettingsProxy,
	},
	settings::connection::Connection,
};
use std::{net::Ipv4Addr, ops::Deref};
use zbus::Result;

#[derive(Debug)]
pub struct Device<'a>(DeviceProxy<'a>);

impl<'a> Device<'a> {
	pub async fn active_connection(&self) -> Result<ActiveConnection<'a>> {
		let active_connection = self.0.active_connection().await?;
		Ok(ActiveConnectionProxy::builder(self.0.inner().connection())
			.path(active_connection)?
			.build()
			.await?
			.into())
	}

	pub async fn available_connections(&self) -> Result<Vec<Connection<'a>>> {
		let available_connections = self.0.available_connections().await?;
		let mut out = Vec::with_capacity(available_connections.len());
		for connection in available_connections {
			let connection = ConnectionSettingsProxy::builder(self.0.inner().connection())
				.path(connection)?
				.build()
				.await?;
			out.push(connection.into());
		}
		Ok(out)
	}

	pub async fn capabilities(&self) -> Result<DeviceCapabilities> {
		self.0
			.capabilities()
			.await
			.map(DeviceCapabilities::from_bits_truncate)
	}

	pub async fn device_type(&self) -> Result<DeviceType> {
		self.0.device_type().await.map(DeviceType::from)
	}

	pub async fn downcast_to_device(&'a self) -> Result<Option<SpecificDevice<'a>>> {
		match self.device_type().await? {
			DeviceType::Bluetooth => Ok(Some(SpecificDevice::Bluetooth(
				BluetoothDeviceProxy::builder(self.0.inner().connection())
					.path(self.0.inner().path())?
					.build()
					.await?
					.into(),
			))),
			DeviceType::Ethernet => Ok(Some(SpecificDevice::Wired(
				WiredDeviceProxy::builder(self.0.inner().connection())
					.path(self.0.inner().path())?
					.build()
					.await?
					.into(),
			))),
			DeviceType::Wifi => Ok(Some(SpecificDevice::Wireless(
				WirelessDeviceProxy::builder(self.0.inner().connection())
					.path(self.0.inner().path())?
					.build()
					.await?
					.into(),
			))),
			DeviceType::TunTap => Ok(Some(SpecificDevice::TunTap(
				TunDeviceProxy::builder(self.0.inner().connection())
					.path(self.0.inner().path())?
					.build()
					.await?
					.into(),
			))),
			DeviceType::WireGuard => Ok(Some(SpecificDevice::WireGuard(
				WireGuardDeviceProxy::builder(self.0.inner().connection())
					.path(self.0.inner().path())?
					.build()
					.await?
					.into(),
			))),
			_ => Ok(None),
		}
	}

	pub async fn ip4_address(&self) -> Result<Ipv4Addr> {
		self.0.ip4_address().await.map(Ipv4Addr::from)
	}

	pub async fn ip4_config(&self) -> Result<Ipv4Config<'a>> {
		let config = Ipv4ConfigProxy::builder(self.0.inner().connection())
			.path(self.0.ip4_config().await?)?
			.build()
			.await?;
		Ok(Ipv4Config::from(config))
	}

	pub async fn ip6_config(&self) -> Result<Ipv6Config<'a>> {
		let config = Ipv6ConfigProxy::builder(self.0.inner().connection())
			.path(self.0.ip6_config().await?)?
			.build()
			.await?;
		Ok(Ipv6Config::from(config))
	}

	pub async fn state(&self) -> Result<DeviceState> {
		self.0.state().await.map(DeviceState::from)
	}
}

impl<'a> Deref for Device<'a> {
	type Target = DeviceProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<DeviceProxy<'a>> for Device<'a> {
	fn from(device: DeviceProxy<'a>) -> Self {
		Device(device)
	}
}

pub enum SpecificDevice<'a> {
	Bluetooth(bluetooth::BluetoothDevice<'a>),
	Wired(wired::WiredDevice<'a>),
	Wireless(wireless::WirelessDevice<'a>),
	TunTap(tun::TunDevice<'a>),
	WireGuard(wireguard::WireGuardDevice<'a>),
}

impl<'a> SpecificDevice<'a> {
	pub fn into_bluetooth(self) -> Option<bluetooth::BluetoothDevice<'a>> {
		match self {
			SpecificDevice::Bluetooth(device) => Some(device),
			_ => None,
		}
	}

	pub fn into_wired(self) -> Option<wired::WiredDevice<'a>> {
		match self {
			SpecificDevice::Wired(device) => Some(device),
			_ => None,
		}
	}

	pub fn into_wireless(self) -> Option<wireless::WirelessDevice<'a>> {
		match self {
			SpecificDevice::Wireless(device) => Some(device),
			_ => None,
		}
	}

	pub fn into_tun(self) -> Option<tun::TunDevice<'a>> {
		match self {
			SpecificDevice::TunTap(device) => Some(device),
			_ => None,
		}
	}

	pub fn into_wireguard(self) -> Option<wireguard::WireGuardDevice<'a>> {
		match self {
			SpecificDevice::WireGuard(device) => Some(device),
			_ => None,
		}
	}
}
