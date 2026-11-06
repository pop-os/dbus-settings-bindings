use crate::manager::ManagerProxy;
use crate::seat::SeatProxy;
use futures_lite::future;

#[test]
fn timestamps_async() {
	future::block_on(async {
		let connection = zbus::Connection::system().await.unwrap();
		let manager = ManagerProxy::new(&connection).await.unwrap();
		let seats = manager.list_seats().await.unwrap();
		let seat = SeatProxy::builder(&connection)
			.path(seats[0].path())
			.build()
			.await
			.unwrap();

		assert!(seat.active_session().await.is_ok());
	});
}

#[test]
fn properties_async() {
	future::block_on(async {
		let connection = zbus::Connection::system().await.unwrap();
		let manager = ManagerProxy::new(&connection).await.unwrap();
		let seats = manager.list_seats().await.unwrap();
		let seat = SeatProxy::builder(&connection)
			.path(seats[0].path())
			.build()
			.await
			.unwrap();

		assert!(seat.active_session().await.is_ok());
		assert!(seat.can_graphical().await.is_ok());
		assert!(seat.can_TTY().await.is_ok());
		assert!(seat.id().await.is_ok());
		assert!(seat.idle_hint().await.is_ok());
		assert!(seat.idle_since_hint().await.is_ok());
		assert!(seat.idle_since_hint_monotonic().await.is_ok());
		assert!(seat.sessions().await.is_ok());
	});
}
