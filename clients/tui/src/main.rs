use color_eyre::Result;
use crossterm::event::EventStream;
use ratatui::prelude::{Buffer, Rect, Widget};
use std::sync::LazyLock;

use rust_shared::request;

mod client;
mod clipboard;
mod events;
mod focus;
mod keys;
mod search;
mod text_box;

use client::websocket::WebSocketClient;
use focus::Focus;
use search::{SearchBox, SearchResultsBox};
use text_box::InteractiveTextBox;
use text_box::TextBox;

// set develop flag
pub static DEV: LazyLock<bool> =
    LazyLock::new(|| std::env::var("RUST_BACKTRACE").unwrap_or("0".to_string()) == "1");

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    App::new().run().await
}

#[derive(Default)]
pub struct App {
    running: bool,
    crossterm_event_stream: EventStream,

    focus: Focus,

    search: SearchBox,
    search_results: SearchResultsBox,
    response_box: TextBox,
    new_note: InteractiveTextBox,

    websocket_client: WebSocketClient,
}

impl App {
    fn new() -> Self {
        Self {
            new_note: InteractiveTextBox::default().title("New Note".to_string()),
            ..Default::default()
        }
    }
}

impl App {
    pub async fn run(mut self) -> Result<()> {
        let Ok((outgoing_thread, incoming_thread)) = self
            .websocket_client
            .connect_and_run("ws://localhost:3210")
            .await
        else {
            eprintln!("websocket connection failed - is the backend running?");
            return Ok(());
        };

        // request the whole list of tags first thing
        self.websocket_client
            .send(request::Message::GetTags)
            .await
            .expect("msg");

        // main loop
        let mut terminal = ratatui::init();
        self.running = true;
        while self.running {
            self.handle_events().await?;
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
        }
        ratatui::restore();

        // clean up
        outgoing_thread.abort();
        incoming_thread.abort();
        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let search_area = Rect::new(0, 0, area.width, 3).clamp(area);
        self.search.render(search_area, buf);

        let search_results_area = Rect::new(1, 3, area.width - 2, 3).clamp(area);
        self.search_results.render(search_results_area, buf);

        let response_area = Rect::new(0, 6, area.width, area.height - 6).clamp(area);
        self.response_box.render(response_area, buf);

        if self.focus == Focus::NewNote {
            // vertical middle third of the screen
            let new_note_area =
                Rect::new(0, area.height / 3, area.width, area.height / 3).clamp(area);
            self.new_note.render(new_note_area, buf);
        }
    }
}
