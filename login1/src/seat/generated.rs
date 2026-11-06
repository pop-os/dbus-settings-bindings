//! # `DBus` interface proxy for: `org.freedesktop.login1.Seat`

#![allow(non_snake_case)]

use zbus::proxy;

use crate::TimeStamp;

use super::SessionPath;

#[proxy(
	interface = "org.freedesktop.login1.Seat",
	default_service = "org.freedesktop.login1",
	default_path = "/org/freedesktop/login1"
)]
pub trait Seat {
	/// ActivateSession method
	#[inline]
	fn activate_session(&self, session_id: &str) -> zbus::Result<()>;

	/// SwitchTo method
	#[inline]
	fn switch_to(&self, vtnr: u32) -> zbus::Result<()>;

	/// SwitchToNext method
	#[inline]
	fn switch_to_next(&self) -> zbus::Result<()>;

	/// SwitchToPrevious method
	#[inline]
	fn switch_to_previous(&self) -> zbus::Result<()>;

	/// Terminate method
	#[inline]
	fn terminate(&self) -> zbus::Result<()>;

	/// ActiveSession property
	#[zbus(property)]
	#[inline]
	fn active_session(&self) -> zbus::Result<SessionPath>;

	/// CanGraphical property
	#[zbus(property)]
	#[inline]
	fn can_graphical(&self) -> zbus::Result<bool>;

	/// CanTTY property
	#[zbus(property)]
	#[inline]
	fn can_TTY(&self) -> zbus::Result<bool>;

	/// Id property
	#[zbus(property)]
	#[inline]
	fn id(&self) -> zbus::Result<String>;

	/// IdleHint property
	#[zbus(property)]
	#[inline]
	fn idle_hint(&self) -> zbus::Result<bool>;

	/// IdleSinceHint property
	#[zbus(property)]
	#[inline]
	fn idle_since_hint(&self) -> zbus::Result<TimeStamp>;

	/// IdleSinceHintMonotonic property
	#[zbus(property)]
	#[inline]
	fn idle_since_hint_monotonic(&self) -> zbus::Result<TimeStamp>;

	/// Sessions property
	#[zbus(property)]
	#[inline]
	fn sessions(&self) -> zbus::Result<Vec<(String, zbus::OwnedObjectPath)>>;
}
