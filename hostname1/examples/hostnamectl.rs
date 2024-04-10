use std::process::ExitCode;

#[tokio::main]
async fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
	let connection = zbus::Connection::system().await?;
	let proxy = hostname1_zbus::Hostname1Proxy::new(&connection).await?;

	let mut parser = pico_args::Arguments::from_env();

	match parser.subcommand()?.as_deref() {
		Some("hostname") => match parser.subcommand()?.as_deref() {
			Some("set") => match parser.free_from_str::<String>().ok().as_deref() {
				Some(new_hostname) => {
					if let Err(why) = proxy.set_hostname(&new_hostname, false).await {
						eprintln!("error: could not set hostname: {why}");
						return Ok(ExitCode::FAILURE);
					}
				}

				None => {
					eprintln!("error: hostname argument not set");
					return Ok(ExitCode::FAILURE);
				}
			},

			_ => {
				println!("{}", proxy.hostname().await?);
			}
		},

		Some("static-hostname") => match parser.subcommand()?.as_deref() {
			Some("set") => match parser.free_from_str::<String>().ok().as_deref() {
				Some(new_hostname) => {
					if let Err(why) = proxy.set_static_hostname(&new_hostname, false).await {
						eprintln!("error: could not set hostname: {why}");
						return Ok(ExitCode::FAILURE);
					}
				}

				None => {
					eprintln!("error: hostname argument not set");
					return Ok(ExitCode::FAILURE);
				}
			},

			_ => {
				println!("{}", proxy.static_hostname().await?);
			}
		},

		_ => print_help(),
	}

	Ok(ExitCode::SUCCESS)
}

fn print_help() {
	println!(
		"\
hostnamectl

USAGE:
    hostnamectl hostname
    hostnamectl hostname set NAME
"
	);
}
