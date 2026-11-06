use crate::manager::ManagerProxy;
use crate::user::UserProxy;
use futures_lite::future;

#[test]
fn timestamps_async() {
	future::block_on(async {
		let connection = zbus::Connection::system().await.unwrap();
		let manager = ManagerProxy::new(&connection).await.unwrap();
		let users = manager.list_users().await.unwrap();
		let user = UserProxy::builder(&connection)
			.path(users[0].path())
			.build()
			.await
			.unwrap();

		let time1 = user.timestamp().await.unwrap();
		assert!(time1.as_secs() > 0);

		let time2 = user.timestamp_monotonic().await.unwrap();
		assert!(time2.as_secs() > 0);
	});
}

#[test]
fn properties_async() {
	future::block_on(async {
		let connection = zbus::Connection::system().await.unwrap();
		let manager = ManagerProxy::new(&connection).await.unwrap();
		let users = manager.list_users().await.unwrap();
		let user = UserProxy::builder(&connection)
			.path(users[0].path())
			.build()
			.await
			.unwrap();

		assert!(user.display().await.is_ok());
		// Special case. Exists only on users
		assert!(user.GID().await.is_ok());
		assert!(user.idle_hint().await.is_ok());
		assert!(user.idle_since_hint().await.is_ok());
		assert!(user.idle_since_hint_monotonic().await.is_ok());
		assert!(user.linger().await.is_ok());
		assert!(user.name().await.is_ok());
		assert!(user.runtime_path().await.is_ok());
		assert!(user.service().await.is_ok());
		assert!(user.slice().await.is_ok());
		assert!(user.sessions().await.is_ok());
		assert!(user.state().await.is_ok());
		assert!(user.timestamp().await.is_ok());
		assert!(user.timestamp_monotonic().await.is_ok());
		// Special case. Exists only on users
		assert!(user.UID().await.is_ok());
	});
}
