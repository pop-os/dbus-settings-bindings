// SPDX-License-Identifier: MPL-2.0

use crate::interface::device::bluetooth::BluetoothDeviceProxy;
use std::ops::Deref;

pub struct BluetoothDevice<'a>(BluetoothDeviceProxy<'a>);

impl<'a> Deref for BluetoothDevice<'a> {
	type Target = BluetoothDeviceProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
