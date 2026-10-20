#[tokio::main]
pub async fn main() {
	let uid = std::env::args().nth(1).unwrap().parse::<u64>().unwrap();

	let conn = zbus::Connection::system().await.unwrap();
	let user = accounts_zbus::UserProxy::from_uid(&conn, uid)
		.await
		.unwrap();

	println!("icon_file: {:?}", user.icon_file().await);
	println!("account type: {:?}", user.account_type().await);
	println!("password hint: {:?}", user.password_hint().await);
}
