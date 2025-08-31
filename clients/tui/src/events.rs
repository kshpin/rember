use color_eyre::Result;
use crossterm::event::{Event, KeyEventKind};
use futures::{FutureExt, StreamExt};

use crate::App;

use rust_shared::response;

impl App {
    pub async fn handle_events(&mut self) -> Result<()> {
        tokio::select! {
            event = self.crossterm_event_stream.next().fuse() => self.handle_crossterm_events(event).await,
            message = self.websocket_client.recv() => self.handle_websocket_message(message).await,
            _ = event_timeout(100).fuse() => {}
        }
        Ok(())
    }

    async fn handle_crossterm_events(&mut self, event: Option<Result<Event, std::io::Error>>) {
        let Some(Ok(event)) = event else {
            return;
        };

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key).await,
            Event::Mouse(_) => {} // all my homies hate mice
            Event::Resize(_, _) => {}
            _ => {}
        }
    }

    async fn handle_websocket_message(&mut self, message: Option<response::Message>) {
        let Some(message) = message else {
            return;
        };

        match message {
            response::Message::Notes(notes) => {
                let text = notes
                    .iter()
                    .map(|note| note.text.clone())
                    .collect::<Vec<_>>()
                    .join("\n");
                self.response_box.text = text;
            }
            response::Message::Tags(tags) => {
                let text = tags
                    .iter()
                    .map(|tag| "#".to_string() + &tag.name.clone())
                    .collect::<Vec<_>>()
                    .join("\n");
                self.response_box.text = text;
            }
            response::Message::Unknown(msg) => {
                self.response_box.text = format!("{msg:?}");
            }
        }
    }
}

async fn event_timeout(timeout_ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(timeout_ms)).await;
}
