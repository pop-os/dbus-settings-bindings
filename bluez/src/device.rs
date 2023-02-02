use std::{collections::HashMap, ops::Deref};

use zvariant::ObjectPath;

use crate::bindings::device::Device1Proxy;

pub struct Device {
	proxy: Device1Proxy<'static>,
}

impl Device {
	pub async fn list_all_devices(connection: &zbus::Connection) -> zbus::Result<Vec<Device>> {
		let device_paths: Vec<_> = connection
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
			.into_iter()
			.filter_map(|(path, interfaces)| {
				if interfaces.contains_key("org.bluez.Device1") {
					Some(path.to_owned())
				} else {
					None
				}
			})
			.collect();

		let mut devices = Vec::with_capacity(device_paths.len());

		for p in device_paths {
			devices.push(Self::new(connection, p).await?);
		}

		Ok(devices)
	}

	pub async fn new<P: Into<ObjectPath<'static>>>(
		connection: &zbus::Connection,
		path: P,
	) -> Result<Self, zbus::Error> {
		Ok(Self {
			proxy: Device1Proxy::builder(connection)
				.interface("org.bluez.Device1")?
				.path(path)?
				.build()
				.await?,
		})
	}

	pub async fn connect(&self) -> zbus::Result<()> {
		self.proxy.connect().await
	}

	pub async fn disconnect(&self) -> zbus::Result<()> {
		self.proxy.disconnect().await
	}

	pub async fn pair(&self) -> zbus::Result<()> {
		self.proxy.pair().await
	}

	pub async fn cancel_pairing(&self) -> zbus::Result<()> {
		self.proxy.cancel_pairing().await
	}

	pub async fn connect_profile(&self, uuid: &str) -> zbus::Result<()> {
		self.proxy.connect_profile(uuid).await
	}

	pub async fn disconnect_profile(&self, uuid: &str) -> zbus::Result<()> {
		self.proxy.disconnect_profile(uuid).await
	}

	pub async fn uuids(&self) -> zbus::Result<Vec<String>> {
		self.proxy.uuids().await
	}

	pub async fn name(&self) -> zbus::Result<String> {
		self.proxy.name().await
	}

	pub async fn alias(&self) -> zbus::Result<String> {
		self.proxy.alias().await
	}

	pub async fn set_alias(&self, alias: &str) -> zbus::Result<()> {
		self.proxy.set_alias(alias).await
	}

	pub async fn address(&self) -> zbus::Result<String> {
		self.proxy.address().await
	}

	pub async fn address_type(&self) -> zbus::Result<String> {
		self.proxy.address_type().await
	}

	pub async fn class(&self) -> zbus::Result<u32> {
		self.proxy.class().await
	}

	pub async fn appearance(&self) -> zbus::Result<u16> {
		self.proxy.appearance().await
	}

	pub async fn icon(&self) -> zbus::Result<String> {
		self.proxy.icon().await
	}

	pub async fn paired(&self) -> zbus::Result<bool> {
		self.proxy.paired().await
	}

	pub async fn trusted(&self) -> zbus::Result<bool> {
		self.proxy.trusted().await
	}

	pub async fn set_trusted(&self, trusted: bool) -> zbus::Result<()> {
		self.proxy.set_trusted(trusted).await
	}

	pub async fn blocked(&self) -> zbus::Result<bool> {
		self.proxy.blocked().await
	}

	pub async fn set_blocked(&self, blocked: bool) -> zbus::Result<()> {
		self.proxy.set_blocked(blocked).await
	}

	pub async fn legacy_pairing(&self) -> zbus::Result<bool> {
		self.proxy.legacy_pairing().await
	}

	pub async fn rssi(&self) -> zbus::Result<i16> {
		self.proxy.rssi().await
	}

	pub async fn tx_power(&self) -> zbus::Result<i16> {
		self.proxy.tx_power().await
	}

	pub async fn modalias(&self) -> zbus::Result<String> {
		self.proxy.modalias().await
	}

	pub async fn wake_allowed(&self) -> zbus::Result<bool> {
		self.proxy.wake_allowed().await
	}

	pub async fn set_wake_allowed(&self, wake_allowed: bool) -> zbus::Result<()> {
		self.proxy.set_wake_allowed(wake_allowed).await
	}

	pub async fn manufacturer_data(&self) -> zbus::Result<HashMap<u16, zvariant::OwnedValue>> {
		self.proxy.manufacturer_data().await
	}
}

impl Deref for Device {
	type Target = Device1Proxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}

impl From<Device1Proxy<'static>> for Device {
	fn from(proxy: Device1Proxy<'static>) -> Self {
		Self { proxy }
	}
}
