use futures_util::StreamExt;
use tracing_subscriber::prelude::*;
use zbus::zvariant::ObjectPath;

const AGENT_PATH: &str = "/org/bluez/agent/cosmic";

#[tokio::main]
async fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let log_level = std::env::var("RUST_LOG")
		.ok()
		.and_then(|level| level.parse::<tracing::Level>().ok())
		.unwrap_or(tracing::Level::DEBUG);

	let log_format = tracing_subscriber::fmt::format()
		.pretty()
		.without_time()
		.with_line_number(true)
		.with_file(true)
		.with_target(false)
		.with_thread_names(true);

	let log_filter = tracing_subscriber::fmt::Layer::default()
		.with_writer(std::io::stderr)
		.event_format(log_format)
		.with_filter(tracing_subscriber::filter::filter_fn(move |metadata| {
			metadata.level() <= &log_level
		}));

	tracing_subscriber::registry().with(log_filter).init();

	let system_conn = zbus::Connection::system().await?;

	let (agent, mut receiver) = bluez_zbus::agent1::create();

	let agent_path = ObjectPath::from_static_str_unchecked(AGENT_PATH);

	tracing::debug!("connecting agent");

	system_conn.object_server().at(&agent_path, agent).await?;

	tracing::debug!("connecting to bluez agent manager");

	let bluez = bluez_zbus::agent_manager1::AgentManager1Proxy::new(&system_conn).await?;

	tracing::debug!("registering agent");

	bluez
		.register_agent(
			&agent_path,
			<&'static str>::from(bluez_zbus::agent1::Capability::DisplayYesNo),
		)
		.await?;

	if let Err(why) = bluez.request_default_agent(&agent_path).await {
		_ = bluez.unregister_agent(&agent_path).await;
		Err(why)?;
	}

	tracing::debug!("registered");

	while let Some(msg) = receiver.next().await {
		tracing::debug!(?msg, "message received");

		match msg {
			bluez_zbus::agent1::Message::RequestAuthorization { device, response } => {
				_ = response.send(true);
			}

			bluez_zbus::agent1::Message::RequestConfirmation {
				device,
				passkey,
				response,
			} => {
				_ = response.send(true);
			}

			bluez_zbus::agent1::Message::RequestPasskey { device, response } => {
				_ = response.send(None);
			}

			bluez_zbus::agent1::Message::RequestPinCode { device, response } => {
				_ = response.send(None);
			}

			_ => (),
		}
	}

	_ = bluez.unregister_agent(&agent_path).await;

	tracing::debug!("exiting");

	Ok(())
}
