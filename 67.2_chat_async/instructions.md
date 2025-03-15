# Broadcast Chat Application

In this exercise, we want to use our new knowledge to implement a broadcast chat application. We have a chat server that the clients connect to and publish their messages. The client reads user messages from the standard input, and sends them to the server. The chat server broadcasts each message that it receives to all the clients.

For this, we use a [broadcast channel](https://docs.rs/tokio/latest/tokio/sync/broadcast/fn.channel.html) on the server, and [tokio_websockets](https://docs.rs/tokio-websockets/) for the communication between the client and the server.

## The required APIs

You are going to need the following functions from tokio and [tokio_websockets](https://docs.rs/tokio-websockets/). Spend a few minutes to familiarize yourself with the API.

- [StreamExt::next()](https://docs.rs/futures-util/0.3.28/futures_util/stream/trait.StreamExt.html#method.next) implemented by WebSocketStream: for asynchronously reading messages from a Websocket Stream.
- [SinkExt::send()](https://docs.rs/futures-util/0.3.28/futures_util/sink/trait.SinkExt.html#method.send) implemented by WebSocketStream: for asynchronously sending messages on a Websocket Stream.
- [Lines::next_line()](https://docs.rs/tokio/latest/tokio/io/struct.Lines.html#method.next_line): for asynchronously reading user messages from the standard input.
- [Sender::subscribe()](https://docs.rs/tokio/latest/tokio/sync/broadcast/struct.Sender.html#method.subscribe): for subscribing to a broadcast channel.


## Tasks
- Implement the handle_connection function in src/bin/server.rs.  
    - Hint: Use tokio::select! for concurrently performing two tasks in a continuous loop. One task receives messages from the client and broadcasts them. The other sends messages received by the server to the client.
- Complete the main function in src/bin/client.rs.
    - Hint: As before, use tokio::select! in a continuous loop for concurrently performing two tasks: (1) reading user messages from standard input and sending them to the server, and (2) receiving messages from the server, and displaying them for the user.
   
-  Optional: Once you are done, change the code to broadcast messages to all clients, but the sender of the message.
