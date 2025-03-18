use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
	// socket connection to server
	let (mut ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
		.connect()
		.await?;

	let stdin = tokio::io::stdin();
	let mut stdin = BufReader::new(stdin).lines();

	loop {
		tokio::select! {
			incoming = ws_stream.next() => {
				// destructure Option<Result<Option<String>>>
				match incoming {
					Some(Ok(message)) => {
						if let Some(msg) = message.as_text() {
							println!("From server: {msg:?}");
						}
					},
					Some(Err(err)) => return Err(err.into()),
					None => return Ok(()),
				}
			}
			outgoing = stdin.next_line() => {
				// destructure io::Result<Option<String>>
				match outgoing {
					Err(err) =>
						return Err(err.into()),
					Ok(None) => return Ok(()),
					Ok(Some(stdin_input)) => {
						ws_stream.send(Message::text(stdin_input.to_string())).await?;
					},
				}

			}
		}
	}
}
