use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::stream::StreamExt;

pub async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(text) = msg {
            tracing::debug!("received message: {}", text);

            if socket.send(Message::Text(text.clone())).await.is_err() {
                break;
            }
        }
    }

    tracing::debug!("client disconnected");
}
