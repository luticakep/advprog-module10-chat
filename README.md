# Kayla Soraya Djakaria - 2306256381

### 2.1 Original code, and how it run
![](/images/image.png)

I first run the server in one terminal window:
```bash
cargo run --bin server
```

And run the client in 3 terminal windows:
```bash
cargo run --bin client
```

After running each client, it will print a new connection with the url in the server terminal. When I type a message in any client and press Enter, that message is sent to the server and then broadcast back to all connected clients. Each client therefore displays not only its own messages but also those from the other two, in the order they arrive. In the screenshot you can see “hello”, “hello2”, and “hello3” sent from the three different clients, followed by “hihi”, “haha”, and “hehe” echoed back. This demonstrates that the server’s broadcast channel fan-outs each incoming text frame to every client. The non‐blocking `tokio::select!` loops in both server and clients ensure that typing and receiving happen concurrently without deadlock.

### 2.2 Modifying port
I updated both sides of the connection to use port 8080. In **server.rs** I changed TcpListener::bind("127.0.0.1:2000") to TcpListener::bind("127.0.0.1:8080"), and in **client.rs** I updated the URI from `ws://127.0.0.1:2000` to `ws://127.0.0.1:8080`. After rebuilding and re-running `cargo run --bin server` alongside each `cargo run --bin client`, the chat still functions as before. Both client and server continue to speak the WebSocket protocol (indicated by the ws:// scheme), which the `tokio-websockets` crate negotiates via an HTTP upgrade before exchanging frames. No other source files or configuration needed adjustment, since the port is only referenced in those two locations. This confirms that changing the port on both ends is sufficient to rebind the entire bi-directional WebSocket connection.
