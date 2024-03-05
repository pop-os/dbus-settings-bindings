// SPDX-License-Identifier: MPL-2.0
//! # DBus interface proxies for: `org.freedesktop.NetworkManager.Device.Wired`
//!
//! This code was generated by `zbus-xmlgen` `2.0.0` from DBus introspection data.
//! Source: `Interface '/org/freedesktop/NetworkManager/Devices/2' from service 'org.freedesktop.NetworkManager' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::proxy;

#[proxy(
	interface = "org.freedesktop.NetworkManager.Device.Wired",
	default_service = "org.freedesktop.NetworkManager"
)]
trait WiredDevice {
	/// Carrier property
	#[zbus(property)]
	fn carrier(&self) -> zbus::Result<bool>;

	/// HwAddress property
	#[zbus(property)]
	fn hw_address(&self) -> zbus::Result<String>;

	/// PermHwAddress property
	#[zbus(property)]
	fn perm_hw_address(&self) -> zbus::Result<String>;

	/// S390Subchannels property
	#[zbus(property)]
	fn s390subchannels(&self) -> zbus::Result<Vec<String>>;

	/// Speed property
	#[zbus(property)]
	fn speed(&self) -> zbus::Result<u32>;
}
