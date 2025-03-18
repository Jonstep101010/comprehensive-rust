use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

///
/// `addr` IP address
async fn handle_connection(
	addr: SocketAddr,
	mut ws_stream: WebSocketStream<TcpStream>,
	bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
	// TODO: For a hint, see the description of the task below.
	// initialize client connection:
	// send greeting and create broadcast receiving handle
	ws_stream
		.send(Message::text("Welcome to the broadcast chat"))
		.await?;
	let mut bcast_rx = bcast_tx.subscribe();
	loop {
		tokio::select! {
			// recv messages from client
			received_from_client = ws_stream.next() => {
				match received_from_client {
					None => return Ok(()),
					Some(Err(err)) => return Err(err.into()),
					Some(Ok(msg)) => {
						if let Some(text) = msg.as_text() {
							println!("From {addr:?}: {text:?}");
							bcast_tx.send(text.into())?;
						}
					}
				}
			}
			// send broadcasts to client
			broadcast_for_client = bcast_rx.recv() => {
				ws_stream.send(Message::text(broadcast_for_client?)).await?;
			}
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
	let (bcast_tx, _) = channel(16);

	let listener = TcpListener::bind("127.0.0.1:2000").await?;
	println!("listening on port 2000");

	loop {
		let (socket, addr) = listener.accept().await?;
		println!("New connection from {addr:?}");
		let bcast_tx = bcast_tx.clone();
		tokio::spawn(async move {
			// Wrap the raw TCP stream into a websocket.
			let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

			handle_connection(addr, ws_stream, bcast_tx).await
		});
	}
}
