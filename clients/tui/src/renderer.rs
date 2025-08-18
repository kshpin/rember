use ratatui::{
    prelude::{Buffer, Rect, Style, Widget},
    style::Color,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let search_area = Rect::new(0, 0, area.width, 3).clamp(area);
        self.render_search_bar(&self.search_text, search_area, buf);
    }
}

impl App {
    fn render_search_bar(&self, text: &str, area: Rect, buf: &mut Buffer) {
        let line = if self.search_cursor_position == text.len() {
            // cursor is at the end of the text
            // render the text and a cursor at the end

            Line::from(vec![
                Span::raw(text),
                Span::raw(" ").style(Style::default().bg(Color::White)),
            ])
        } else {
            // cursor is in the middle of the text
            // render the text and highlight the character at the cursor position

            let text_before_cursor = &text[..self.search_cursor_position];
            let text_at_cursor =
                &text[self.search_cursor_position..self.search_cursor_position + 1];
            let text_after_cursor = &text[self.search_cursor_position + 1..];

            Line::from(vec![
                Span::raw(text_before_cursor),
                Span::styled(text_at_cursor, Style::default().bg(Color::White)),
                Span::raw(text_after_cursor),
            ])
        };

        let search_bar = Paragraph::new(line).block(Block::bordered().title("Search"));
        search_bar.render(area, buf);
    }
}
