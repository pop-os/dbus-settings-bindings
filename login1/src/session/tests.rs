use crate::manager::ManagerProxy;
use crate::session::SessionProxy;
use futures_lite::future;

use super::SessionType;

#[test]
fn timestamps_async() {
	future::block_on(async {
		let connection = zbus::Connection::system().await.unwrap();
		let manager = ManagerProxy::new(&connection).await.unwrap();
		let sessions = manager.list_sessions().await.unwrap();
		let session = SessionProxy::builder(&connection)
			.path(sessions[0].path())
			.build()
			.await
			.unwrap();

		let time1 = session.timestamp().await.unwrap();
		assert!(time1.as_secs() > 0);

		let time2 = session.timestamp_monotonic().await.unwrap();
		assert!(time2.as_secs() > 0);
	});
}

#[test]
fn list_active_session_types_async() {
	future::block_on(async {
		let connection = zbus::Connection::system().await.unwrap();
		let manager = ManagerProxy::new(&connection).await.unwrap();
		let sessions = manager.list_sessions().await.unwrap();

		for session in sessions {
			let session_proxy = SessionProxy::builder(&connection)
				.path(session.path())
				.build()
				.await
				.unwrap();
			if session_proxy.active().await.unwrap() {
				let st = session_proxy.type_().await.unwrap();
				match st {
					SessionType::X11 => assert_eq!(st, SessionType::X11),
					SessionType::Wayland => assert_eq!(st, SessionType::Wayland),
					SessionType::MIR => assert_eq!(st, SessionType::MIR),
					SessionType::TTY => assert_eq!(st, SessionType::TTY),
					SessionType::Unspecified => {
						assert_eq!(st, SessionType::Unspecified);
					}
				}
			}
		}
	});
}

#[test]
fn properties_async() {
	future::block_on(async {
		let connection = zbus::Connection::system().await.unwrap();
		let manager = ManagerProxy::new(&connection).await.unwrap();
		let sessions = manager.list_sessions().await.unwrap();
		let session = SessionProxy::builder(&connection)
			.path(sessions[0].path())
			.build()
			.await
			.unwrap();

		assert!(session.active().await.is_ok());
		assert!(session.audit().await.is_ok());
		assert!(session.class().await.is_ok());
		assert!(session.desktop().await.is_ok());
		assert!(session.display().await.is_ok());
		assert!(session.id().await.is_ok());
		assert!(session.idle_hint().await.is_ok());
		assert!(session.idle_since_hint().await.is_ok());
		assert!(session.idle_since_hint_monotonic().await.is_ok());
		assert!(session.remote().await.is_ok());
		assert!(session.leader().await.is_ok());
		assert!(session.locked_hint().await.is_ok());
		assert!(session.name().await.is_ok());
		assert!(session.remote_host().await.is_ok());
		assert!(session.remote_user().await.is_ok());
		assert!(session.scope().await.is_ok());
		assert!(session.seat().await.is_ok());
		assert!(session.service().await.is_ok());
		assert!(session.state().await.is_ok());
		assert!(session.timestamp().await.is_ok());
		assert!(session.timestamp_monotonic().await.is_ok());
		assert!(session.TTY().await.is_ok());
		assert!(session.type_().await.is_ok());
		assert!(session.user().await.is_ok());
		assert!(session.VTNr().await.is_ok());
	});
}
