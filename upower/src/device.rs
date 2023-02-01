// Copyright 2021 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::dbus_proxy;
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

#[dbus_proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower",
    assume_defaults = false
)]
trait Device {
    #[dbus_proxy(property)]
    fn battery_level(&self) -> zbus::Result<BatteryLevel>;

    #[dbus_proxy(property)]
    fn capacity(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn energy(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn energy_empty(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn energy_full(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn energy_full_design(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn has_history(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn has_statistics(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn icon_name(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn is_present(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn is_rechargeable(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn luminosity(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn model(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn native_path(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn online(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn percentage(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property)]
    fn power_supply(&self) -> zbus::Result<bool>;

    fn refresh(&self) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn serial(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn state(&self) -> zbus::Result<BatteryState>;

    #[dbus_proxy(property)]
    fn temperature(&self) -> zbus::Result<f64>;

    #[dbus_proxy(property, name = "Type")]
    fn type_(&self) -> zbus::Result<BatteryType>;

    #[dbus_proxy(property)]
    fn vendor(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn voltage(&self) -> zbus::Result<f64>;
}
