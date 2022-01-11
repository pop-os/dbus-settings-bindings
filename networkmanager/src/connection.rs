// SPDX-License-Identifier: MPL-2.0

use crate::{
	config::{ip4::Ipv4Config, ip6::Ipv6Config},
	device::Device,
	interface::{
		config::{ip4::Ipv4ConfigProxy, ip6::Ipv6ConfigProxy},
		connection::ActiveConnectionProxy,
		device::DeviceProxy,
		enums::{ActivationStateFlags, State},
	},
};
use std::ops::Deref;
use zbus::Result;

pub struct Connection<'a>(ActiveConnectionProxy<'a>);

impl<'a> Connection<'a> {
	pub async fn devices(&self) -> Result<Vec<Device<'a>>> {
		let devices = self.0.devices().await?;
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

	pub async fn ip4_config(&self) -> Result<Ipv4Config<'a>> {
		let config = Ipv4ConfigProxy::builder(self.0.connection())
			.path(self.0.ip4_config().await?)?
			.build()
			.await?;
		Ok(Ipv4Config::from(config))
	}

	pub async fn ip6_config(&self) -> Result<Ipv6Config<'a>> {
		let config = Ipv6ConfigProxy::builder(self.0.connection())
			.path(self.0.ip6_config().await?)?
			.build()
			.await?;
		Ok(Ipv6Config::from(config))
	}

	pub async fn state(&self) -> Result<State> {
		self.0.state().await.map(State::from)
	}

	pub async fn state_flags(&self) -> Result<ActivationStateFlags> {
		self.0
			.state_flags()
			.await
			.map(ActivationStateFlags::from_bits_truncate)
	}
}

impl<'a> Deref for Connection<'a> {
	type Target = ActiveConnectionProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<ActiveConnectionProxy<'a>> for Connection<'a> {
	fn from(connection: ActiveConnectionProxy<'a>) -> Self {
		Connection(connection)
	}
}
