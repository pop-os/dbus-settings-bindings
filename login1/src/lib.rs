//! Reference <https://www.freedesktop.org/software/systemd/man/org.freedesktop.login1.html>

use std::{
	ops::{Deref, DerefMut},
	time::Duration,
};

use serde::{Deserialize, Serialize};
use zbus::{OwnedObjectPath, OwnedVariant, Signature, Structure, Type};
pub mod manager;
pub mod seat;
pub mod session;
pub mod user;

pub struct TimeStamp(Duration);

impl Type for TimeStamp {
	const SIGNATURE: &'static Signature = <u64 as Type>::SIGNATURE;
}

impl Serialize for TimeStamp {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_u64(self.0.as_micros() as u64)
	}
}

impl<'de> Deserialize<'de> for TimeStamp {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Ok(Self(Duration::from_micros(u64::deserialize(deserializer)?)))
	}
}

impl Deref for TimeStamp {
	type Target = Duration;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for TimeStamp {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl TryFrom<OwnedVariant> for TimeStamp {
	type Error = zbus::Error;

	fn try_from(value: OwnedVariant) -> Result<Self, Self::Error> {
		let value = <u64>::try_from(value)?;
		Ok(Self(Duration::from_micros(value)))
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub struct SomePath {
	/// The seat label
	id: String,
	/// DBUS path for this seat
	path: OwnedObjectPath,
}

impl SomePath {
	pub fn id(&self) -> &str {
		&self.id
	}

	pub fn path(&self) -> &OwnedObjectPath {
		&self.path
	}
}

impl TryFrom<OwnedVariant> for SomePath {
	type Error = zbus::Error;

	fn try_from(value: OwnedVariant) -> Result<Self, Self::Error> {
		let value = <Structure<'_>>::try_from(value)?;
		Ok(Self {
			id: <String>::try_from(value.fields()[0].try_clone()?)?,
			path: <OwnedObjectPath>::try_from(value.fields()[1].try_clone()?)?,
		})
	}
}

#[macro_export]
macro_rules! enum_impl_serde_str {
	($type_name:ident) => {
		impl Serialize for $type_name {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where
				S: serde::Serializer,
			{
				serializer.serialize_str(self.into())
			}
		}

		impl<'de> Deserialize<'de> for $type_name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				let s = String::deserialize(deserializer)?;
				$type_name::from_str(s.as_str()).map_err(serde::de::Error::custom)
			}
		}
	};
}

#[macro_export]
macro_rules! impl_try_from_owned_as_str {
	($type_name:ident) => {
		impl TryFrom<OwnedVariant> for $type_name {
			type Error = zbus::Error;

			fn try_from(value: OwnedVariant) -> Result<Self, Self::Error> {
				let value = <String>::try_from(value)?;
				return Ok($type_name::from_str(value.as_str())?);
			}
		}
	};
}

#[macro_export]
macro_rules! enum_impl_str_conv {
    ($type_name:ident, { $($label:tt : $variant:tt,)* }) => {
        impl FromStr for $type_name {
            type Err = fdo::Error;

            fn from_str(m: &str) -> Result<Self, Self::Err> {
                let res = match m {
                    $($label => $type_name::$variant,)+
                    _ => return Err(fdo::Error::IOError(format!("{} is an invalid variant", m))),
                };
                Ok(res)
            }
        }

        impl From<$type_name> for &str {
            fn from(m: $type_name) -> Self {
                match m {
                    $($type_name::$variant => $label,)+
                }
            }
        }

        impl From<&$type_name> for &str {
            fn from(s: &$type_name) -> Self {
                <&str>::from(*s)
            }
        }
}}

#[cfg(test)]
mod tests {
	use crate::{manager::ManagerProxy, session::SessionProxy};
	use futures_lite::future;

	#[test]
	fn basic_test() {
		future::block_on(async {
			let connection = zbus::Connection::system().await.unwrap();
			let manager = ManagerProxy::new(&connection).await.unwrap();
			let sessions = manager.list_sessions().await.unwrap();
			let session_proxy = SessionProxy::builder(&connection)
				.path(sessions[0].path())
				.build()
				.await
				.unwrap();

			assert!(session_proxy.seat().await.is_ok());
		});
	}
}
