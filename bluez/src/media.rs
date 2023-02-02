use std::{collections::HashMap, ops::Deref};

use crate::bindings::adapter::Media1Proxy;

pub struct Media {
	proxy: Media1Proxy<'static>,
}

impl Media {
	pub async fn new(connection: &zbus::Connection) -> Result<Self, zbus::Error> {
		let proxy = Media1Proxy::new(connection).await?;
		Ok(Self { proxy })
	}

	pub async fn register_endpoint(
		&self,
		endpoint: &zbus::zvariant::ObjectPath<'_>,
		capabilities: HashMap<&str, zbus::zvariant::Value<'_>>,
	) -> zbus::Result<()> {
		self.proxy.register_endpoint(endpoint, capabilities).await
	}

	pub async fn unregister_endpoint(
		&self,
		endpoint: &zbus::zvariant::ObjectPath<'_>,
	) -> zbus::Result<()> {
		self.proxy.unregister_endpoint(endpoint).await
	}

	pub async fn register_player(
		&self,
		player: &zbus::zvariant::ObjectPath<'_>,
		capabilities: HashMap<&str, zbus::zvariant::Value<'_>>,
	) -> zbus::Result<()> {
		self.proxy.register_player(player, capabilities).await
	}

	pub async fn unregister_player(
		&self,
		player: &zbus::zvariant::ObjectPath<'_>,
	) -> zbus::Result<()> {
		self.proxy.unregister_player(player).await
	}

	pub async fn register_application(
		&self,
		application: &zbus::zvariant::ObjectPath<'_>,
		capabilities: HashMap<&str, zbus::zvariant::Value<'_>>,
	) -> zbus::Result<()> {
		self.proxy
			.register_application(application, capabilities)
			.await
	}

	pub async fn unregister_application(
		&self,
		application: &zbus::zvariant::ObjectPath<'_>,
	) -> zbus::Result<()> {
		self.proxy.unregister_application(application).await
	}
}

impl Deref for Media {
	type Target = Media1Proxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}

impl From<Media1Proxy<'static>> for Media {
	fn from(proxy: Media1Proxy<'static>) -> Self {
		Self { proxy }
	}
}
