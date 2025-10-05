use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::{
	Result, proxy,
	zvariant::{ObjectPath, OwnedValue, Type},
};

#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq, Eq)]
pub enum Accuracy {
	/// Accuracy level unknown or unset
	None,
	/// Country level accuracy
	Country,
	/// City level accuracy
	City,
	/// Neighborhood level accuracy
	Neighborhood,
	/// Street level accuracy
	Street,
	/// Exact accuracy. Typically requires GPS receiver
	Exact,
}

impl From<Accuracy> for u32 {
	fn from(value: Accuracy) -> Self {
		value as u32
	}
}

impl TryFrom<OwnedValue> for Accuracy {
	type Error = <u32 as TryFrom<OwnedValue>>::Error;

	fn try_from(value: OwnedValue) -> std::prelude::v1::Result<Self, Self::Error> {
		Ok(match <u32>::try_from(value)? {
			1 => Accuracy::Country,
			2 => Accuracy::City,
			3 => Accuracy::Neighborhood,
			4 => Accuracy::Street,
			5 => Accuracy::Exact,
			_ => Accuracy::None,
		})
	}
}

#[proxy(
	default_service = "org.freedesktop.GeoClue2",
	interface = "org.freedesktop.GeoClue2.Manager",
	default_path = "/org/freedesktop/GeoClue2/Manager"
)]
pub trait Manager {
	/// Retrieves a client object which can only be used by the calling application only. On the first call from a specific D-Bus peer, this method will create the client object but subsequent calls will return the path of the existing client.
	#[zbus(object = "Client")]
	fn get_client(&self);

	/// Use this method to explicitly destroy a client, created using GetClient() or CreateClient().
	///
	/// Long-running applications, should either use this to delete associated client(s) when not needed, or disconnect from the D-Bus connection used for communicating with Geoclue (which is implicit on client process termination).
	#[zbus(object = "Client")]
	fn delete_client<'a>(&self, client: ObjectPath<'a>);

	/// InUse property
	#[zbus(property)]
	fn in_use(&self) -> Result<bool>;

	/// AvailableAccuracyLevel property
	#[zbus(property)]
	fn available_accuracy_level(&self) -> zbus::Result<Accuracy>;
}

#[proxy(
	default_service = "org.freedesktop.GeoClue2",
	interface = "org.freedesktop.GeoClue2.Client"
)]
pub trait Client {
	/// Start method
	fn start(&self) -> zbus::Result<()>;

	/// Stop method
	fn stop(&self) -> zbus::Result<()>;

	/// LocationUpdated signal
	#[zbus(signal)]
	fn location_updated(
		&self,
		old: zbus::zvariant::ObjectPath<'_>,
		new: zbus::zvariant::ObjectPath<'_>,
	) -> zbus::Result<()>;

	/// Active property
	#[zbus(property)]
	fn active(&self) -> zbus::Result<bool>;

	/// DesktopId property
	#[zbus(property)]
	fn desktop_id(&self) -> zbus::Result<String>;
	#[zbus(property)]
	fn set_desktop_id(&self, value: &str) -> zbus::Result<()>;

	/// DistanceThreshold property
	#[zbus(property)]
	fn distance_threshold(&self) -> zbus::Result<u32>;
	#[zbus(property)]
	fn set_distance_threshold(&self, value: u32) -> zbus::Result<()>;

	/// Location property
	#[zbus(property, object = "Location")]
	fn location(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

	/// RequestedAccuracyLevel property
	#[zbus(property)]
	fn requested_accuracy_level(&self) -> zbus::Result<Accuracy>;
	#[zbus(property)]
	fn set_requested_accuracy_level(&self, value: u32) -> zbus::Result<()>;

	/// TimeThreshold property
	#[zbus(property)]
	fn time_threshold(&self) -> zbus::Result<u32>;
	fn set_time_threshold(&self, value: u32) -> zbus::Result<()>;
}

#[proxy(
	default_service = "org.freedesktop.GeoClue2",
	interface = "org.freedesktop.GeoClue2.Location"
)]
pub trait Location {
	/// The latitude of the location, in degrees.
	#[zbus(property)]
	fn latitude(&self) -> Result<f64>;
	/// The longitude of the location, in degrees.
	#[zbus(property)]
	fn longitude(&self) -> Result<f64>;
	/// The accuracy of the location fix, in meters.
	#[zbus(property)]
	fn accuracy(&self) -> Result<f64>;
	/// The altitude of the location fix, in meters.
	/// When unknown, its set to minimum f64 value, -1.7976931348623157e+308.
	#[zbus(property)]
	fn altitude(&self) -> Result<f64>;
	/// Speed in meters per second.
	/// When unknown, it's set to -1.0.
	#[zbus(property)]
	fn speed(&self) -> Result<f64>;
	/// The heading direction in degrees with respect to North direction, in clockwise order. That means North becomes 0 degree, East: 90 degrees, South: 180 degrees, West: 270 degrees and so on. When unknown, it's set to -1.0.
	#[zbus(property)]
	fn heading(&self) -> Result<f64>;
}
