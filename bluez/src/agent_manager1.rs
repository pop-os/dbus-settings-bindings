//! # D-Bus interface proxy for: `org.bluez.AgentManager1`
//!
//! This code was generated by `zbus-xmlgen` `4.1.0` from D-Bus introspection data.
//! Source: `Interface '/org/bluez' from service 'org.bluez' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//! This type implements the [D-Bus standard interfaces], (`org.freedesktop.DBus.*`) for which the
//! following zbus API can be used:
//!
//! * [`zbus::fdo::IntrospectableProxy`]
//!
//! Consequently `zbus-xmlgen` did not generate code for the above interfaces.
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::proxy;
#[proxy(
	interface = "org.bluez.AgentManager1",
	default_service = "org.bluez",
	default_path = "/org/bluez"
)]
pub trait AgentManager1 {
	/// RegisterAgent method
	fn register_agent(
		&self,
		agent: &zbus::zvariant::ObjectPath<'_>,
		capability: &str,
	) -> zbus::Result<()>;

	/// RequestDefaultAgent method
	fn request_default_agent(&self, agent: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

	/// UnregisterAgent method
	fn unregister_agent(&self, agent: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;
}
