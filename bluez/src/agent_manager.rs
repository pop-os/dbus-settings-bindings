use std::ops::Deref;

use crate::bindings::bluez::AgentManager1Proxy;

pub struct AgentManager {
	proxy: AgentManager1Proxy<'static>,
}

impl AgentManager {
	pub async fn new(connection: &zbus::Connection) -> Result<Self, zbus::Error> {
		let proxy = AgentManager1Proxy::new(connection).await?;
		Ok(Self { proxy })
	}

	pub async fn register_agent(
		&self,
		agent: &zbus::zvariant::ObjectPath<'_>,
		capability: &str,
	) -> zbus::Result<()> {
		self.proxy.register_agent(agent, capability).await
	}

	pub async fn request_default_agent(
		&self,
		agent: &zbus::zvariant::ObjectPath<'_>,
	) -> zbus::Result<()> {
		self.proxy.request_default_agent(agent).await
	}

	pub async fn unregister_agent(
		&self,
		agent: &zbus::zvariant::ObjectPath<'_>,
	) -> zbus::Result<()> {
		self.proxy.unregister_agent(agent).await
	}
}

impl Deref for AgentManager {
	type Target = AgentManager1Proxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}

impl From<AgentManager1Proxy<'static>> for AgentManager {
	fn from(proxy: AgentManager1Proxy<'static>) -> Self {
		Self { proxy }
	}
}
