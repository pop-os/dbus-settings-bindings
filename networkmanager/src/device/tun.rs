// SPDX-License-Identifier: MPL-2.0

use super::Device;
use crate::interface::device::{DeviceProxy, tun::TunDeviceProxy};
use std::ops::Deref;
use zbus::Result;

#[derive(Debug)]
pub struct TunDevice<'a>(TunDeviceProxy<'a>);

impl<'a> TunDevice<'a> {
	pub async fn upcast(&'a self) -> Result<Device<'a>> {
		DeviceProxy::builder(self.0.inner().connection())
			.path(self.0.inner().path())?
			.build()
			.await
			.map(Device::from)
	}

	pub async fn owner(&self) -> Result<Option<u16>> {
		let owner = self.0.owner().await?;
		match owner {
			-1 => Ok(None),
			_ => Ok(u16::try_from(owner).ok()),
		}
	}
}

impl<'a> Deref for TunDevice<'a> {
	type Target = TunDeviceProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<TunDeviceProxy<'a>> for TunDevice<'a> {
	fn from(device: TunDeviceProxy<'a>) -> Self {
		TunDevice(device)
	}
}
