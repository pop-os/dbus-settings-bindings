use futures::StreamExt;

#[tokio::main]
async fn main() -> zbus::Result<()> {
	let connection = zbus::Connection::session().await?;

	let proxy = cosmic_settings_daemon::CosmicSettingsDaemonProxy::new(&connection).await?;

	println!("Display brightness: {}", proxy.display_brightness().await?);
	println!(
		"Keyboard brightness: {}",
		proxy.keyboard_brightness().await?
	);

	let (config_path, name) = proxy
		.watch_config("com.system76.CosmicTheme.Light", 1)
		.await?;
	dbg!(&config_path, &name);
	let config_proxy = cosmic_settings_daemon::ConfigProxy::builder(&connection)
		.path(config_path)?
		.destination(name)?
		.build()
		.await?;
	let mut stream = config_proxy.receive_changed().await?;

	println!("Watching config for the libcosmic light theme...");
	println!("Change the light theme in Settings > Appearance to trigger a signal.");
	while let Some(c) = stream.next().await {
		let c = c.args()?;
		println!("Config changed: {} {}", c.id, c.key);
	}
	Ok(())
}
