// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Integrations for creating bluez agents.

use futures_channel::{mpsc, oneshot};
use futures_util::SinkExt;
use zbus::zvariant::OwnedObjectPath;

pub fn create() -> (Agent, mpsc::Receiver<Message>) {
	let (message_sender, message_receiver) = futures_channel::mpsc::channel(1);

	(Agent { message_sender }, message_receiver)
}

#[derive(Clone, Copy, Debug)]
pub enum Capability {
	DisplayOnly = 0x00,
	DisplayYesNo = 0x01,
	KeyboardOnly = 0x02,
	NoInputNoOutput = 0x03,
	KeyboardDisplay = 0x04,
}

impl From<Capability> for &'static str {
	fn from(capability: Capability) -> &'static str {
		match capability {
			Capability::DisplayOnly => "DisplayOnly",
			Capability::DisplayYesNo => "DisplayYesNo",
			Capability::KeyboardOnly => "KeyboardOnly",
			Capability::NoInputNoOutput => "NoInputNoOutput",
			Capability::KeyboardDisplay => "KeyboardDisplay",
		}
	}
}

#[derive(Debug)]
pub enum Message {
	AuthorizeService {
		device: OwnedObjectPath,
		uuid: String,
	},
	Cancel,
	DisplayPasskey {
		device: OwnedObjectPath,
		passkey: u32,
		entered: u16,
	},
	DisplayPinCode {
		device: OwnedObjectPath,
		pincode: String,
	},
	Release,
	RequestAuthorization {
		device: OwnedObjectPath,
		response: oneshot::Sender<bool>,
	},
	RequestConfirmation {
		device: OwnedObjectPath,
		passkey: u32,
		response: oneshot::Sender<bool>,
	},
	RequestPasskey {
		device: OwnedObjectPath,
		response: oneshot::Sender<Option<u32>>,
	},
	RequestPinCode {
		device: OwnedObjectPath,
		response: oneshot::Sender<Option<String>>,
	},
}

pub struct Agent {
	pub(self) message_sender: mpsc::Sender<Message>,
}

#[zbus::interface(name = "org.bluez.Agent1")]
impl Agent {
	/// This method gets called when the service daemon
	/// needs to authorize a connection/service request.
	async fn authorize_service(
		&mut self,
		device: OwnedObjectPath,
		uuid: String,
	) -> zbus::fdo::Result<()> {
		tracing::debug!(?device, uuid, "authorize_service");

		Ok(())
	}

	/// This method gets called to indicate that the agent request
	/// failed before a reply was returned.
	async fn cancel(&mut self) -> zbus::fdo::Result<()> {
		tracing::debug!("cancel");

		_ = self.message_sender.send(Message::Cancel).await;

		Ok(())
	}

	/// This method gets called when the service daemon
	/// needs to display a passkey for an authentication.
	///
	/// The entered parameter indicates the number of already
	/// typed keys on the remote side.
	///
	/// An empty reply should be returned. When the passkey
	/// needs no longer to be displayed, the Cancel method
	/// of the agent will be called.
	///
	/// During the pairing process this method might be
	/// called multiple times to update the entered value.
	///
	/// Note that the passkey will always be a 6-digit number,
	/// so the display should be zero-padded at the start if
	/// the value contains less than 6 digits.
	async fn display_passkey(
		&mut self,
		device: OwnedObjectPath,
		passkey: u32,
		entered: u16,
	) -> zbus::fdo::Result<()> {
		tracing::debug!(?device, passkey, entered, "display_passkey");

		Ok(())
	}

	/// This method gets called when the service daemon
	/// needs to display a pin code for an authentication.
	///
	/// An empty reply should be returned. When the pin code
	/// needs no longer to be displayed, the Cancel method
	/// of the agent will be called.
	///
	/// This is used during the pairing process of keyboards
	/// that don't support Bluetooth 2.1 Secure Simple Pairing,
	/// in contrast to DisplayPasskey which is used for those
	/// that do.
	///
	/// This method will only ever be called once since
	/// older keyboards do not support typing notification.
	///
	/// Note that the PIN will always be a 6-digit number,
	/// zero-padded to 6 digits. This is for harmony with
	/// the later specification.
	async fn display_pin_code(
		&mut self,
		device: OwnedObjectPath,
		pin_code: String,
	) -> zbus::fdo::Result<()> {
		tracing::debug!(?device, pin_code, "display_pin_code");

		Ok(())
	}

	async fn release(&mut self) -> zbus::fdo::Result<()> {
		tracing::debug!("release");

		_ = self.message_sender.send(Message::Release).await;

		Ok(())
	}

	/// This method gets called to request the user to
	/// authorize an incoming pairing attempt which
	/// would in other circumstances trigger the just-works
	/// model, or when the user plugged in a device that
	/// implements cable pairing.
	///
	/// In the latter case, the
	/// device would not be connected to the adapter via
	/// Bluetooth yet.
	async fn request_authorization(&mut self, device: OwnedObjectPath) -> zbus::fdo::Result<()> {
		tracing::debug!(?device, "request_authorization");

		Ok(())
	}

	/// This method gets called when the service daemon
	/// needs to confirm a passkey for an authentication.
	///
	/// To confirm the value it should return an empty reply
	/// or an error in case the passkey is invalid.
	///
	/// Note that the passkey will always be a 6-digit number,
	/// so the display should be zero-padded at the start if
	/// the value contains less than 6 digits.
	async fn request_confirmation(
		&mut self,
		device: OwnedObjectPath,
		passkey: u32,
	) -> zbus::fdo::Result<()> {
		tracing::debug!(?device, passkey, "request_confirmation");

		let (response, response_rx) = oneshot::channel::<bool>();

		_ = self
			.message_sender
			.send(Message::RequestConfirmation {
				device,
				passkey,
				response,
			})
			.await;

		match response_rx.await {
			Ok(true) => Ok(()),
			Ok(false) => Err(zbus::fdo::Error::Failed("cancelled".to_string())),
			Err(why) => Err(zbus::fdo::Error::Failed(why.to_string())),
		}
	}

	/// This method gets called when the service daemon
	/// needs to get the passkey for an authentication.
	///
	/// The return value should be a numeric value
	/// between 0-999999.
	async fn request_passkey(&mut self, device: OwnedObjectPath) -> zbus::fdo::Result<u32> {
		tracing::debug!(?device, "request_passkey");

		let (response, response_rx) = oneshot::channel::<Option<u32>>();

		_ = self
			.message_sender
			.send(Message::RequestPasskey { device, response })
			.await;

		match response_rx.await {
			Ok(Some(passkey)) => Ok(passkey),
			Ok(None) => Err(zbus::fdo::Error::Failed("cancelled".to_string())),
			Err(why) => Err(zbus::fdo::Error::Failed(why.to_string())),
		}
	}

	/// This method gets called when the service daemon
	/// needs to get the passkey for an authentication.
	///
	/// The return value should be a string of 1-16 characters
	/// length. The string can be alphanumeric.
	async fn request_pin_code(&mut self, device: OwnedObjectPath) -> zbus::fdo::Result<String> {
		tracing::debug!(?device, "request_pin_code");

		let (response, response_rx) = oneshot::channel::<Option<String>>();

		_ = self
			.message_sender
			.send(Message::RequestPinCode { device, response })
			.await;

		match response_rx.await {
			Ok(Some(pin_code)) => Ok(pin_code),
			Ok(None) => Err(zbus::fdo::Error::Failed("cancelled".to_string())),
			Err(why) => Err(zbus::fdo::Error::Failed(why.to_string())),
		}
	}
}
