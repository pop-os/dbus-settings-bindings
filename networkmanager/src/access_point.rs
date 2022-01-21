// SPDX-License-Identifier: MPL-2.0

use crate::interface::{
	access_point::AccessPointProxy,
	enums::{ApFlags, ApSecurityFlags},
};
use std::ops::Deref;
use zbus::Result;

#[derive(Debug)]
pub struct AccessPoint<'a>(AccessPointProxy<'a>);

impl<'a> AccessPoint<'a> {
	/* TODO: figure out how to convert CLOCK_BOOTTIME to SystemTime, as CLOCK_BOOTTIME's starting point is arbritary and not guaranteed to match up with the UNIX Epoch
	pub async fn last_seen(&self) -> Result<Option<SystemTime>> {
		let last_seen = self.0.last_seen().await?;
		if !last_seen.is_positive() {
			return Ok(None);
		}
	}*/

	pub async fn flags(&self) -> Result<ApFlags> {
		self.0.flags().await.map(ApFlags::from_bits_truncate)
	}

	pub async fn rsn_flags(&self) -> Result<ApSecurityFlags> {
		self.0
			.rsn_flags()
			.await
			.map(ApSecurityFlags::from_bits_truncate)
	}

	pub async fn wpa_flags(&self) -> Result<ApSecurityFlags> {
		self.0
			.wpa_flags()
			.await
			.map(ApSecurityFlags::from_bits_truncate)
	}
}

impl<'a> Deref for AccessPoint<'a> {
	type Target = AccessPointProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<AccessPointProxy<'a>> for AccessPoint<'a> {
	fn from(access_point: AccessPointProxy<'a>) -> Self {
		AccessPoint(access_point)
	}
}
