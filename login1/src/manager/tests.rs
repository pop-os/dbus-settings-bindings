use crate::manager::{InhibitType, IsSupported, ManagerProxy, Mode};
use futures_lite::future;

#[test]
fn timestamps_async() {
	future::block_on(async {
		let connection = zbus::Connection::system().await.unwrap();
		let manager = ManagerProxy::new(&connection).await.unwrap();

		assert!(manager.can_suspend().await.is_ok());
	});
}

#[test]
fn properties_async() {
	future::block_on(async {
		let connection = zbus::Connection::system().await.unwrap();
		let manager = ManagerProxy::new(&connection).await.unwrap();

		assert!(manager.block_inhibited().await.is_ok());
		assert!(manager.boot_loader_entries().await.is_ok());
		assert!(manager.delay_inhibited().await.is_ok());
		assert!(manager.docked().await.is_ok());
		assert!(manager.handle_hibernate_key().await.is_ok());
		assert!(manager.handle_lid_switch().await.is_ok());
		assert!(manager.handle_lid_switch_docked().await.is_ok());
		assert!(manager.handle_lid_switch_external_power().await.is_ok());
		assert!(manager.handle_power_key().await.is_ok());
		assert!(manager.handle_suspend_key().await.is_ok());
		assert!(manager.holdoff_timeout_USec().await.is_ok());
		assert!(manager.idle_action().await.is_ok());
		assert!(manager.idle_action_USec().await.is_ok());
		assert!(manager.idle_hint().await.is_ok());
		assert!(manager.idle_since_hint().await.is_ok());
		assert!(manager.idle_since_hint_monotonic().await.is_ok());
		assert!(manager.inhibit_delay_max_USec().await.is_ok());
		assert!(manager.inhibitors_max().await.is_ok());
		assert!(manager.kill_exclude_users().await.is_ok());
		assert!(manager.kill_only_users().await.is_ok());
		assert!(manager.kill_user_processes().await.is_ok());
		assert!(manager.lid_closed().await.is_ok());
		assert!(manager.NAuto_VTs().await.is_ok());
		assert!(manager.NCurrent_inhibitors().await.is_ok());
		assert!(manager.NCurrent_sessions().await.is_ok());
		assert!(manager.on_external_power().await.is_ok());
		assert!(manager.preparing_for_shutdown().await.is_ok());
		assert!(manager.preparing_for_sleep().await.is_ok());
		assert!(manager.reboot_parameter().await.is_ok());
		assert!(manager.reboot_to_boot_loader_entry().await.is_ok());
		assert!(manager.reboot_to_boot_loader_menu().await.is_ok());
		assert!(manager.reboot_to_firmware_setup().await.is_ok());
		assert!(manager.remove_IPC().await.is_ok());
		assert!(manager.runtime_directory_inodes_max().await.is_ok());
		assert!(manager.runtime_directory_size().await.is_ok());
		assert!(manager.scheduled_shutdown().await.is_ok());
		assert!(manager.get_seat("seat0").await.is_ok());

		// Requires a valid session ID
		// assert!(manager.get_session("c4").await.is_ok());
		// Requires knowing a process ID owned by a session
		// assert!(manager.get_session_by_pid(5455).await.is_ok());

		assert!(manager.sessions_max().await.is_ok());
		// Requires a valid logged-in user ID
		// assert!(manager.get_user(1000).await.is_ok());
		// Requires knowing a process ID
		// assert!(manager.get_user_by_pid(25813).await.is_ok());
		assert!(manager.user_stop_delay_USec().await.is_ok());
		assert!(manager.wall_message().await.is_ok());
	});
}

#[test]
fn inhibitors_async() {
	future::block_on(async {
		let connection = zbus::Connection::system().await.unwrap();
		let manager = ManagerProxy::new(&connection).await.unwrap();

		assert!(manager.can_suspend().await.is_ok());

		let inhibitors = manager.list_inhibitors().await;
		let n_inhibitors = manager.NCurrent_inhibitors().await.unwrap();
		assert!(inhibitors.is_ok());
		assert_eq!(n_inhibitors, inhibitors.unwrap().len() as u64);

		let res = manager
			.inhibit(
				InhibitType::HandleHibernateKey,
				"inhibit test",
				"inhibit test",
				<&str>::from(Mode::Delay),
			)
			.await;

		assert!(res.is_err());
		res.map_err(|e| {
			if let zbus::Error::MethodError(_, data, _) = e {
				assert_eq!(
					data,
					Some("Delay inhibitors only supported for shutdown and sleep".to_owned())
				);
			}
		})
		.ok();
	});
}

#[test]
fn can_do_async() {
	future::block_on(async {
		let connection = zbus::Connection::system().await.unwrap();
		let manager = ManagerProxy::new(&connection).await.unwrap();

		let res = manager.can_suspend().await;
		assert!(res.is_ok());
		assert_eq!(res, zbus::Result::Ok(IsSupported::Yes));
	});
}
