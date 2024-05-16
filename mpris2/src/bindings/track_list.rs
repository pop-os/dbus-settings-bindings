// SPDX-License-Identifier: MPL-2.0
//! # DBus interface proxies for: `org.mpris.MediaPlayer2`, `org.mpris.MediaPlayer2.Player`, `org.mpris.MediaPlayer2.TrackList`, `org.mpris.MediaPlayer2.Playlists`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Interface '/org/mpris/MediaPlayer2' from service 'org.mpris.MediaPlayer2.org.gnome.Music' on session bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use crate::track::TrackId;
use zbus::proxy;

#[proxy(
	interface = "org.mpris.MediaPlayer2.TrackList",
	default_path = "/org/mpris/MediaPlayer2"
)]
trait TrackList {
	/// AddTrack method
	fn add_track(&self, uri: &str, after_track: &TrackId, set_as_current: bool)
		-> zbus::Result<()>;

	/// GetTracksMetadata method
	fn get_tracks_metadata(
		&self,
		track_ids: Vec<TrackId>,
	) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

	/// GoTo method
	fn go_to(&self, track_id: &TrackId) -> zbus::Result<()>;

	/// RemoveTrack method
	fn remove_track(&self, track_id: &TrackId) -> zbus::Result<()>;

	/// TrackListReplaced signal
	#[zbus(signal)]
	fn track_list_replaced(&self, tracks: Vec<TrackId>, current_track: TrackId)
		-> zbus::Result<()>;

	/// CanEditTracks property
	#[zbus(property)]
	fn can_edit_tracks(&self) -> zbus::Result<bool>;

	/// Tracks property
	#[zbus(property)]
	fn tracks(&self) -> zbus::Result<Vec<TrackId>>;
}
