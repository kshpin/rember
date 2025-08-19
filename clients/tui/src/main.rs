use color_eyre::Result;
use crossterm::event::EventStream;

use client::websocket::WebSocketClient;

mod client;
mod events;
mod keys;
mod renderer;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    App::new().run().await
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

    pub async fn run(mut self) -> Result<()> {
        let Ok((outgoing_thread, incoming_thread)) = self
            .websocket_client
            .connect_and_run("ws://localhost:3210")
            .await
        else {
            eprintln!("websocket connection failed - is the backend running?");
            return Ok(());
        };

        let mut terminal = ratatui::init();
        self.running = true;
        while self.running {
            self.handle_events().await?;
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
        }
        ratatui::restore();

        outgoing_thread.abort();
        incoming_thread.abort();
        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
