use crate::{enum_impl_serde_str, enum_impl_str_conv, impl_try_from_owned_as_str};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use zbus::fdo;
use zbus::{OwnedVariant, Type};

/// State of a User
#[derive(Debug, PartialEq, Eq, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum UserState {
	Online,
	Offline,
	Lingering,
	Active,
	Closing,
}
enum_impl_serde_str!(UserState);
impl_try_from_owned_as_str!(UserState);
enum_impl_str_conv!(UserState, {
	"online": Online,
	"offline": Offline,
	"lingering": Lingering,
	"active": Active,
	"closing": Closing,
});
