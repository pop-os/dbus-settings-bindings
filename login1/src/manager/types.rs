use serde::{Deserialize, Serialize};
use std::str::FromStr;
use zbus::fdo;
use zbus::{OwnedObjectPath, OwnedVariant, Structure, Type};

use crate::{enum_impl_serde_str, enum_impl_str_conv};

/// Basic user information
#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub struct UserInfo {
	/// User ID
	uid: u32,
	/// User name
	name: String,
	/// DBUS path to this user
	path: OwnedObjectPath,
}

impl UserInfo {
	pub fn uid(&self) -> u32 {
		self.uid
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn path(&self) -> &OwnedObjectPath {
		&self.path
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub struct ScheduledShutdown {
	/// Name of the shutdown
	id: String,
	/// Time in micros
	time: u64,
}

impl ScheduledShutdown {
	pub fn id(&self) -> &str {
		&self.id
	}

	pub fn time(&self) -> u64 {
		self.time
	}
}

impl TryFrom<OwnedVariant> for ScheduledShutdown {
	type Error = zbus::Error;

	fn try_from(value: OwnedVariant) -> Result<Self, Self::Error> {
		let value = <Structure<'_>>::try_from(value)?;
		Ok(Self {
			id: <String>::try_from(value.fields()[0].try_clone()?)?,
			time: <u64>::try_from(value.fields()[1].try_clone()?)?,
		})
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum IsSupported {
	NA,
	Yes,
	No,
	Challenge,
}
enum_impl_serde_str!(IsSupported);
enum_impl_str_conv!(IsSupported, {
	"na": NA,
	"yes": Yes,
	"no": No,
	"challenge": Challenge,
});

#[derive(Debug, PartialEq, Eq, Clone, Type)]
#[zvariant(signature = "s")]
pub struct InhibitTypes(Vec<InhibitType>);

impl InhibitTypes {
	pub fn new(inhibit_types: &[InhibitType]) -> InhibitTypes {
		Self(inhibit_types.to_vec())
	}

	pub fn types(&self) -> &Vec<InhibitType> {
		&self.0
	}
}

impl FromStr for InhibitTypes {
	type Err = fdo::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut buf = Vec::new();
		for chunk in s.split(':') {
			buf.push(InhibitType::from_str(chunk)?);
		}
		Ok(Self(buf))
	}
}

impl From<InhibitTypes> for String {
	fn from(s: InhibitTypes) -> Self {
		let mut string = String::new();
		for (i, inhibit) in s.0.iter().enumerate() {
			if i > 0 && i < s.0.len() {
				string.push(':');
			}
			string.push_str((*inhibit).into());
		}
		string
	}
}

impl From<&InhibitTypes> for String {
	fn from(s: &InhibitTypes) -> Self {
		let mut string = String::new();
		for (i, inhibit) in s.0.iter().enumerate() {
			if i > 0 && i < s.0.len() {
				string.push(':');
			}
			string.push_str((*inhibit).into());
		}
		string
	}
}

impl Serialize for InhibitTypes {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(String::from(self).as_str())
	}
}

impl<'de> Deserialize<'de> for InhibitTypes {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		InhibitTypes::from_str(s.as_str()).map_err(serde::de::Error::custom)
	}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Type)]
#[zvariant(signature = "s")]
pub enum InhibitType {
	Shutdown,
	Sleep,
	Idle,
	HandlePowerKey,
	HandleSuspendKey,
	HandleHibernateKey,
	HandleLidSwitch,
}
enum_impl_serde_str!(InhibitType);
enum_impl_str_conv!(InhibitType, {
	"shutdown": Shutdown,
	"sleep": Sleep,
	"idle": Idle,
	"handle-power-key": HandlePowerKey,
	"handle-suspend-key": HandleSuspendKey,
	"handle-hibernate-key": HandleHibernateKey,
	"handle-lid-switch": HandleLidSwitch,
});

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub struct Inhibitor {
	/// What this lock is inhibiting
	what: InhibitTypes,
	/// The name or ID of what is inhibiting, for example the applicaiton name creating this lock
	who: String,
	/// A description of why the lock was created
	why: String,
	/// The lock behaviour
	mode: Mode,
	user_id: u32,
	process_id: u32,
}

impl Inhibitor {
	pub fn new(
		what: InhibitTypes,
		who: String,
		why: String,
		mode: Mode,
		user_id: u32,
		process_id: u32,
	) -> Inhibitor {
		Inhibitor {
			what,
			who,
			why,
			mode,
			user_id,
			process_id,
		}
	}

	pub fn what(&self) -> &InhibitTypes {
		&self.what
	}

	pub fn who(&self) -> &str {
		&self.who
	}

	pub fn why(&self) -> &str {
		&self.why
	}

	pub fn mode(&self) -> Mode {
		self.mode
	}

	pub fn user_id(&self) -> u32 {
		self.user_id
	}

	pub fn process_id(&self) -> u32 {
		self.process_id
	}
}

/// Used to determine behaviour of inhibitors
#[derive(Debug, PartialEq, Eq, Copy, Clone, Type)]
#[zvariant(signature = "s")]
pub enum Mode {
	/// Inhibitor is mandatory
	Block,
	/// Inhibitor delays to a certain time
	Delay,
}
enum_impl_serde_str!(Mode);
enum_impl_str_conv!(Mode, {
	"block": Block,
	"delay": Delay,
});

#[derive(Debug, PartialEq, Eq, Type, Serialize, Deserialize)]
pub struct SessionInfo {
	/// Session ID
	sid: String,
	/// User ID
	uid: u32,
	/// Name of session user
	user: String,
	/// The session seat label
	seat: String,
	/// DBUS path for this session
	path: OwnedObjectPath,
}

impl SessionInfo {
	pub fn sid(&self) -> &str {
		&self.sid
	}

	pub fn uid(&self) -> u32 {
		self.uid
	}

	pub fn user(&self) -> &str {
		&self.user
	}

	pub fn seat(&self) -> &str {
		&self.seat
	}

	pub fn path(&self) -> &OwnedObjectPath {
		&self.path
	}
}
