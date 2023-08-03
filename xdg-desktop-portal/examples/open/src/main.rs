use xdg_desktop_portal::{file_chooser, request};
use futures_util::stream::StreamExt;
use zbus::Connection;

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let conn = Connection::session().await?;
    let file_chooser = file_chooser::Proxy::new(&conn).await?;

    let options = file_chooser::OpenOptions::default()
        .accept_label("_Open")
        .modal(true)
        .multiple(true)
        .directory(false)
        .current_folder("/usr/share/backgrounds".as_ref());

    let request = file_chooser.open_file("parent", "title", options).await?;

    let request = request::Proxy::builder(&conn)
        .path(request)?
        .build()
        .await?;


    let mut responses = request.receive_response().await?;

   if let Some(signal) = responses.next().await {
        let args = signal.args()?;

        match file_chooser::Response::try_from(&args.results) {
            Ok(response) => println!("{}: {:#?}", args.response, response),
            Err(why) => eprintln!("response = {}, error = {why}, value = {:?}", args.response, args.results),
        }
    }

   Ok(())
}