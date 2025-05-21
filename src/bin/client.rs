use futures_util::{SinkExt, StreamExt};
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    // connect to the chat server
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut lines = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            // read a line & send
            Ok(Some(line)) = lines.next_line() => {
                ws_stream.send(Message::text(line)).await?;
            }
            // receive from server + print
            Some(Ok(msg)) = ws_stream.next() => {
                if let Some(txt) = msg.as_text() {
                    println!("{txt}");
                }
            }
            // if EOF, then quit
            else => break,
        }
    }

    Ok(())
}
