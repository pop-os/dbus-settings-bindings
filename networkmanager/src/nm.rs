// SPDX-License-Identifier: MPL-2.0

use crate::{
	device::Device,
	interface::{device::DeviceProxy, NetworkManagerProxy},
};
use zbus::{Connection, Result};

pub struct NetworkManager<'a>(NetworkManagerProxy<'a>);

impl<'a> NetworkManager<'a> {
	pub async fn new(connection: &'a Connection) -> Result<NetworkManager<'a>> {
		NetworkManagerProxy::new(connection).await.map(Self)
	}

	pub async fn devices(&self) -> Result<Vec<Device<'a>>> {
		let devices = self.0.get_all_devices().await?;
		let mut out = Vec::with_capacity(devices.len());
		for device in devices {
			let device = DeviceProxy::builder(self.0.connection())
				.path(device)?
				.build()
				.await?;
			out.push(device.into());
		}
		Ok(out)
	}

	pub async fn all_devices(&self) -> Result<Vec<Device<'a>>> {
		let devices = self.0.get_all_devices().await?;
		let mut out = Vec::with_capacity(devices.len());
		for device in devices {
			let device = DeviceProxy::builder(self.0.connection())
				.path(device)?
				.build()
				.await?;
			out.push(device.into());
		}
		Ok(out)
	}
}
