// SPDX-License-Identifier: MPL-2.0

use crate::{
	config::{ip4::Ipv4Config, ip6::Ipv6Config},
	device::Device,
	interface::{
		active_connection::ActiveConnectionProxy,
		config::{ip4::Ipv4ConfigProxy, ip6::Ipv6ConfigProxy},
		device::DeviceProxy,
		enums::{ActivationStateFlags, ActiveConnectionState},
	},
};
use std::ops::Deref;
use zbus::Result;

#[derive(Debug)]
pub struct ActiveConnection<'a>(ActiveConnectionProxy<'a>);

impl<'a> ActiveConnection<'a> {
	pub async fn devices(&self) -> Result<Vec<Device<'a>>> {
		let devices = self.0.devices().await?;
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

	pub async fn state(&self) -> Result<ActiveConnectionState> {
		self.0.state().await.map(ActiveConnectionState::from)
	}

	pub async fn state_flags(&self) -> Result<ActivationStateFlags> {
		self.0
			.state_flags()
			.await
			.map(ActivationStateFlags::from_bits_truncate)
	}
}

impl<'a> Deref for ActiveConnection<'a> {
	type Target = ActiveConnectionProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<ActiveConnectionProxy<'a>> for ActiveConnection<'a> {
	fn from(connection: ActiveConnectionProxy<'a>) -> Self {
		ActiveConnection(connection)
	}
}
