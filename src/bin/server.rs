use futures_util::{sink::SinkExt, stream::StreamExt};
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};
use tokio::net::TcpListener;

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // each connection gets its own subscriber
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // read frames from the client…
            incoming = ws_stream.next() => match incoming {
                Some(Ok(msg)) => {
                    if let Some(txt) = msg.as_text() {
                        println!("From client {addr} \"{txt}\"");
                        let _ = bcast_tx.send(format!("Kayla's Computer - From server [{addr}]: {txt}"));
                    }
                }
                Some(Err(err)) => {
                    eprintln!("WS error from {addr}: {err}");
                    break;
                }
                None => {
                    // client disconnected
                    break;
                }
            },

            Ok(txt) = bcast_rx.recv() => {
                ws_stream.send(Message::text(txt)).await?;
            }
        }
    }


    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}
