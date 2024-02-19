// SPDX-License-Identifier: MPL-2.0

use derive_setters::Setters;
use std::{collections::HashMap, path::Path};
use zbus::{dbus_proxy, zvariant};
use zvariant::{SerializeDict, Type};

/// org.freedesktop.portal.FileChooser:
/// @short_description: File chooser portal
///
/// The FileChooser portal allows sandboxed applications to ask
/// the user for access to files outside the sandbox. The portal
/// backend will present the user with a file chooser dialog.
///
/// The selected files will be made accessible to the application
/// via the document portal, and the returned URI will point
/// into the document portal fuse filesystem in /run/user/$UID/doc/.
///
/// This documentation describes version 3 of this interface.
#[dbus_proxy(
	async_name = "Proxy",
	blocking_name = "BlockingProxy",
	interface = "org.freedesktop.portal.FileChooser",
	default_service = "org.freedesktop.portal.Desktop",
	default_path = "/org/freedesktop/portal/desktop"
)]
trait FileChooser {
	async fn open_file(
		&self,
		parent_window: &str,
		title: &str,
		options: OpenOptions<'_>,
	) -> zbus::Result<zvariant::OwnedObjectPath>;
}

#[derive(Debug, SerializeDict, Setters, Type)]
#[zvariant(signature = "a{sv}")]
pub struct OpenOptions<'a> {
	/// The label for the accept button. Mnemonic underlines are allowed.
	#[setters(strip_option)]
	accept_label: Option<&'a str>,

	/// Whether to make the dialog modal. Default is yes.
	modal: bool,

	/// Whether to allow selection of multiple files. Default is no.
	multiple: bool,

	/// Whether to select for folders instead of files. Default is to select files.
	directory: bool,

	// TODO: filters a(sa(us)): A list of serialized file filters.
	// TODO: current_filter (sa(us)): Request that this filter be set by default at dialog creation.
	// TODO: choices a(ssa(ss)s): A list of serialized combo boxes.
	/// A suggested folder to open the files from.
	#[setters(strip_option)]
	current_folder: Option<&'a Path>,
}

impl<'a> Default for OpenOptions<'a> {
	fn default() -> Self {
		Self {
			accept_label: None,
			modal: true,
			multiple: false,
			directory: false,
			current_folder: None,
		}
	}
}

#[derive(Debug, SerializeDict, Setters, Type)]
#[zvariant(signature = "a{sv}")]
pub struct SaveOptions<'a> {
	/// The label for the accept button. Mnemonic underlines are allowed.
	#[setters(strip_option)]
	accept_label: Option<&'a str>,

	/// Whether to make the dialog modal. Default is yes.
	modal: bool,

	/// Whether to allow selection of multiple files. Default is no.
	multiple: bool,

	// TODO: filters a(sa(us)): A list of serialized file filters.
	// TODO: current_filter (sa(us)): Request that this filter be set by default at dialog creation.
	// TODO: choices a(ssa(ss)s): A list of serialized combo boxes.
	/// A suggested filename.
	#[setters(strip_option)]
	current_name: Option<&'a str>,

	// A suggested folder to save the file in.
	#[setters(strip_option)]
	current_folder: Option<&'a Path>,

	/// The current file (when saving an existing file)
	#[setters(strip_option)]
	current_file: Option<&'a Path>,
}

impl<'a> Default for SaveOptions<'a> {
	fn default() -> Self {
		Self {
			accept_label: None,
			modal: true,
			multiple: false,
			current_name: None,
			current_folder: None,
			current_file: None,
		}
	}
}

#[derive(Debug)]
pub struct Response<'a> {
	pub uris: Vec<&'a str>,
	pub choices: Vec<(&'a str, &'a str)>,
	pub writable: bool,
}

impl<'a> TryFrom<&'a HashMap<&'a str, zvariant::Value<'a>>> for Response<'a> {
	type Error = String;

	fn try_from(result: &'a HashMap<&'a str, zvariant::Value<'a>>) -> Result<Self, Self::Error> {
		let mut response = Self {
			uris: Vec::new(),
			choices: Vec::new(),
			writable: false,
		};

		for (name, value) in result {
			match *name {
				"choices" => {
					let zvariant::Value::Array(array) = value else {
                        return Err("choices property is not an array".into());
                    };

					for value in array.into_iter() {
						if let zvariant::Value::Structure(choice) = value {
							let fields = choice.fields();

							let Some(zvariant::Value::Str(second)) = fields.get(1) else {
                                continue;
                            };

							let zvariant::Value::Str(first) = &fields[0] else {
                                continue;
                            };

							response.choices.push((first.as_str(), second.as_str()));
						}
					}
				}

				"uris" => {
					let zvariant::Value::Array(array) = value else {
                        return Err("uris propert is not an array".into());
                    };

					for value in array.into_iter() {
						if let zvariant::Value::Str(string) = value {
							response.uris.push(string.as_str());
						}
					}
				}

				"writable" => {
					if let zvariant::Value::Bool(writable) = value {
						response.writable = *writable;
					}
				}

				_ => (),
			}
		}

		Ok(response)
	}
}
