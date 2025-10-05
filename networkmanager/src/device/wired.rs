// SPDX-License-Identifier: MPL-2.0

use super::Device;
use crate::interface::device::{DeviceProxy, wired::WiredDeviceProxy};
use std::ops::Deref;
use zbus::Result;

#[derive(Debug)]
pub struct WiredDevice<'a>(WiredDeviceProxy<'a>);

impl<'a> WiredDevice<'a> {
	pub async fn upcast(&'a self) -> Result<Device<'a>> {
		DeviceProxy::builder(self.0.inner().connection())
			.path(self.0.inner().path())?
			.build()
			.await
			.map(Device::from)
	}
}

impl<'a> Deref for WiredDevice<'a> {
	type Target = WiredDeviceProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<WiredDeviceProxy<'a>> for WiredDevice<'a> {
	fn from(device: WiredDeviceProxy<'a>) -> Self {
		WiredDevice(device)
	}
}
