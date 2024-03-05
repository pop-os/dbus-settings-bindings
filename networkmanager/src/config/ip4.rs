// SPDX-License-Identifier: MPL-2.0
use crate::interface::config::ip4::Ipv4ConfigProxy;
use std::{net::Ipv4Addr, ops::Deref, str::FromStr};
use zbus::Result;

#[derive(Debug)]
pub struct Ipv4Config<'a>(Ipv4ConfigProxy<'a>);

impl<'a> Ipv4Config<'a> {
	pub async fn addresses(&self) -> Result<Vec<Vec<Ipv4Addr>>> {
		let addresses = self.0.addresses().await?;
		Ok(addresses
			.into_iter()
			.map(|addresses| {
				addresses
					.into_iter()
					.map(|addr| addr.swap_bytes())
					.map(Ipv4Addr::from)
					.collect()
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
					Ipv4Addr::from_str(address_str).ok()?
				};
				let prefix = u32::try_from(map.remove("prefix")?).ok()?;
				Some(AddressData { address, prefix })
			})
			.collect())
	}
}

impl<'a> Deref for Ipv4Config<'a> {
	type Target = Ipv4ConfigProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<Ipv4ConfigProxy<'a>> for Ipv4Config<'a> {
	fn from(config: Ipv4ConfigProxy<'a>) -> Self {
		Ipv4Config(config)
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AddressData {
	pub address: Ipv4Addr,
	pub prefix: u32,
}
