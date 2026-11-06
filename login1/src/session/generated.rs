//! # `DBus` interface proxy for: `org.freedesktop.login1.Session`

#![allow(non_snake_case)]

use zbus::{OwnedFd, proxy};

use crate::{SomePath, TimeStamp};

use super::{Device, SessionClass, SessionState, SessionType, User};

#[proxy(
	interface = "org.freedesktop.login1.Session",
	default_service = "org.freedesktop.login1",
	default_path = "/org/freedesktop/login1"
)]
pub trait Session {
	/// Activate method
	#[inline]
	fn activate(&self) -> zbus::Result<()>;

	/// Kill method
	#[inline]
	fn kill(&self, who: &str, signal_number: i32) -> zbus::Result<()>;

	/// Lock method
	#[inline]
	fn lock(&self) -> zbus::Result<()>;

	/// PauseDeviceComplete method
	#[inline]
	fn pause_device_complete(&self, major: u32, minor: u32) -> zbus::Result<()>;

	/// ReleaseControl method
	#[inline]
	fn release_control(&self) -> zbus::Result<()>;

	/// ReleaseDevice method
	#[inline]
	fn release_device(&self, major: u32, minor: u32) -> zbus::Result<()>;

	/// SetBrightness method
	#[inline]
	fn set_brightness(&self, subsystem: &str, name: &str, brightness: u32) -> zbus::Result<()>;

	/// SetIdleHint method
	#[inline]
	fn set_idle_hint(&self, idle: bool) -> zbus::Result<()>;

	/// SetLockedHint method
	#[inline]
	fn set_locked_hint(&self, locked: bool) -> zbus::Result<()>;

	/// SetType method
	#[inline]
	fn set_type(&self, type_: &str) -> zbus::Result<()>;

	/// TakeControl method
	#[inline]
	fn take_control(&self, force: bool) -> zbus::Result<()>;

	/// TakeDevice method
	#[inline]
	fn take_device(&self, major: u32, minor: u32) -> zbus::Result<Device>;

	/// Terminate method
	#[inline]
	fn terminate(&self) -> zbus::Result<()>;

	/// Unlock method
	#[inline]
	fn unlock(&self) -> zbus::Result<()>;

	/// Lock signal
	#[zbus(signal)]
	#[inline]
	fn lock(&self) -> zbus::Result<()>;

	/// PauseDevice signal
	#[zbus(signal)]
	#[inline]
	fn pause_device(&self, major: u32, minor: u32, type_: &str) -> zbus::Result<()>;

	/// ResumeDevice signal
	#[zbus(signal)]
	#[inline]
	fn resume_device(&self, major: u32, minor: u32, fd: OwnedFd) -> zbus::Result<()>;

	/// Unlock signal
	#[zbus(signal)]
	#[inline]
	fn unlock(&self) -> zbus::Result<()>;

	/// Active property
	#[zbus(property)]
	#[inline]
	fn active(&self) -> zbus::Result<bool>;

	/// Audit property
	#[zbus(property)]
	#[inline]
	fn audit(&self) -> zbus::Result<u32>;

	/// Class property
	#[zbus(property)]
	#[inline]
	fn class(&self) -> zbus::Result<SessionClass>;

	/// Desktop property
	#[zbus(property)]
	#[inline]
	fn desktop(&self) -> zbus::Result<String>;

	/// Display property
	#[zbus(property)]
	#[inline]
	fn display(&self) -> zbus::Result<String>;

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

	/// Leader property
	#[zbus(property)]
	#[inline]
	fn leader(&self) -> zbus::Result<u32>;

	/// LockedHint property
	#[zbus(property)]
	#[inline]
	fn locked_hint(&self) -> zbus::Result<bool>;

	/// Name property
	#[zbus(property)]
	#[inline]
	fn name(&self) -> zbus::Result<String>;

	/// Remote property
	#[zbus(property)]
	#[inline]
	fn remote(&self) -> zbus::Result<bool>;

	/// RemoteHost property
	#[zbus(property)]
	#[inline]
	fn remote_host(&self) -> zbus::Result<String>;

	/// RemoteUser property
	#[zbus(property)]
	#[inline]
	fn remote_user(&self) -> zbus::Result<String>;

	/// Scope property
	#[zbus(property)]
	#[inline]
	fn scope(&self) -> zbus::Result<String>;

	/// Seat property
	#[zbus(property)]
	#[inline]
	fn seat(&self) -> zbus::Result<SomePath>;

	/// Service property
	#[zbus(property)]
	#[inline]
	fn service(&self) -> zbus::Result<String>;

	/// State property
	#[zbus(property)]
	#[inline]
	fn state(&self) -> zbus::Result<SessionState>;

	/// TTY property
	#[zbus(property)]
	#[inline]
	fn TTY(&self) -> zbus::Result<String>;

	/// Timestamp property
	#[zbus(property)]
	#[inline]
	fn timestamp(&self) -> zbus::Result<TimeStamp>;

	/// TimestampMonotonic property
	#[zbus(property)]
	#[inline]
	fn timestamp_monotonic(&self) -> zbus::Result<TimeStamp>;

	/// Type property
	#[zbus(property)]
	#[inline]
	fn type_(&self) -> zbus::Result<SessionType>;

	/// User property
	#[zbus(property)]
	#[inline]
	fn user(&self) -> zbus::Result<User>;

	/// VTNr property
	#[zbus(property)]
	#[inline]
	fn VTNr(&self) -> zbus::Result<u32>;
}
