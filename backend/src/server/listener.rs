use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Utf8Bytes;
use tracing::info;

use crate::server::websocket::handle_websocket;

pub async fn start<F, Fut>(addr: &str, handle_message: F)
where
    F: Fn(Utf8Bytes) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = Result<Utf8Bytes, String>> + Send + 'static,
{
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let handle = handle_message.clone();
        tokio::spawn(async move {
            handle_websocket(stream, handle).await;
        });
    }
}
