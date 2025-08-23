use ratatui::{
    prelude::{Buffer, Rect, Style, Widget},
    style::Color,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};
use std::cmp::{max, min};

use crate::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let search_area = Rect::new(0, 0, area.width, 3).clamp(area);
        self.render_search_bar(&self.search_box.text, search_area, buf);
    }
}

impl App {
    fn render_search_bar(&self, text: &str, area: Rect, buf: &mut Buffer) {
        let cursor_style = Style::default().bg(Color::White);
        let selection_style = Style::default().bg(Color::LightBlue);

        let selection_range = self.search_box.cursor.selection_anchor.map(|anchor| {
            (
                min(anchor, self.search_box.cursor.position),
                max(anchor, self.search_box.cursor.position),
            )
        });

        let mut spans = Vec::new();

        for (i, c) in text.chars().enumerate() {
            if let Some((selection_left, selection_right)) = selection_range
                && i >= selection_left
                && i < selection_right
            {
                spans.push(Span::styled(c.to_string(), selection_style));
            } else if i == self.search_box.cursor.position {
                spans.push(Span::styled(c.to_string(), cursor_style));
            } else {
                spans.push(Span::raw(c.to_string()));
            }
        }

        // render cursor if it's at the end of the text
        if self.search_box.cursor.position == text.len() {
            spans.push(Span::styled(" ", cursor_style));
        }

        let search_bar = Paragraph::new(Line::from(spans)).block(Block::bordered().title("Search"));
        search_bar.render(area, buf);
    }
}
