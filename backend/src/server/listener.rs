use tokio::net::TcpListener;
use tracing::info;

use rust_shared::{request, response};

use crate::server::websocket::handle_websocket;

pub async fn start<F, Fut>(addr: &str, handle_message: F)
where
    F: Fn(request::Message) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = response::Message> + Send + 'static,
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
