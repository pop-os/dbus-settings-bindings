#[tokio::main]
async fn main() -> zbus::Result<()> {
	let connection = zbus::Connection::system().await?;

	let proxy = switcheroo_control::SwitcherooControlProxy::new(&connection).await?;

	println!("GPUs: {:?}", proxy.get_gpus().await?);
	println!("HasDualGpu: {}", proxy.has_dual_gpu().await?);
	println!("NumGPUs: {}", proxy.num_gpus().await?);

	Ok(())
}
