use futures::{SinkExt, stream::StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Utf8Bytes, protocol::Message},
};
use tracing::debug;

pub async fn handle_websocket<F, Fut>(raw_stream: TcpStream, handle_message: F)
where
    F: Fn(Utf8Bytes) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = Result<Utf8Bytes, ()>> + Send + 'static,
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

        let response = handle_message(text).await.expect("couldn't handle message");

        outgoing
            .send(Message::Text(response))
            .await
            .expect("failed to send message");
    }
}
