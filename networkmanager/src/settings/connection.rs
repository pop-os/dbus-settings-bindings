// SPDX-License-Identifier: MPL-2.0

use crate::interface::settings::connection::ConnectionSettingsProxy;
use derive_builder::Builder;
use std::{collections::HashMap, ops::Deref};

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
		#[derive(Debug, Default, Builder, Clone)]
		pub struct $name {
			$(
				#[builder(setter(into, strip_option))]
				pub $arg: Option<$arg_ty>,
			)*
		}

		impl $name {
			pub fn new(mut src: std::collections::HashMap<String, zbus::zvariant::OwnedValue>) -> Self {
				let mut ret = Self::default();
				$(
					if let Some(val) = src.remove($rename).and_then(|val| val.try_into().ok()) {
						ret.$arg = Some(val);
					}
				)*
				ret
			}

			pub fn build(&self) -> std::collections::HashMap<String, zbus::zvariant::Value<'_>> {
				let mut out = std::collections::HashMap::new();
				$(
					if let Some(val) = &self.$arg {
						out.insert($rename.to_string(), val.to_owned().into());
					}
				)*
				out
			}
		}
	};
}

#[derive(Debug, Default, Builder, Clone)]
pub struct Settings {
	#[builder(setter(strip_option))]
	pub connection: Option<ConnectionSettings>,
	#[builder(setter(strip_option))]
	pub ethernet: Option<EthernetSettings>,
	#[builder(setter(strip_option))]
	pub wifi: Option<WifiSettings>,
	#[builder(setter(strip_option))]
	pub bluetooth: Option<BluetoothSettings>,
	#[builder(setter(strip_option))]
	pub ipv4: Option<Ipv4Settings>,
	#[builder(setter(strip_option))]
	pub ipv6: Option<Ipv6Settings>,
	#[builder(setter(strip_option))]
	pub proxy: Option<WwwProxySettings>,
}

impl Settings {
	pub fn new(
		mut src: std::collections::HashMap<
			String,
			std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
		>,
	) -> Self {
		Self {
			connection: src.remove("connection").map(ConnectionSettings::new),
			ethernet: src.remove("802-3-ethernet").map(EthernetSettings::new),
			wifi: src.remove("802-11-wireless").map(WifiSettings::new),
			bluetooth: src.remove("bluetooth").map(BluetoothSettings::new),
			ipv4: src.remove("ipv4").map(Ipv4Settings::new),
			ipv6: src.remove("ipv6").map(Ipv6Settings::new),
			proxy: src.remove("proxy").map(WwwProxySettings::new),
		}
	}

	pub fn build(
		&self,
	) -> std::collections::HashMap<
		String,
		std::collections::HashMap<String, zbus::zvariant::Value<'_>>,
	> {
		let mut out = std::collections::HashMap::new();
		if let Some(val) = &self.connection {
			out.insert("connection".into(), val.build());
		}
		if let Some(val) = &self.ethernet {
			out.insert("802-3-ethernet".into(), val.build());
		}
		if let Some(val) = &self.wifi {
			out.insert("802-11-wireless".into(), val.build());
		}
		if let Some(val) = &self.bluetooth {
			out.insert("bluetooth".into(), val.build());
		}
		if let Some(val) = &self.ipv4 {
			out.insert("ipv4".into(), val.build());
		}
		if let Some(val) = &self.ipv6 {
			out.insert("ipv6".into(), val.build());
		}
		if let Some(val) = &self.proxy {
			out.insert("proxy".into(), val.build());
		}
		out
	}
}

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
	(timestamp("timestamp"): u64),
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
	(accept_all_mac_addresses("accept-all-mac-addresses"): i32),
	(assigned_mac_address("assigned-mac-address"): String),
	(auto_negotiate("auto-negotiate"): bool),
	(cloned_mac_address("cloned-mac-address"): Vec<u8>),
	(duplex("duplex"): String),
	(generate_mac_address_mask("generate-mac-address-mask"): String),
	(mac_address("mac-address"): Vec<u8>),
	(mac_address_blacklist("mac-address-blacklist"): Vec<String>),
	(mtu("mtu"): u32),
	(port("port"): String),
	(s390_nettype("s390-nettype"): String),
	(s390_options("s390-options"): HashMap<String, String>),
	(s390_subchannels("s390-subchannels"): Vec<String>),
	(speed("speed"): u32),
	(wake_on_lan("wake-on-lan"): u32),
	(wake_on_lan_password("wake-on-lan-password"): String)
);

