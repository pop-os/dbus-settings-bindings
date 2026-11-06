use cosmic_login1_zbus::{
	manager::ManagerProxy,
	session::{SessionClass, SessionProxy, SessionType},
};
use futures_lite::future;

// Type, Class, Active
fn main() -> Result<(), Box<dyn std::error::Error>> {
	future::block_on(async {
		let connection = zbus::Connection::system().await?;
		let manager = ManagerProxy::new(&connection).await?;
		let sessions = manager.list_sessions().await?;

		for session in sessions {
			let session_proxy = SessionProxy::builder(&connection)
				.path(session.path())
				.build()
				.await?;

			let class = session_proxy.class().await?;
			let typ = session_proxy.type_().await?;

			if class == SessionClass::User
				&& matches!(
					typ,
					SessionType::X11 | SessionType::Wayland | SessionType::MIR
				) {
				if session_proxy.active().await? {
					println!("Active graphical session found");
				} else {
					println!("Inactive graphical session found");
				}
			}
		}
		Ok::<(), Box<dyn std::error::Error>>(())
	})
}
