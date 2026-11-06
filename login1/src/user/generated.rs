//! # `DBus` interface proxy for: `org.freedesktop.login1.User`

#![allow(non_snake_case)]

use zbus::proxy;

use crate::{SomePath, TimeStamp};

use super::UserState;

#[proxy(
	interface = "org.freedesktop.login1.User",
	default_service = "org.freedesktop.login1",
	default_path = "/org/freedesktop/login1"
)]
pub trait User {
	/// Kill method
	#[inline]
	fn kill(&self, signal_number: i32) -> zbus::Result<()>;

	/// Terminate method
	#[inline]
	fn terminate(&self) -> zbus::Result<()>;

	/// Display property
	#[zbus(property)]
	#[inline]
	fn display(&self) -> zbus::Result<SomePath>;

	/// GID property
	#[zbus(property)]
	#[inline]
	fn GID(&self) -> zbus::Result<u32>;

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

	/// Linger property
	#[zbus(property)]
	#[inline]
	fn linger(&self) -> zbus::Result<bool>;

	/// Name property
	#[zbus(property)]
	#[inline]
	fn name(&self) -> zbus::Result<String>;

	/// RuntimePath property
	#[zbus(property)]
	#[inline]
	fn runtime_path(&self) -> zbus::Result<String>;

	/// Service property
	#[zbus(property)]
	#[inline]
	fn service(&self) -> zbus::Result<String>;

	/// Sessions property
	#[zbus(property)]
	#[inline]
	fn sessions(&self) -> zbus::Result<Vec<(String, zbus::OwnedObjectPath)>>;

	/// Slice property
	#[zbus(property)]
	#[inline]
	fn slice(&self) -> zbus::Result<String>;

	/// State property
	#[zbus(property)]
	#[inline]
	fn state(&self) -> zbus::Result<UserState>;

	/// Timestamp property
	#[zbus(property)]
	#[inline]
	fn timestamp(&self) -> zbus::Result<TimeStamp>;

	/// TimestampMonotonic property
	#[zbus(property)]
	#[inline]
	fn timestamp_monotonic(&self) -> zbus::Result<TimeStamp>;

	/// UID property
	#[zbus(property)]
	#[inline]
	fn UID(&self) -> zbus::Result<u32>;
}
