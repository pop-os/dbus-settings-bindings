// Copyright 2021 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

extern crate upower_dbus;

use futures::stream::StreamExt;
use upower_dbus::UPowerProxy;

fn main() -> zbus::Result<()> {
	futures::executor::block_on(async move {
		let connection = zbus::Connection::system().await?;

		let upower = UPowerProxy::new(&connection).await?;

		println!("On Battery: {:?}", upower.on_battery().await);

		let mut stream = upower.receive_on_battery_changed().await;

		while let Some(event) = stream.next().await {
			eprintln!("{:?}", event.get().await);
		}

		Ok(())
	})
}
