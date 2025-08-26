use ratatui::{
    prelude::{Buffer, Rect, Style, Widget},
    style::Color,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};
use std::cmp::{max, min};

use crate::{App, TextBox};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let search_area = Rect::new(0, 0, area.width, 3).clamp(area);
        self.search_box.render(search_area, buf);

        let response_area = Rect::new(0, 3, area.width, area.height - 3).clamp(area);
        self.response_box.render(response_area, buf);
    }
}

impl Widget for &TextBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cursor_style = Style::default().bg(Color::White);
        let selection_style = Style::default().bg(Color::LightGreen);
        let selection_cursor_style = Style::default().bg(Color::Yellow);

        let selection_range = self.cursor.selection_anchor.map(|anchor| {
            (
                min(anchor, self.cursor.position),
                max(anchor, self.cursor.position),
            )
        });

        let mut spans = Vec::new();
        for (i, c) in self.text.chars().enumerate() {
            if i == self.cursor.position {
                let style = if let Some((selection_left, _)) = selection_range
                    && i == selection_left
                {
                    selection_cursor_style
                } else {
                    cursor_style
                };
                spans.push(Span::styled(c.to_string(), style));
            } else if let Some((selection_left, selection_right)) = selection_range
                && i >= selection_left
                && i < selection_right
            {
                spans.push(Span::styled(c.to_string(), selection_style));
            } else {
                spans.push(Span::raw(c.to_string()));
            }
        }

        // render cursor if it's at the end of the text
        if self.cursor.position == self.text.len() {
            spans.push(Span::styled(" ", cursor_style));
        }

        let text_box =
            Paragraph::new(Line::from(spans)).block(Block::bordered().title(self.title.clone()));
        text_box.render(area, buf);
    }
}
