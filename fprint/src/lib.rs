// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use zbus::proxy;

#[proxy(
	interface = "net.reactivated.Fprint.Manager",
	default_service = "net.reactivated.Fprint",
	default_path = "/net/reactivated/Fprint/Manager"
)]
pub trait FprintManager {
	fn get_default_device(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

	fn get_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}

#[proxy(
	interface = "net.reactivated.Fprint.Device",
	default_service = "net.reactivated.Fprint",
	assume_defaults = true
)]
pub trait FprintDevice {
	fn claim(&self, username: &str) -> zbus::Result<()>;

	fn delete_enrolled_finger(&self, finger_name: &str) -> zbus::Result<()>;

	fn delete_enrolled_fingers(&self, username: &str) -> zbus::Result<()>;

	fn delete_enrolled_fingers2(&self) -> zbus::Result<()>;

	fn enroll_start(&self, finger_name: &str) -> zbus::Result<()>;

	fn enroll_stop(&self) -> zbus::Result<()>;

	fn list_enrolled_fingers(&self, username: &str) -> zbus::Result<Vec<String>>;

	fn release(&self) -> zbus::Result<()>;

	fn verify_start(&self, finger_name: &str) -> zbus::Result<()>;

	fn verify_stop(&self) -> zbus::Result<()>;

	#[zbus(signal)]
	fn enroll_status(&self, result: &str, done: bool) -> zbus::Result<()>;

	#[zbus(signal)]
	fn verify_finger_selected(&self, finger_name: &str) -> zbus::Result<()>;

	#[zbus(signal)]
	fn verify_status(&self, result: &str, done: bool) -> zbus::Result<()>;

	#[zbus(property, name = "finger-needed")]
	fn finger_needed(&self) -> zbus::Result<bool>;

	#[zbus(property, name = "finger-present")]
	fn finger_present(&self) -> zbus::Result<bool>;

	#[zbus(property, name = "name")]
	fn name(&self) -> zbus::Result<String>;

	#[zbus(property, name = "num-enroll-stages")]
	fn num_enroll_stages(&self) -> zbus::Result<i32>;

	#[zbus(property, name = "scan-type")]
	fn scan_type(&self) -> zbus::Result<String>;
}
