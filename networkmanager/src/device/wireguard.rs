// SPDX-License-Identifier: MPL-2.0

use super::Device;
use crate::interface::device::{wireguard::WireGuardDeviceProxy, DeviceProxy};
use std::ops::Deref;
use zbus::Result;

#[derive(Debug)]
pub struct WireGuardDevice<'a>(WireGuardDeviceProxy<'a>);

impl<'a> WireGuardDevice<'a> {
	pub async fn upcast(&'a self) -> Result<Device<'a>> {
		DeviceProxy::builder(self.0.connection())
			.path(self.0.path())?
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
