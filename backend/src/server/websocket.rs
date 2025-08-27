use futures::{SinkExt, stream::StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use tracing::{debug, info, warn};

use rust_shared::{deserialize, request, response, serialize};

pub async fn handle_websocket<F, Fut>(raw_stream: TcpStream, handle_message: F)
where
    F: Fn(request::Message) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = response::Message> + Send + 'static,
{
    let ws_stream = accept_async(raw_stream).await.expect("accept_async failed");
    let (mut outgoing, mut incoming) = ws_stream.split();

    loop {
        let msg = incoming.next().await;
        let Some(msg) = msg else {
            debug!("socket closed");
            break;
        };

        // only process text messages
        let Ok(Message::Text(text)) = msg else {
            continue;
        };
        debug!("received message: {}", text);

        // parse message
        let parsed_message = match deserialize(&text) {
            Ok(parsed_message) => {
                info!("parsed message: {parsed_message:?}");
                parsed_message
            }
            Err(e) => {
                warn!("error parsing message: {e:?}");
                request::Message::Unknown(text.to_string())
            }
        };

        let response = handle_message(parsed_message).await;

        // serialize response
        let response_text = serialize(response, *crate::DEV);

        outgoing
            .send(Message::Text(response_text.into()))
            .await
            .expect("failed to send message");
    }
}
