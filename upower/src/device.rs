// Copyright 2021 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::proxy;
use zbus::zvariant::OwnedValue;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize_repr, Serialize_repr, OwnedValue)]
#[repr(u32)]
pub enum BatteryState {
	Unknown = 0,
	Charging = 1,
	Discharging = 2,
	Empty = 3,
	FullyCharged = 4,
	PendingCharge = 5,
	PendingDischarge = 6,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize_repr, Serialize_repr, OwnedValue)]
#[repr(u32)]
pub enum BatteryType {
	Unknown = 0,
	LinePower = 1,
	Battery = 2,
	Ups = 3,
	Monitor = 4,
	Mouse = 5,
	Keyboard = 6,
	Pda = 7,
	Phone = 8,
	MediaPlayer = 9,
	Tablet = 10,
	Computer = 11,
	GamingInput = 12,
	Pen = 13,
	Touchpad = 14,
	Modem = 15,
	Network = 16,
	Headset = 17,
	Speakers = 18,
	Headphones = 19,
	Video = 20,
	OtherAudio = 21,
	RemoteControl = 22,
	Printer = 23,
	Scanner = 24,
	Camera = 25,
	Wearable = 26,
	Toy = 27,
	BluetoothGeneric = 28,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize_repr, Serialize_repr, OwnedValue)]
#[repr(u32)]
pub enum BatteryLevel {
	Unknown = 0,
	None = 1,
	Low = 3,
	Critical = 4,
	Normal = 6,
	High = 7,
	Full = 8,
}

#[proxy(
	interface = "org.freedesktop.UPower.Device",
	default_service = "org.freedesktop.UPower",
	assume_defaults = false
)]
pub trait Device {
	#[zbus(property)]
	fn battery_level(&self) -> zbus::Result<BatteryLevel>;

	#[zbus(property)]
	fn capacity(&self) -> zbus::Result<f64>;

	#[zbus(property)]
	fn energy(&self) -> zbus::Result<f64>;

	#[zbus(property)]
	fn energy_empty(&self) -> zbus::Result<f64>;

	#[zbus(property)]
	fn energy_full(&self) -> zbus::Result<f64>;

	#[zbus(property)]
	fn energy_full_design(&self) -> zbus::Result<f64>;

	#[zbus(property)]
	fn has_history(&self) -> zbus::Result<bool>;

	#[zbus(property)]
	fn has_statistics(&self) -> zbus::Result<bool>;

	#[zbus(property)]
	fn icon_name(&self) -> zbus::Result<String>;

	#[zbus(property)]
	fn is_present(&self) -> zbus::Result<bool>;

	#[zbus(property)]
	fn is_rechargeable(&self) -> zbus::Result<bool>;

	#[zbus(property)]
	fn luminosity(&self) -> zbus::Result<f64>;

	#[zbus(property)]
	fn model(&self) -> zbus::Result<String>;

	#[zbus(property)]
	fn native_path(&self) -> zbus::Result<String>;

	#[zbus(property)]
	fn online(&self) -> zbus::Result<bool>;

	#[zbus(property)]
	fn percentage(&self) -> zbus::Result<f64>;

	#[zbus(property)]
	fn power_supply(&self) -> zbus::Result<bool>;

	fn refresh(&self) -> zbus::Result<()>;

	#[zbus(property)]
	fn serial(&self) -> zbus::Result<String>;

	#[zbus(property)]
	fn state(&self) -> zbus::Result<BatteryState>;

	#[zbus(property)]
	fn temperature(&self) -> zbus::Result<f64>;

	#[zbus(property)]
	fn time_to_empty(&self) -> zbus::Result<i64>;

	#[zbus(property)]
	fn time_to_full(&self) -> zbus::Result<i64>;

	#[zbus(property, name = "Type")]
	fn type_(&self) -> zbus::Result<BatteryType>;

	#[zbus(property)]
	fn vendor(&self) -> zbus::Result<String>;

	#[zbus(property)]
	fn voltage(&self) -> zbus::Result<f64>;
}
