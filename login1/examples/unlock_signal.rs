use cosmic_login1_zbus::{manager::ManagerProxy, session::SessionProxy};
use futures_lite::{StreamExt, future};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	future::block_on(async {
		let connection = zbus::Connection::system().await?;
		let manager = ManagerProxy::new(&connection).await?;
		let sessions = manager.list_sessions().await?;

		let mut seat = 0;

		for (i, s) in sessions.iter().enumerate() {
			if s.uid() >= 1000 {
				seat = i;
				break;
			}
		}

		let session = SessionProxy::builder(&connection)
			.path(sessions[seat].path())
			.build()
			.await?;

		let Ok(mut sig_iter) = session.receive_unlock().await else {
			return Ok(());
		};
		if sig_iter.next().await.is_some() {
			println!("Unlocked");
		}
		Ok::<(), Box<dyn std::error::Error>>(())
	})
}
