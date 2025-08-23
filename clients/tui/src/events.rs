use color_eyre::Result;
use crossterm::event::{Event, KeyEventKind};
use futures::{FutureExt, StreamExt};

use crate::App;

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

    async fn handle_websocket_message(&mut self, message: Option<String>) {
        if let Some(message) = message {
            println!("Received message from backend: {message}");
        }
    }
}

async fn event_timeout(timeout_ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(timeout_ms)).await;
}
