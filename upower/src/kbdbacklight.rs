// Copyright 2021 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use zbus::proxy;

#[proxy(
	default_service = "org.freedesktop.UPower",
	interface = "org.freedesktop.UPower.KbdBacklight",
	default_path = "/org/freedesktop/UPower/KbdBacklight"
)]
pub trait KbdBacklight {
	/// GetBrightness method
	fn get_brightness(&self) -> zbus::Result<i32>;

	/// GetMaxBrightness method
	fn get_max_brightness(&self) -> zbus::Result<i32>;

	/// SetBrightness method
	fn set_brightness(&self, value: i32) -> zbus::Result<()>;

	/// BrightnessChanged signal
	#[zbus(signal)]
	fn brightness_changed(&self, value: i32) -> zbus::Result<()>;

	/// BrightnessChangedWithSource signal
	#[zbus(signal)]
	fn brightness_changed_with_source(&self, value: i32, source: &str) -> zbus::Result<()>;
}
