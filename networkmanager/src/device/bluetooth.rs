// SPDX-License-Identifier: MPL-2.0

use super::Device;
use crate::interface::device::{bluetooth::BluetoothDeviceProxy, DeviceProxy};
use std::ops::Deref;
use zbus::Result;

pub struct BluetoothDevice<'a>(BluetoothDeviceProxy<'a>);

impl<'a> BluetoothDevice<'a> {
	pub async fn upcast(&'a self) -> Result<Device<'a>> {
		DeviceProxy::builder(self.0.connection())
			.path(self.0.path())?
			.build()
			.await
			.map(Device::from)
	}
}

impl<'a> Deref for BluetoothDevice<'a> {
	type Target = BluetoothDeviceProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<BluetoothDeviceProxy<'a>> for BluetoothDevice<'a> {
	fn from(device: BluetoothDeviceProxy<'a>) -> Self {
		BluetoothDevice(device)
	}
}
