// SPDX-License-Identifier: MPL-2.0

use futures_util::stream::StreamExt;
use mpris2_zbus::enumerator::Enumerator;

#[tokio::main]
async fn main() -> zbus::Result<()> {
	let connection = zbus::Connection::session().await?;
	let enumerator = Enumerator::new(&connection).await?;
	let mut stream = enumerator.receive_changes().await?;
	println!("players: {:?}", enumerator.players().await?);
	while let Some(event) = stream.next().await {
		println!("change: {:?}", event?);
	}
	Ok(())
}
