use std::io::Stdout;

use color_eyre::Result;
use crossterm::event::EventStream;
use ratatui::{Terminal, prelude::CrosstermBackend};

mod events;
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
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            crossterm_event_stream: EventStream::default(),
            search_text: String::new(),
        }
    }

    pub async fn run(mut self, mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        self.running = true;
        while self.running {
            self.handle_events().await?;
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
