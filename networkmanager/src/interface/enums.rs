// SPDX-License-Identifier: MPL-2.0
use bitflags::bitflags;

pub enum State {
	Asleep,
	Disconnected,
	Disconnecting,
	Connecting,
	ConnectedLocal,
	ConnectedSite,
	ConnectedGlobal,
	Unknown,
}

impl From<u32> for State {
	fn from(state: u32) -> State {
		match state {
			10 => State::Asleep,
			20 => State::Disconnected,
			30 => State::Disconnecting,
			40 => State::Connecting,
			50 => State::ConnectedLocal,
			60 => State::ConnectedSite,
			70 => State::ConnectedGlobal,
			_ => State::Unknown,
		}
	}
}

pub enum ConnectivityState {
	None,
	Portal,
	Loss,
	Full,
	Unknown,
}

impl From<u32> for ConnectivityState {
	fn from(state: u32) -> ConnectivityState {
		match state {
			1 => ConnectivityState::None,
			2 => ConnectivityState::Portal,
			3 => ConnectivityState::Loss,
			4 => ConnectivityState::Full,
			_ => ConnectivityState::Unknown,
		}
	}
}

pub enum DeviceType {
	Ethernet,
	Wifi,
	Bluetooth,
	Generic,
	Other,
	Unknown,
}

impl From<u32> for DeviceType {
	fn from(device_type: u32) -> DeviceType {
		match device_type {
			1 => DeviceType::Ethernet,
			2 => DeviceType::Wifi,
			5 => DeviceType::Bluetooth,
			14 => DeviceType::Generic,
			3..=32 => DeviceType::Other,
			_ => DeviceType::Unknown,
		}
	}
}

pub enum DeviceState {
	Unmanaged,
	Unavailable,
	Disconnected,
	Prepare,
	Config,
	NeedAuth,
	IpConfig,
	IpCheck,
	Secondaries,
	Activated,
	Deactivating,
	Failed,
	Unknown,
}

impl From<u32> for DeviceState {
	fn from(device_state: u32) -> Self {
		match device_state {
			10 => DeviceState::Unmanaged,
			20 => DeviceState::Unavailable,
			30 => DeviceState::Disconnected,
			40 => DeviceState::Prepare,
			50 => DeviceState::Config,
			60 => DeviceState::NeedAuth,
			70 => DeviceState::IpConfig,
			80 => DeviceState::IpCheck,
			90 => DeviceState::Secondaries,
			100 => DeviceState::Activated,
			110 => DeviceState::Deactivating,
			120 => DeviceState::Failed,
			_ => DeviceState::Unknown,
		}
	}
}

pub enum WifiMode {
	AdHoc,
	Infra,
	Ap,
	Mesh,
	Unknown,
}

impl From<u32> for WifiMode {
	fn from(mode: u32) -> Self {
		match mode {
			1 => WifiMode::AdHoc,
			2 => WifiMode::Infra,
			3 => WifiMode::Ap,
			4 => WifiMode::Mesh,
			_ => WifiMode::Unknown,
		}
	}
}

bitflags! {
	pub struct DeviceCapabilities: u32 {
		const SUPPORTED = 0x00000001;
		const CARRIER_DETECT = 0x00000002;
		const SOFTWARE = 0x00000004;
		const SINGLE_ROOT_IO_VIRT = 0x00000008;
	}
}

bitflags! {
	pub struct WifiCapabilities: u32 {
		const CIPHER_WEP40 = 0x00000001;
		const CIPHER_WEP104 = 0x00000002;
		const CIPHER_TKIP = 0x00000004;
		const CIPHER_CCMP = 0x00000008;
		const WPA = 0x00000010;
		const RSN = 0x00000020;
		const AP = 0x00000040;
		const AD_HOC = 0x00000080;
		const FREQ_VALID = 0x00000100;
		const FREQ_2GHZ = 0x00000200;
		const FREQ_5GHZ = 0x00000400;
		const MESH = 0x00001000;
		const IBSS_RSN = 0x00002000;
	}
}

bitflags! {
	pub struct ActivationStateFlags: u32 {
		const IS_MASTER = 0x1;
		const IS_SLAVE = 0x2;
		const LAYER2_READY = 0x4;
		const IP4_READY = 0x8;
		const IP6_READY = 0x10;
		const MASTER_HAS_SLAVES = 0x20;
		const LIFETIME_BOUND_TO_PROFILE_VISIBILITY = 0x40;
		const EXTERNAL = 0x80;
	}
}

bitflags! {
	pub struct ApFlags: u32 {
		const PRIVACY = 0x1;
		const WPS = 0x2;
		const WPS_PBC = 0x4;
		const WPS_PIN = 0x8;
	}
}

bitflags! {
	pub struct ApSecurityFlags: u32 {
		const WEP40 = 0x1;
		const WEP104 = 0x2;
		const TKIP = 0x4;
		const CCMP = 0x8;
		const GROUP_WEP40 = 0x10;
		const GROUP_WEP104 = 0x20;
		const GROUP_TKIP = 0x40;
		const GROUP_CCMP = 0x80;
		const KEY_MGMTPSK = 0x100;
		const KEY_MGMT_802_1X = 0x200;
		const KEY_MGMT_SAE = 0x400;
		const KEY_MGMT_OWE = 0x800;
		const KEY_MGMT_OWE_TM = 0x1000;
		const KEY_MGMT_EAP_SUITE_B_192 = 0x2000;
	}
}
