use tokio::net::TcpListener;
use tracing::info;

use crate::server::websocket::handle_websocket;

pub async fn start(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            handle_websocket(stream).await;
        });
    }
}
