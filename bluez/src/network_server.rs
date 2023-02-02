use std::ops::Deref;

use crate::bindings::adapter::NetworkServer1Proxy;

pub struct NetworkServer {
	proxy: NetworkServer1Proxy<'static>,
}

impl NetworkServer {
	pub async fn new(connection: &zbus::Connection) -> Result<Self, zbus::Error> {
		let proxy = NetworkServer1Proxy::new(connection).await?;
		Ok(Self { proxy })
	}

	pub async fn register_agent(&self, uuid: &str, capability: &str) -> zbus::Result<()> {
		self.proxy.register(uuid, capability).await
	}

	pub async fn unregister_agent(&self, uuid: &str) -> zbus::Result<()> {
		self.proxy.unregister(uuid).await
	}
}

impl Deref for NetworkServer {
	type Target = NetworkServer1Proxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}

impl From<NetworkServer1Proxy<'static>> for NetworkServer {
	fn from(proxy: NetworkServer1Proxy<'static>) -> Self {
		Self { proxy }
	}
}
