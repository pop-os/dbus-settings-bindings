// Copyright 2021 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use zbus::dbus_proxy;

use crate::device::{DeviceProxy, DeviceProxyBlocking};

#[dbus_proxy(interface = "org.freedesktop.UPower", assume_defaults = true)]
trait UPower {
    /// EnumerateDevices method
    fn enumerate_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// GetCriticalAction method
    fn get_critical_action(&self) -> zbus::Result<String>;

    /// GetDisplayDevice method
    #[dbus_proxy(object = "Device")]
    fn get_display_device(&self);

    /// DeviceAdded signal
    #[dbus_proxy(signal)]
    fn device_added(&self, device: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// DeviceRemoved signal
    #[dbus_proxy(signal)]
    fn device_removed(&self, device: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// DaemonVersion property
    #[dbus_proxy(property)]
    fn daemon_version(&self) -> zbus::Result<String>;

    /// LidIsClosed property
    #[dbus_proxy(property)]
    fn lid_is_closed(&self) -> zbus::Result<bool>;

    /// LidIsPresent property
    #[dbus_proxy(property)]
    fn lid_is_present(&self) -> zbus::Result<bool>;

    /// OnBattery property
    #[dbus_proxy(property)]
    fn on_battery(&self) -> zbus::Result<bool>;
}
