// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;
use zbus::{dbus_proxy, zvariant};

#[dbus_proxy(
	async_name = "Proxy",
	blocking_name = "BlockingProxy",
	interface = "org.freedesktop.portal.Request",
	default_service = "org.freedesktop.portal.Desktop",
	default_path = "/org/freedesktop/portal/desktop"
)]
trait Request {
	fn close(&self) -> zbus::Result<()>;

	#[dbus_proxy(signal)]
	fn response(
		&self,
		response: u32,
		results: HashMap<&str, zvariant::Value<'_>>,
	) -> zbus::Result<()>;
}
