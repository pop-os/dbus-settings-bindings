// SPDX-License-Identifier: MPL-2.0

use crate::interface::settings::connection::ConnectionSettingsProxy;
use derive_builder::Builder;
use std::ops::Deref;

pub struct Connection<'a>(ConnectionSettingsProxy<'a>);

impl<'a> Deref for Connection<'a> {
	type Target = ConnectionSettingsProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<ConnectionSettingsProxy<'a>> for Connection<'a> {
	fn from(conn: ConnectionSettingsProxy<'a>) -> Self {
		Connection(conn)
	}
}

macro_rules! derive_value_build {
	($name:ident, $(($arg:ident: $arg_ty:ty)),*) => {
		#[derive(Builder, serde::Deserialize, serde::Serialize)]
		pub struct $name {
			$(
				#[builder(setter(into, strip_option))]
				#[serde(skip_serializing_if = "Option::is_none")]
				$arg: Option<$arg_ty>,
			)*
		}
		impl $name {
			pub fn build<'a>(&'a self) -> std::collections::HashMap<String, zbus::zvariant::Value<'a>> {
				let mut out = std::collections::HashMap::new();
				$(
					if let Some(val) = &self.$arg {
						out.insert(stringify!($arg).trim_end_matches("_").replace("_", "-"), zbus::zvariant::Value::from(val));
					}
				)*
				out
			}
		}
	};
}

derive_value_build!(
	ConnectionSettings,
	(auth_retries: i32),
	(autoconnect: bool),
	(autoconnect_priority: i32),
	(autoconnect_retries: i32),
	(gateway_ping_timeout: u32),
	(id: String),
	(interface_name: String),
	(lldp: i32),
	(llmnr: i32),
	(master: String),
	(mdns: i32),
	(mud_url: String),
	(multi_connect: String),
	(permissions: Vec<String>),
	(read_only: bool),
	(secondaries: Vec<String>),
	(stable_id: String),
	(type_: String),
	(uuid: String),
	(wait_device_timeout: i32),
	(zone: String)
);

derive_value_build!(
	EthernetSettings,
	(assigned_mac_address: String),
	(auto_negotiate: bool),
	(duplex: String),
	(generate_mac_address_mask: String),
	(mtu: u32),
	(port: String),
	(speed: u32),
	(wake_on_lan: u32),
	(wake_on_lan_password: String)
);

derive_value_build!(
	WifiSettings,
	(assigned_mac_address: String),
	(band: String),
	(bssid: Vec<u8>),
	(channel: u32),
	(cloned_mac_address: Vec<u8>),
	(generate_mac_address_mask: String),
	(hidden: bool),
	(mac_address: Vec<u8>),
	(mac_address_blacklist: Vec<String>),
	(mac_address_randomization: u32),
	(mode: String),
	(mtu: u32),
	(powersave: u32),
	(rate: u32),
	(seen_bssids: Vec<String>),
	(ssid: Vec<u8>),
	(tx_power: u32),
	(wake_on_wlan: u32)
);

derive_value_build!(BluetoothSettings, (bdaddr: Vec<u8>), (type_: String));
