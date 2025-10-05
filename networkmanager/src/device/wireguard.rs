// SPDX-License-Identifier: MPL-2.0

use super::Device;
use crate::interface::device::{DeviceProxy, wireguard::WireGuardDeviceProxy};
use std::ops::Deref;
use zbus::Result;

#[derive(Debug)]
pub struct WireGuardDevice<'a>(WireGuardDeviceProxy<'a>);

impl<'a> WireGuardDevice<'a> {
	pub async fn upcast(&'a self) -> Result<Device<'a>> {
		DeviceProxy::builder(self.0.inner().connection())
			.path(self.0.inner().path())?
			.build()
			.await
			.map(Device::from)
	}
}

impl<'a> Deref for WireGuardDevice<'a> {
	type Target = WireGuardDeviceProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<WireGuardDeviceProxy<'a>> for WireGuardDevice<'a> {
	fn from(device: WireGuardDeviceProxy<'a>) -> Self {
		WireGuardDevice(device)
	}
}
