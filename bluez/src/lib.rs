use futures::{join, StreamExt};

pub mod adapter1;
pub mod battery1;
pub mod device1;

pub async fn get_adapters<'a>(
	connection: &zbus::Connection,
) -> zbus::Result<Vec<adapter1::Adapter1Proxy<'a>>> {
	let managed_object_proxy =
		zbus::fdo::ObjectManagerProxy::new(connection, "org.bluez", "/").await?;
	let managed_object: zbus::fdo::ManagedObjects =
		managed_object_proxy.get_managed_objects().await?;
	let adapter_addresses: Vec<zbus::zvariant::OwnedObjectPath> = managed_object
		.into_iter()
		.filter_map(move |(path, interfaces)| {
			interfaces
				.contains_key("org.bluez.Adapter1")
				.then_some(path.to_owned())
		})
		.collect();
	let adapters: Vec<zbus::Result<adapter1::Adapter1Proxy<'a>>> = futures::future::join_all(
		adapter_addresses
			.into_iter()
			.map(|path| adapter1::Adapter1Proxy::new(connection, path)),
	)
	.await;

	let errors = adapters.iter().filter(|device| device.is_err());
	if errors.count() > 0 {
		let mut errors: Vec<zbus::Error> = adapters
			.into_iter()
			.filter_map(std::result::Result::err)
			.collect();
		if errors.len() > 1 {
			eprintln!("Multiple errors occurs when fetching connected device: {errors:?}. Only the last one will be returned.");
		}
		return Err(errors.pop().unwrap());
	}
	Ok(adapters
		.into_iter()
		.filter_map(std::result::Result::ok)
		.collect())
}

#[derive(Debug)]
pub struct BluetoothDevice<'a> {
	pub device: device1::Device1Proxy<'a>,
	pub battery: Option<battery1::Battery1Proxy<'a>>,
}

impl<'a> BluetoothDevice<'a> {
	pub async fn new<'b: 'a>(
		connection: &zbus::Connection,
		path: zbus::zvariant::ObjectPath<'b>,
	) -> zbus::Result<Self> {
		let (device, battery) = join!(
			device1::Device1Proxy::builder(connection)
				.path(&path)?
				.build(),
			battery1::Battery1Proxy::builder(connection)
				.path(path)?
				.build()
		);

		match (device, battery) {
			(Ok(device), Ok(battery)) if battery.percentage().await.is_err() => Ok(Self {
				device,
				battery: None,
			}),
			(Ok(device), Ok(battery)) => Ok(Self {
				device,
				battery: Some(battery),
			}),
			(Ok(device), Err(zbus::Error::InterfaceNotFound)) => Ok(Self {
				device,
				battery: None,
			}),
			(Err(why), _) => Err(why),
			(_, Err(why)) => Err(why),
		}
	}

	pub async fn icon(&self) -> String {
		self.device
			.inner()
			.get_property::<String>("Icon")
			.await
			.unwrap_or("unknown".to_owned())
	}

	pub fn path(&self) -> zbus::zvariant::OwnedObjectPath {
		self.device.inner().path().to_owned().into()
	}
}

pub async fn get_device<'a>(
	connection: &zbus::Connection,
	adapter_iface: impl Into<String>,
	device_addr: impl Into<String>,
) -> zbus::Result<BluetoothDevice<'a>> {
	BluetoothDevice::new(
		connection,
		zbus::zvariant::ObjectPath::try_from(format!(
			"/org/bluez/{}/dev_{}",
			adapter_iface.into(),
			device_addr.into().replace(":", "_")
		))?,
	)
	.await
}

pub async fn get_adapter<'a>(
	connection: &zbus::Connection,
	adapter_iface: impl Into<String>,
) -> zbus::Result<adapter1::Adapter1Proxy<'a>> {
	adapter1::Adapter1Proxy::builder(connection)
		.path(zbus::zvariant::ObjectPath::try_from(format!(
			"/org/bluez/{}",
			adapter_iface.into()
		))?)?
		.build()
		.await
}

pub async fn get_devices<'a>(
	connection: &zbus::Connection,
	adapter: Option<&str>,
) -> zbus::Result<Vec<BluetoothDevice<'a>>> {
	let managed_object_proxy =
		zbus::fdo::ObjectManagerProxy::new(connection, "org.bluez", "/").await?;
	let managed_object: zbus::fdo::ManagedObjects =
		managed_object_proxy.get_managed_objects().await?;
	let device_addresses: Vec<zbus::zvariant::OwnedObjectPath> = managed_object
		.into_iter()
		.filter_map(move |(path, interfaces)| {
			if matches!(
				adapter.map(|adapter| path.as_str().starts_with(&format!("{}/", adapter))),
				None | Some(true)
			) {
				return interfaces
					.contains_key("org.bluez.Device1")
					.then_some(path.to_owned());
			}
			None
		})
		.collect();
	let devices: Vec<zbus::Result<BluetoothDevice<'a>>> = futures::future::join_all(
		device_addresses
			.into_iter()
			.map(|path| BluetoothDevice::new(connection, path.into())),
	)
	.await;

	let errors = devices.iter().filter(|device| device.is_err());
	if errors.count() > 0 {
		let mut errors: Vec<zbus::Error> = devices
			.into_iter()
			.filter_map(std::result::Result::err)
			.collect();
		if errors.len() > 1 {
			eprintln!("Multiple errors occurs when fetching connected device: {errors:?}. Only the last one will be returned.");
		}
		return Err(errors.pop().unwrap());
	}
	Ok(devices
		.into_iter()
		.filter_map(std::result::Result::ok)
		.collect())
}