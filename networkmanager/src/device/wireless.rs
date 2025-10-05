// SPDX-License-Identifier: MPL-2.0

use super::Device;
use crate::{
	access_point::AccessPoint,
	interface::{
		access_point::AccessPointProxy,
		device::{DeviceProxy, wireless::WirelessDeviceProxy},
		enums::{WifiCapabilities, WifiMode},
	},
};
use std::ops::Deref;
use zbus::Result;

#[derive(Debug)]
pub struct WirelessDevice<'a>(WirelessDeviceProxy<'a>);

impl<'a> WirelessDevice<'a> {
	pub async fn get_access_points(&self) -> Result<Vec<AccessPoint<'a>>> {
		let access_points = self.0.get_access_points().await?;
		let mut out = Vec::with_capacity(access_points.len());
		for access_point in access_points {
			let access_point = AccessPointProxy::builder(self.0.inner().connection())
				.path(access_point)?
				.build()
				.await?;
			out.push(access_point.into());
		}
		Ok(out)
	}

	pub async fn get_all_access_points(&self) -> Result<Vec<AccessPoint<'a>>> {
		let access_points = self.0.get_all_access_points().await?;
		let mut out = Vec::with_capacity(access_points.len());
		for access_point in access_points {
			let access_point = AccessPointProxy::builder(self.0.inner().connection())
				.path(access_point)?
				.build()
				.await?;
			out.push(access_point.into());
		}
		Ok(out)
	}

	pub async fn access_points(&self) -> Result<Vec<AccessPoint<'a>>> {
		let access_points = self.0.access_points().await?;
		let mut out = Vec::with_capacity(access_points.len());
		for access_point in access_points {
			let access_point = AccessPointProxy::builder(self.0.inner().connection())
				.path(access_point)?
				.build()
				.await?;
			out.push(access_point.into());
		}
		Ok(out)
	}

	pub async fn active_access_point(&self) -> Result<AccessPoint<'a>> {
		AccessPointProxy::builder(self.0.inner().connection())
			.path(self.0.active_access_point().await?)?
			.build()
			.await
			.map(AccessPoint::from)
	}

	pub async fn upcast(&'a self) -> Result<Device<'a>> {
		DeviceProxy::builder(self.0.inner().connection())
			.path(self.0.inner().path())?
			.build()
			.await
			.map(Device::from)
	}

	pub async fn mode(&self) -> Result<WifiMode> {
		self.0.mode().await.map(WifiMode::from)
	}

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
