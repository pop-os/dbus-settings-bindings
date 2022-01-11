// SPDX-License-Identifier: MPL-2.0

pub mod bluetooth;
pub mod wired;
pub mod wireless;

use crate::{
	config::{ip4::Ipv4Config, ip6::Ipv6Config},
	interface::{
		config::{ip4::Ipv4ConfigProxy, ip6::Ipv6ConfigProxy},
		device::DeviceProxy,
	},
};
use std::{net::Ipv4Addr, ops::Deref};
use zbus::Result;

pub struct Device<'a>(DeviceProxy<'a>);

impl<'a> Device<'a> {
	pub async fn ip4_address(&self) -> Result<Ipv4Addr> {
		self.0.ip4_address().await.map(Ipv4Addr::from)
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
