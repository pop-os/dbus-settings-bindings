// Copyright 2021 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

extern crate upower_dbus;

use upower_dbus::UPowerProxy;

fn main() -> zbus::Result<()> {
	futures::executor::block_on(async move {
		let connection = zbus::Connection::system().await?;

		let upower = UPowerProxy::new(&connection).await?;

		let device = upower.get_display_device().await?;

		println!("BatteryLevel: {:?}", device.battery_level().await);
		println!("IconName: {:?}", device.icon_name().await);
		println!("IsPresent: {:?}", device.is_present().await);
		println!("Online: {:?}", device.online().await);
		println!("Percentage: {:?}", device.percentage().await);
		println!("State: {:?}", device.state().await);
		println!("Type: {:?}", device.type_().await);

		Ok(())
	})
}
