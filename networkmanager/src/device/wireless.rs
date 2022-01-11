// SPDX-License-Identifier: MPL-2.0

use crate::interface::{device::wireless::WirelessDeviceProxy, enums::WifiCapabilities};
use std::ops::Deref;
use zbus::Result;

pub struct WirelessDevice<'a>(WirelessDeviceProxy<'a>);

impl<'a> WirelessDevice<'a> {
	pub async fn wireless_capabilities(&self) -> Result<WifiCapabilities> {
		self.0
			.wireless_capabilities()
			.await
			.map(WifiCapabilities::from_bits_truncate)
	}
}

impl<'a> Deref for WirelessDevice<'a> {
	type Target = WirelessDeviceProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<WirelessDeviceProxy<'a>> for WirelessDevice<'a> {
	fn from(device: WirelessDeviceProxy<'a>) -> Self {
		WirelessDevice(device)
	}
}
