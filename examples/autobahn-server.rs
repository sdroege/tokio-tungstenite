use futures::{SinkExt, StreamExt};
use log::*;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    info!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        let msg = msg.expect("Failed to get request");
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await.expect("Failed to send response");
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = "127.0.0.1:9002"
        .to_socket_addrs()
        .expect("Not a valid address")
        .next()
        .expect("Not a socket address");
    let mut listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }
}
