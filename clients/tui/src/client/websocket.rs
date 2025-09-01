use color_eyre::Result;
use futures::{SinkExt, stream::StreamExt};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use rust_shared::{deserialize, request, response, serialize};

#[derive(Debug, Default)]
pub struct WebSocketClient {
    app_to_server_tx: Option<mpsc::Sender<String>>,
    server_to_app_rx: Option<mpsc::Receiver<String>>,
}

impl WebSocketClient {
    pub async fn connect_and_run(&mut self, url: &str) -> Result<(JoinHandle<()>, JoinHandle<()>)> {
        let (app_to_server_tx, mut app_to_server_rx) = mpsc::channel(100);
        let (server_to_app_tx, server_to_app_rx) = mpsc::channel(100);

        self.app_to_server_tx = Some(app_to_server_tx);
        self.server_to_app_rx = Some(server_to_app_rx);

        let (ws_stream, _) = connect_async(url).await?;
        let (mut outgoing, mut incoming) = ws_stream.split();

        // Handle messages app -> backend
        let outgoing_thread = tokio::spawn(async move {
            while let Some(msg) = app_to_server_rx.recv().await {
                let send_result = outgoing.send(Message::Text(msg.into())).await;

                if send_result.is_err() {
                    break;
                }
            }
        });

        // Handle messages app <- backend
        let incoming_thread = tokio::spawn(async move {
            while let Some(Ok(msg)) = incoming.next().await {
                let send_result = server_to_app_tx.try_send(msg.to_string());

                if send_result.is_err() {
                    break;
                }
            }
        });

        Ok((outgoing_thread, incoming_thread))
    }

    pub async fn send(&self, message: request::Message) -> Result<()> {
        let Some(app_to_server_tx) = &self.app_to_server_tx else {
            return Ok(());
        };

        let msg_text = serialize(message, *crate::DEV);
        app_to_server_tx.send(msg_text).await.map_err(|e| e.into())
    }

    pub async fn recv(&mut self) -> Option<response::Message> {
        let Some(server_to_app_rx) = &mut self.server_to_app_rx else {
            return None;
        };

        let msg_text = server_to_app_rx.recv().await?;
        let msg = deserialize(&msg_text).ok()?;
        Some(msg)
    }
}
