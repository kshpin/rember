use std::io::Stdout;

use color_eyre::Result;
use crossterm::event::EventStream;
use ratatui::{Terminal, prelude::CrosstermBackend};

use client::websocket::WebSocketClient;

mod client;
mod events;
mod keys;
mod renderer;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    crossterm_event_stream: EventStream,
    search_text: String,
    search_cursor_position: usize,
    websocket_client: WebSocketClient,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            crossterm_event_stream: EventStream::default(),
            search_text: String::new(),
            search_cursor_position: 0,
            websocket_client: WebSocketClient::new(),
        }
    }

    pub async fn run(mut self, mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        let (outgoing_handle, incoming_handle) = self
            .websocket_client
            .connect_and_run("ws://localhost:3210")
            .await
            .expect("websocket connection failed - is the backend running?");

        self.running = true;
        while self.running {
            self.handle_events().await?;
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
        }

        outgoing_handle.abort();
        incoming_handle.abort();
        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
