use zvariant::ObjectPath;

use crate::bindings::adapter::Adapter1Proxy;

pub struct Adapter {
	proxy: Adapter1Proxy<'static>,
}

impl Adapter {
	pub async fn default(connection: &zbus::Connection) -> Result<Self, zbus::Error> {
		let proxy = Adapter1Proxy::new(connection).await?;
		Ok(Self { proxy })
	}

	pub async fn list_all_adapters(
		connection: &zbus::Connection,
	) -> Result<Vec<Self>, zbus::Error> {
		let objects: Vec<_> = connection
			.call_method(
				Some("org.bluez"),
				"/",
				Some("org.freedesktop.DBus.ObjectManager"),
				"GetManagedObjects",
				&(),
			)
			.await?
			.body::<std::collections::HashMap<
				ObjectPath<'_>,
				std::collections::HashMap<
					String,
					std::collections::HashMap<String, zvariant::Value<'_>>,
				>,
			>>()?
			.into_iter()
			.filter_map(|o| {
				if o.1.contains_key("org.bluez.Adapter1") {
					Some(o.0.to_owned())
				} else {
					None
				}
			})
			.collect();
		let mut adapters = Vec::with_capacity(objects.len());
		for path in objects {
			adapters.push(Adapter::new(connection, path.to_owned()).await?);
		}
		Ok(adapters)
	}

	pub async fn auto(connection: &zbus::Connection) -> Result<Self, zbus::Error> {
		if let Some(adapter_path) = connection
			.call_method(
				Some("org.bluez"),
				"/",
				Some("org.freedesktop.DBus.ObjectManager"),
				"GetManagedObjects",
				&(),
			)
			.await?
			.body::<std::collections::HashMap<
				ObjectPath<'_>,
				std::collections::HashMap<
					String,
					std::collections::HashMap<String, zvariant::Value<'_>>,
				>,
			>>()?
			.into_iter()
			.find_map(|(path, interfaces)| {
				if interfaces.contains_key("org.bluez.Adapter1") {
					Some(path.to_owned())
				} else {
					None
				}
			}) {
			Self::new(connection, adapter_path).await
		} else {
			Self::default(connection).await
		}
	}

	pub async fn new<P: Into<ObjectPath<'static>>>(
		connection: &zbus::Connection,
		path: P,
	) -> Result<Self, zbus::Error> {
		let proxy = Adapter1Proxy::builder(connection)
			.interface("org.bluez.Adapter1")?
			.path(path)?
			.build()
			.await?;
		Ok(Self { proxy })
	}

	pub async fn start_discovery(&self) -> zbus::Result<()> {
		self.proxy.start_discovery().await
	}

	pub async fn stop_discovery(&self) -> zbus::Result<()> {
		self.proxy.stop_discovery().await
	}

	pub async fn set_discoverable(&self, discoverable: bool) -> zbus::Result<()> {
		self.proxy.set_discoverable(discoverable).await
	}

	pub async fn set_pairable(&self, pairable: bool) -> zbus::Result<()> {
		self.proxy.set_pairable(pairable).await
	}

	pub async fn set_pairable_timeout(&self, timeout: u32) -> zbus::Result<()> {
		self.proxy.set_pairable_timeout(timeout).await
	}

	pub async fn set_alias(&self, alias: &str) -> zbus::Result<()> {
		self.proxy.set_alias(alias).await
	}

	pub async fn set_powered(&self, powered: bool) -> zbus::Result<()> {
		self.proxy.set_powered(powered).await
	}

	pub async fn set_discoverable_timeout(&self, timeout: u32) -> zbus::Result<()> {
		self.proxy.set_discoverable_timeout(timeout).await
	}

	pub async fn modalias(&self) -> zbus::Result<String> {
		self.proxy.modalias().await
	}

	pub async fn name(&self) -> zbus::Result<String> {
		self.proxy.name().await
	}

	pub async fn alias(&self) -> zbus::Result<String> {
		self.proxy.alias().await
	}

	pub async fn class(&self) -> zbus::Result<u32> {
		self.proxy.class().await
	}

	pub async fn powered(&self) -> zbus::Result<bool> {
		self.proxy.powered().await
	}

	pub async fn discoverable(&self) -> zbus::Result<bool> {
		self.proxy.discoverable().await
	}

	pub async fn pairable(&self) -> zbus::Result<bool> {
		self.proxy.pairable().await
	}

	pub async fn discoverable_timeout(&self) -> zbus::Result<u32> {
		self.proxy.discoverable_timeout().await
	}

	pub async fn roles(&self) -> zbus::Result<Vec<String>> {
		self.proxy.roles().await
	}

	pub async fn uuids(&self) -> zbus::Result<Vec<String>> {
		self.proxy.uuids().await
	}
}

impl From<Adapter1Proxy<'static>> for Adapter {
	fn from(proxy: Adapter1Proxy<'static>) -> Self {
		Self { proxy }
	}
}

impl std::ops::Deref for Adapter {
	type Target = Adapter1Proxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}
