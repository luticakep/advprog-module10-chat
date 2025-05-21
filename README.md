# Kayla Soraya Djakaria - 2306256381

### 2.1 Original code, and how it run
![](/images/image.png)

<!-- How to run it, and what happens when you type some text in the clients.
 -->

I first run the server in one terminal window:
```bash
cargo run --bin server
```

And run the client in 3 terminal windows:
```bash
cargo run --bin client
```

After running each client, it will print a new connection with the url in the server terminal. When I type a message in any client and press Enter, that message is sent to the server and then broadcast back to all connected clients. Each client therefore displays not only its own messages but also those from the other two, in the order they arrive. In the screenshot you can see “hello”, “hello2”, and “hello3” sent from the three different clients, followed by “hihi”, “haha”, and “hehe” echoed back. This demonstrates that the server’s broadcast channel fan-outs each incoming text frame to every client. The non‐blocking `tokio::select!` loops in both server and clients ensure that typing and receiving happen concurrently without deadlock.