derive_value_build!(
	WifiSettings,
	(ap_isolation("ap-isolation"): i32),
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

derive_value_build!(
	Ipv4Settings,
	(addresses("addresses"): Vec<Vec<u32>>),
	(dad_timeout("dad-timeout"): i32),
	(dhcp_client_id("dhcp-client-id"): String),
	(dhcp_fqdn("dhcp-fqdn"): String),
	(dhcp_hostname("dhcp-hostname"): String),
	(dhcp_hostname_flags("dhcp-hostname-flags"): u32),
	(dhcp_iaid("dhcp-iaid"): String),
	(dhcp_reject_servers("dhcp-reject-servers"): Vec<String>),
	(dhcp_send_hostname("dhcp-send-hostname"): bool),
	(dhcp_timeout("dhcp-timeout"): i32),
	(dhcp_vendor_class_identifier("dhcp-vendor-class-identifier"): String),
	(dns("dns"): Vec<u32>),
	(dns_options("dns-options"): Vec<String>),
	(dns_priority("dns-priority"): u32),
	(dns_search("dns-search"): Vec<String>),
	(ignore_auto_dns("ignore-auto-dns"): bool),
	(ignore_auto_routes("ignore-auto-routes"): bool),
	(may_fail("may-fail"): bool),
	(method("method"): String),
	(never_default("never-default"): bool),
	(ra_timeout("ra-timeout"): i32),
	(route_metric("route-metric"): i32),
	(route_table("route-table"): u32),
	(routes("routes"): Vec<Vec<u32>>)
);

derive_value_build!(
	Ipv6Settings,
	(addr_gen_mode("addr-gen-mode"): i32),
	(addresses("addresses"): Vec<String>),
	(dad_timeout("dad-timeout"): i32),
	(dhcp_duid("dhcp-duid"): Vec<u8>),
	(dhcp_hostname("dhcp-hostname"): String),
	(dhcp_hostname_flags("dhcp-hostname-flags"): u32),
	(dhcp_iaid("dhcp-iaid"): String),
	(dhcp_reject_servers("dhcp-reject-servers"): Vec<String>),
	(dhcp_send_hostname("dhcp-send-hostname"): bool),
	(dhcp_timeout("dhcp-timeout"): i32),
	(dns("dns"): Vec<Vec<u8>>),
	(dns_options("dns-options"): Vec<String>),
	(dns_priority("dns-priority"): i32),
	(dns_search("dns-search"): Vec<String>),
	(gateway("gateway"): String),
	(ignore_auto_dns("ignore-auto-dns"): bool),
	(ignore_auto_routes("ignore-auto-routes"): bool),
	(ip6_privacy("ip6-privacy"): i32),
	(may_fail("may-fail"): bool),
	(method("method"): String),
	(never_default("never-default"): bool),
	(ra_timeout("ra-timeout"): i32),
	(route_metric("route-metric"): i32),
	(route_table("route-table"): u32),
	(routes("routes"): Vec<String>),
	(token("token"): String)
);

derive_value_build!(
	WwwProxySettings,
	(browser_only("browser-only"): bool),
	(method("method"): i32),
	(pac_script("pac-script"): String),
	(pac_url("pac-url"): String)
);

#[derive(Debug, Default, Builder, Clone)]
pub struct Secrets {
	#[builder(setter(strip_option))]
	pub wifi: Option<WifiSecurity>,
}

impl Secrets {
	pub async fn new(connection: &Connection<'_>) -> Self {
		Self {
			wifi: connection
				.get_secrets("802-11-wireless-security")
				.await
				.ok()
				.and_then(|mut s| s.remove("802-11-wireless-security").map(WifiSecurity::new)),
		}
	}
}

derive_value_build!(
	WifiSecurity,
	(psk("psk"): String),
	(key_mgmt("key-mgmt"): String),
	(auth_alg("auth-alg"): String),
	(leap_password("leap-password"): String),
	(leap_password_flags("leap-password-flags"): u32),
	(leap_username("leap-username"): String),
	(pairwise("pairwise"): Vec<String>),
	(pmf("pmf"): u32),
	(psk_flags("psk-flags"): u32),
	(wep_key_flags("wep-key-flags"): u32),
	(wep_key_type("wep-key-type"): u32),
	(wep_key0("wep-key0"): String),
	(wep_key1("wep-key1"): String),
	(wep_key2("wep-key2"): String),
	(wep_key3("wep-key3"): String),
	(wep_tx_keyidx("wep-tx-keyidx"): u32),
	(wps_method("wps-method"): u32)
);
