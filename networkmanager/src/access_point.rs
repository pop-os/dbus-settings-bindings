// SPDX-License-Identifier: MPL-2.0

use crate::{
	interface::{
		access_point::AccessPointProxy,
		enums::{ApFlags, ApSecurityFlags},
	},
	util::clock_boottime_to_time,
};
use std::ops::Deref;
use time::OffsetDateTime;
use zbus::Result;

#[derive(Debug)]
pub struct AccessPoint<'a>(AccessPointProxy<'a>);

impl<'a> AccessPoint<'a> {
	pub async fn last_seen(&self) -> Result<Option<OffsetDateTime>> {
		Ok(clock_boottime_to_time(self.0.last_seen().await?))
	}

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
