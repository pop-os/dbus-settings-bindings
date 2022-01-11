// SPDX-License-Identifier: MPL-2.0

use crate::interface::device::wireless::WirelessDeviceProxy;
use std::ops::Deref;

pub struct WirelessDevice<'a>(WirelessDeviceProxy<'a>);

impl<'a> Deref for WirelessDevice<'a> {
	type Target = WirelessDeviceProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
