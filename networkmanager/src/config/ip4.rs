// SPDX-License-Identifier: MPL-2.0
use crate::interface::config::ip4::Ipv4ConfigProxy;
use std::{net::Ipv4Addr, ops::Deref};
use zbus::Result;

#[derive(Debug)]
pub struct Ipv4Config<'a>(Ipv4ConfigProxy<'a>);

impl<'a> Ipv4Config<'a> {
	pub async fn addresses(&self) -> Result<Vec<Vec<Ipv4Addr>>> {
		let addresses = self.0.addresses().await?;
		Ok(addresses
			.into_iter()
			.map(|addresses| addresses.into_iter().map(Ipv4Addr::from).collect())
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
