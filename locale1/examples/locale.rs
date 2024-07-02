#[tokio::main]
async fn main() -> zbus::Result<()> {
	let connection = zbus::Connection::system().await?;

	let proxy = locale1::locale1Proxy::new(&connection).await?;

	println!("Locale {:?}", proxy.locale().await?);
	println!("Layout {:?}", proxy.x11layout().await?);
	println!("Model {:?}", proxy.x11model().await?);
	println!("Options {:?}", proxy.x11options().await?);
	println!("Variant {:?}", proxy.x11variant().await?);

	Ok(())
}
