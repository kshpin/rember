use futures::{SinkExt, stream::StreamExt};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Debug, Default)]
pub struct WebSocketClient {
    app_to_client_tx: Option<mpsc::Sender<String>>,
    client_to_app_rx: Option<mpsc::Receiver<String>>,
}

impl WebSocketClient {
    pub fn new() -> Self {
        Self {
            app_to_client_tx: None,
            client_to_app_rx: None,
        }
    }

    pub async fn connect_and_run(
        &mut self,
        url: &str,
    ) -> Result<(JoinHandle<()>, JoinHandle<()>), Box<dyn std::error::Error>> {
        let (app_to_client_tx, mut app_to_client_rx) = mpsc::channel::<String>(100);
        let (client_to_app_tx, client_to_app_rx) = mpsc::channel::<String>(100);

        self.app_to_client_tx = Some(app_to_client_tx);
        self.client_to_app_rx = Some(client_to_app_rx);

        let (ws_stream, _) = connect_async(url).await?;
        let (mut outgoing, mut incoming) = ws_stream.split();

        // Handle messages app -> backend
        let outgoing_handle = tokio::spawn(async move {
            while let Some(msg) = app_to_client_rx.recv().await {
                let send_result = outgoing.send(Message::Text(msg.into())).await;

                if send_result.is_err() {
                    break;
                }
            }
        });

        // Handle messages app <- backend
        let incoming_handle = tokio::spawn(async move {
            while let Some(Ok(msg)) = incoming.next().await {
                let send_result = client_to_app_tx.try_send(msg.to_string());

                if send_result.is_err() {
                    break;
                }
            }
        });

        Ok((outgoing_handle, incoming_handle))
    }

    pub async fn send(&self, message: String) -> Result<(), mpsc::error::SendError<String>> {
        let Some(app_to_client_tx) = &self.app_to_client_tx else {
            return Err(mpsc::error::SendError(
                "send channel doesn't exist".to_string(),
            ));
        };

        app_to_client_tx.send(message).await
    }

    pub async fn recv(&mut self) -> Option<String> {
        let Some(client_to_app_rx) = &mut self.client_to_app_rx else {
            return None;
        };

        client_to_app_rx.recv().await
    }
}
