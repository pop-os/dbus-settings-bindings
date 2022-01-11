// SPDX-License-Identifier: MPL-2.0

pub mod bluetooth;
pub mod wired;
pub mod wireless;

use crate::interface::device::DeviceProxy;
use std::ops::Deref;

pub struct Device<'a>(DeviceProxy<'a>);

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
