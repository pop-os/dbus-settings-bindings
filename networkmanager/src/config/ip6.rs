// SPDX-License-Identifier: MPL-2.0
use crate::interface::config::ip6::Ipv6ConfigProxy;
use std::{net::Ipv6Addr, ops::Deref, str::FromStr};
use zbus::Result;

#[derive(Debug)]
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

	pub async fn address_data(&self) -> Result<Vec<AddressData>> {
		Ok(self
			.0
			.address_data()
			.await?
			.into_iter()
			.filter_map(|mut map| {
				let address = {
					let address_str = map.remove("address")?;
					let address_str = address_str.downcast_ref().ok()?;
					Ipv6Addr::from_str(address_str).ok()?
				};
				let prefix = u32::try_from(map.remove("prefix")?).ok()?;
				Some(AddressData { address, prefix })
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AddressData {
	pub address: Ipv6Addr,
	pub prefix: u32,
}
