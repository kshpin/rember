use futures::{SinkExt, stream::StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use tracing::{debug, info, warn};

use rust_shared as shared;

pub async fn handle_websocket<F, Fut>(raw_stream: TcpStream, handle_message: F)
where
    F: Fn(shared::MessageRequest) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = shared::MessageResponse> + Send + 'static,
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
        let parsed_message = match serde_json::from_str(&text) {
            Ok(parsed_message) => {
                info!("parsed message: {parsed_message:?}");
                parsed_message
            }
            Err(e) => {
                warn!("error parsing message: {e:?}");
                shared::MessageRequest::Unknown(text.to_string())
            }
        };

        let response = handle_message(parsed_message).await;

        // serialize response
        let response_text = serialize_response(response);

        outgoing
            .send(Message::Text(response_text.into()))
            .await
            .expect("failed to send message");
    }
}

fn serialize_response(response: shared::MessageResponse) -> String {
    let serialize = if *crate::DEV {
        serde_json::to_string_pretty
    } else {
        serde_json::to_string
    };

    // this should never fail
    serialize(&response).expect("failed to serialize response")
}
