use futures::{SinkExt, stream::StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Utf8Bytes, protocol::Message},
};
use tracing::{debug, error};

pub async fn handle_websocket<F, Fut>(raw_stream: TcpStream, handle_message: F)
where
    F: Fn(Utf8Bytes) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = Result<Utf8Bytes, String>> + Send + 'static,
{
    let ws_stream = accept_async(raw_stream).await.expect("accept_async failed");
    let (mut outgoing, mut incoming) = ws_stream.split();

    loop {
        let msg = incoming.next().await;
        let Some(msg) = msg else {
            debug!("socket closed");
            break;
        };

        let Ok(Message::Text(text)) = msg else {
            continue;
        };
        debug!("received message: {}", text);

        let response = match handle_message(text).await {
            Ok(response) => response,
            Err(err) => {
                let err_msg = format!("Error handling message: {err}");
                error!("{err_msg}");
                Utf8Bytes::from(err_msg)
            }
        };

        outgoing
            .send(Message::Text(response))
            .await
            .expect("failed to send message");
    }
}
