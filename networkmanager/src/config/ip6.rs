// SPDX-License-Identifier: MPL-2.0
use crate::interface::config::ip6::Ipv6ConfigProxy;
use std::{net::Ipv6Addr, ops::Deref};
use zbus::Result;

pub struct Ipv6Config<'a>(Ipv6ConfigProxy<'a>);

impl<'a> Ipv6Config<'a> {
	pub async fn addresses(&self) -> Result<Vec<Ipv6Addr>> {
		let addresses = self.0.addresses().await?;
		Ok(addresses
			.into_iter()
			.map(|(address, _, _)| {
				let address_bytes: [u8; 16] = address
					.try_into()
					.expect("NetworkManager gave invalid IPv6 addresss");
				Ipv6Addr::from(address_bytes)
			})
			.collect())
	}
}

impl<'a> Deref for Ipv6Config<'a> {
	type Target = Ipv6ConfigProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<Ipv6ConfigProxy<'a>> for Ipv6Config<'a> {
	fn from(config: Ipv6ConfigProxy<'a>) -> Self {
		Ipv6Config(config)
	}
}
