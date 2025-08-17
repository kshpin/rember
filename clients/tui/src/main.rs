use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

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
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            self.handle_events().await?;
            terminal.draw(|frame| self.render(frame))?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();
        let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }

    async fn handle_events(&mut self) -> Result<()> {
        tokio::select! {
            event = self.crossterm_event_stream.next().fuse() => self.handle_crossterm_events(event).await,
            _ = event_timeout(100).fuse() => {}
        }
        Ok(())
    }

    async fn handle_crossterm_events(&mut self, event: Option<Result<Event, std::io::Error>>) {
        let Some(Ok(event)) = event else {
            return;
        };

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {} // all my homies hate mice
            Event::Resize(_, _) => {}
            _ => {}
        }
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}

async fn event_timeout(timeout_ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(timeout_ms)).await;
}
