use std::process::ExitCode;

#[tokio::main]
async fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
	let connection = zbus::Connection::system().await?;

	let adapters = bluez_zbus::get_adapters(&connection).await?;

	// if adapters.is_empty() {
	// 	eprintln!("No adapter found");
	// 	return Ok(ExitCode::FAILURE);
	// }

	// if adapters.len() > 1 {
	// 	eprintln!("More than one adapter found. Using the first one");
	// }
	// let adapter = adapters.first().unwrap();

	let mut parser = pico_args::Arguments::from_env();

	match parser.subcommand()?.as_deref() {
		Some("connected-devices") => match bluez_zbus::get_devices(&connection, None).await {
			Err(why) => {
				eprintln!("error: could not get devices: {why}");
				return Ok(ExitCode::FAILURE);
			}
			Ok(devices) => {
				for (_path, proxy) in devices {
					if !proxy.device.connected().await? {
						continue;
					}
					println!(
						"{} ({})",
						proxy
							.device
							.name()
							.await
							.unwrap_or(proxy.device.address().await.unwrap()),
						proxy
							.device
							.inner()
							.get_property::<String>("Icon")
							.await
							.unwrap_or("unknown".to_owned())
					);
				}
			}
		},

		Some("paired-devices") => match bluez_zbus::get_devices(&connection, None).await {
			Err(why) => {
				eprintln!("error: could not get devices: {why}");
				return Ok(ExitCode::FAILURE);
			}
			Ok(devices) => {
				for (_path, proxy) in devices {
					if !proxy.device.paired().await? {
						continue;
					}
					println!(
						"{} ({})",
						proxy
							.device
							.name()
							.await
							.unwrap_or(proxy.device.address().await.unwrap()),
						proxy
							.device
							.inner()
							.get_property::<String>("Icon")
							.await
							.unwrap_or("unknown".to_owned())
					);
				}
			}
		},

		Some("nearby-devices") => {
			futures_util::future::join_all(
				adapters
					.iter()
					.map(|(_path, adapter)| adapter.start_discovery()),
			)
			.await;
			match bluez_zbus::get_devices(&connection, None).await {
				Err(why) => {
					eprintln!("error: could not get devices: {why}");
					return Ok(ExitCode::FAILURE);
				}
				Ok(devices) => {
					for (_path, proxy) in devices {
						if proxy.device.paired().await? {
							continue;
						}
						println!(
							"{} ({})",
							proxy
								.device
								.name()
								.await
								.unwrap_or(proxy.device.address().await.unwrap()),
							proxy
								.device
								.inner()
								.get_property::<String>("Icon")
								.await
								.unwrap_or("unknown".to_owned())
						);
					}
				}
			}
		}

		Some("connect") => match parser.free_from_str::<String>().ok().as_deref() {
			Some(addr_or_alias) => match bluez_zbus::get_devices(&connection, None).await {
				Err(why) => {
					eprintln!("error: could not get devices: {why}");
					return Ok(ExitCode::FAILURE);
				}
				Ok(devices) => {
					let devices: Vec<bluez_zbus::BluetoothDevice> = futures_util::future::join_all(
						devices.into_iter().map(|(_path, proxy)| async {
							match (proxy.device.name().await, proxy.device.address().await) {
								(Ok(alias), _) if alias == addr_or_alias => Some(proxy),
								(_, Ok(addr)) if addr == addr_or_alias => Some(proxy),
								_ => None,
							}
						}),
					)
					.await
					.into_iter()
					.flatten()
					.collect();
					if devices.is_empty() {
						eprintln!("No device found");
						return Ok(ExitCode::FAILURE);
					}

					if devices.len() > 1 {
						eprintln!("More than one one found. Use the address.");
						return Ok(ExitCode::FAILURE);
					}
					let proxy = devices.first().unwrap();

					if proxy.device.connected().await? {
						eprintln!("Device already connected.");
						return Ok(ExitCode::FAILURE);
					}

					if let Err(why) = proxy.device.connect().await {
						eprintln!("error: could not connect: {why}");
						return Ok(ExitCode::FAILURE);
					}
				}
			},

			None => {
				eprintln!("error: device address or alias missing");
				return Ok(ExitCode::FAILURE);
			}
		},

		Some("disconnect") => match parser.free_from_str::<String>().ok().as_deref() {
			Some(addr_or_alias) => match bluez_zbus::get_devices(&connection, None).await {
				Err(why) => {
					eprintln!("error: could not get devices: {why}");
					return Ok(ExitCode::FAILURE);
				}
				Ok(devices) => {
					let devices: Vec<bluez_zbus::BluetoothDevice> = futures_util::future::join_all(
						devices.into_iter().map(|(_path, proxy)| async {
							match (proxy.device.name().await, proxy.device.address().await) {
								(Ok(alias), _) if alias == addr_or_alias => Some(proxy),
								(_, Ok(addr)) if addr == addr_or_alias => Some(proxy),
								_ => None,
							}
						}),
					)
					.await
					.into_iter()
					.flatten()
					.collect();

					if devices.is_empty() {
						eprintln!("No device found");
						return Ok(ExitCode::FAILURE);
					}

					if devices.len() > 1 {
						eprintln!("More than one one found. Use the address.");
						return Ok(ExitCode::FAILURE);
					}
					let proxy = devices.first().unwrap();

					if !proxy.device.connected().await? {
						eprintln!("Device not connected.");
						return Ok(ExitCode::FAILURE);
					}

					if let Err(why) = proxy.device.disconnect().await {
						eprintln!("error: could not disconnect: {why}");
						return Ok(ExitCode::FAILURE);
					}
				}
			},

			None => {
				eprintln!("error: device address or alias missing");
				return Ok(ExitCode::FAILURE);
			}
		},

		Some("forget") => match parser.free_from_str::<String>().ok().as_deref() {
			Some(addr_or_alias) => match bluez_zbus::get_devices(&connection, None).await {
				Err(why) => {
					eprintln!("error: could not get devices: {why}");
					return Ok(ExitCode::FAILURE);
				}
				Ok(devices) => {
					let devices: Vec<bluez_zbus::BluetoothDevice> = futures_util::future::join_all(
						devices.into_iter().map(|(_path, proxy)| async {
							match (proxy.device.name().await, proxy.device.address().await) {
								(Ok(alias), _) if alias == addr_or_alias => Some(proxy),
								(_, Ok(addr)) if addr == addr_or_alias => Some(proxy),
								_ => None,
							}
						}),
					)
					.await
					.into_iter()
					.flatten()
					.collect();
					if devices.is_empty() {
						eprintln!("No device found");
						return Ok(ExitCode::FAILURE);
					}

					if devices.len() > 1 {
						eprintln!("More than one one found. Use the address.");
						return Ok(ExitCode::FAILURE);
					}
					let proxy = devices.first().unwrap();

					if !proxy.device.paired().await? {
						eprintln!("Device not connected.");
						return Ok(ExitCode::FAILURE);
					}

					if proxy.device.connected().await? {
						eprintln!("Cannot remove a connected proxy.device.");
						return Ok(ExitCode::FAILURE);
					}
				}
			},

			None => {
				eprintln!("error: device address or alias missing");
				return Ok(ExitCode::FAILURE);
			}
		},
		_ => print_help(),
	}

	Ok(ExitCode::SUCCESS)
}

fn print_help() {
	println!(
		"\
bluetoothctl

USAGE:
    bluetoothctl connected-devices
    bluetoothctl paired-devices
    bluetoothctl nearby-devices
    bluetoothctl connect ADDRESS
    bluetoothctl disconnect ADDRESS
    bluetoothctl forget ADDRESS
"
	);
}
