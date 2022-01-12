// SPDX-License-Identifier: MPL-2.0

use crate::interface::settings::connection::ConnectionSettingsProxy;
use std::ops::Deref;

pub struct Connection<'a>(ConnectionSettingsProxy<'a>);

impl<'a> Deref for Connection<'a> {
	type Target = ConnectionSettingsProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<ConnectionSettingsProxy<'a>> for Connection<'a> {
	fn from(conn: ConnectionSettingsProxy<'a>) -> Self {
		Connection(conn)
	}
}
