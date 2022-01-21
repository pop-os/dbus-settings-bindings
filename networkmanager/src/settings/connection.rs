// SPDX-License-Identifier: MPL-2.0

use crate::interface::settings::connection::ConnectionSettingsProxy;
use derive_builder::Builder;
use std::ops::Deref;

#[derive(Debug)]
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
	($name:ident, $(($arg:ident($rename:expr): $arg_ty:ty)),*) => {
		#[derive(Debug, Builder, Clone, zbus::zvariant::DeserializeDict, zbus::zvariant::SerializeDict, zbus::zvariant::Type)]
		#[zvariant(signature = "dict")]
		pub struct $name {
			$(
				#[zvariant(rename = $rename)]
				#[builder(setter(into, strip_option))]
				pub $arg: Option<$arg_ty>,
			)*
		}
	};
}

derive_value_build!(
	Settings,
	(connection("connection"): ConnectionSettings),
	(ethernet("802-3-ethernet"): EthernetSettings),
	(wifi("802-11-wireless"): WifiSettings),
	(bluetooth("bluetooth"): BluetoothSettings)
);

derive_value_build!(
	ConnectionSettings,
	(auth_retries("auth-retries"): i32),
	(autoconnect("autoconnect"): bool),
	(autoconnect_priority("autoconnect-priority"): i32),
	(autoconnect_retries("autoconnect-retries"): i32),
	(gateway_ping_timeout("gateway-ping-timeout"): u32),
	(id("id"): String),
	(interface_name("interface-name"): String),
	(lldp("lldp"): i32),
	(llmnr("llmnr"): i32),
	(master("master"): String),
	(mdns("mdns"): i32),
	(mud_url("mud_url"): String),
	(multi_connect("multi-connect"): String),
	(permissions("permissions"): Vec<String>),
	(read_only("read-only"): bool),
	(secondaries("secondaries"): Vec<String>),
	(stable_id("stable-id"): String),
	(type_("type"): String),
	(uuid("uuid"): String),
	(wait_device_timeout("wait-device-timeout"): i32),
	(zone("zone"): String)
);

derive_value_build!(
	EthernetSettings,
	(assigned_mac_address("assigned-mac-address"): String),
	(auto_negotiate("auto-negotiate"): bool),
	(duplex("duplex"): String),
	(generate_mac_address_mask("generate-mac-address-mask"): String),
	(mtu("mtu"): u32),
	(port("port"): String),
	(speed("speed"): u32),
	(wake_on_lan("wake-on-lan"): u32),
	(wake_on_lan_password("wake-on-lan-password"): String)
);

derive_value_build!(
	WifiSettings,
	(assigned_mac_address("assigned-mac-address"): String),
	(band("band"): String),
	(bssid("bssid"): Vec<u8>),
	(channel("channel"): u32),
	(cloned_mac_address("cloned-mac-address"): Vec<u8>),
	(generate_mac_address_mask("generate-mac-address-mask"): String),
	(hidden("hidden"): bool),
	(mac_address("mac-address"): Vec<u8>),
	(mac_address_blacklist("mac-address-blacklist"): Vec<String>),
	(mac_address_randomization("mac-address-randomization"): u32),
	(mode("mode"): String),
	(mtu("mtu"): u32),
	(powersave("powersave"): u32),
	(rate("rate"): u32),
	(seen_bssids("seen-bssids"): Vec<String>),
	(ssid("ssid"): Vec<u8>),
	(tx_power("tx-power"): u32),
	(wake_on_wlan("wake-on-wlan"): u32)
);

derive_value_build!(
	BluetoothSettings,
	(bdaddr("bdaddr"): Vec<u8>),
	(type_("type"): String)
);
