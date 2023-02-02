use std::ops::Deref;

use crate::bindings::bluez::ProfileManager1Proxy;

pub struct ProfileManager {
	proxy: ProfileManager1Proxy<'static>,
}

impl ProfileManager {
	pub async fn new(connection: &zbus::Connection) -> Result<Self, zbus::Error> {
		let proxy = ProfileManager1Proxy::new(connection).await?;
		Ok(Self { proxy })
	}

	pub async fn register_profile(
		&self,
		profile: &zbus::zvariant::ObjectPath<'_>,
		uuid: &str,
		options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
	) -> zbus::Result<()> {
		self.proxy.register_profile(profile, uuid, options).await
	}

	pub async fn unregister_profile(
		&self,
		profile: &zbus::zvariant::ObjectPath<'_>,
	) -> zbus::Result<()> {
		self.proxy.unregister_profile(profile).await
	}
}

impl Deref for ProfileManager {
	type Target = ProfileManager1Proxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}

impl From<ProfileManager1Proxy<'static>> for ProfileManager {
	fn from(proxy: ProfileManager1Proxy<'static>) -> Self {
		Self { proxy }
	}
}
