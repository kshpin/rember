use axum::{Router, routing::any};
use tokio::net::TcpListener;

use crate::server::websocket::websocket_handler;

pub async fn start(addr: &str) {
    let app = Router::new().route("/ws", any(websocket_handler));

    tracing::debug!("listening on {}", addr);

    axum::serve(TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
