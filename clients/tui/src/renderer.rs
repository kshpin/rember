use ratatui::{
    prelude::{Buffer, Rect, Widget},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();
        let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";
        Paragraph::new(text)
            .block(Block::bordered().title(title))
            .centered()
            .render(area, buf);
    }
}
