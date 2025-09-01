use ratatui::{
    prelude::{Buffer, Rect, Style, Widget},
    style::Color,
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};
use std::cmp::{max, min};

use crate::{App, InteractiveTextBox, Search, TextBox};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let search_area = Rect::new(0, 0, area.width, 6).clamp(area);
        self.search.render(search_area, buf);

        let response_area = Rect::new(0, 6, area.width, area.height - 6).clamp(area);
        self.response_box.render(response_area, buf);
    }
}

impl Widget for &TextBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text_box = Paragraph::new(self.text.clone())
            .block(Block::bordered().title(self.title.clone()))
            .wrap(Wrap { trim: true });
        text_box.render(area, buf);
    }
}

impl Widget for &InteractiveTextBox {
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
        for (i, c) in self.text_box.text.chars().enumerate() {
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
        if self.cursor.position == self.text_box.text.len() {
            spans.push(Span::styled(" ", cursor_style));
        }

        let text_box = Paragraph::new(Line::from(spans))
            .block(Block::bordered().title(self.text_box.title.clone()));
        text_box.render(area, buf);
    }
}

impl Widget for &Search {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let default_style = Style::default();
        let cursor_color = Color::White;
        let selection_color = Color::LightGreen;
        let selection_cursor_color = Color::Yellow;
        let tag_color = Color::Blue;
        let text_color = Color::White;

        let selection_range = self.search_box.cursor.selection_anchor.map(|anchor| {
            (
                min(anchor, self.search_box.cursor.position),
                max(anchor, self.search_box.cursor.position),
            )
        });

        let mut spans = Vec::new();
        let mut num_words_seen = 0;
        let mut last_char_was_space = true;
        for (i, c) in self.search_box.text_box.text.chars().enumerate() {
            if !c.is_whitespace() && last_char_was_space {
                num_words_seen += 1;
            }
            last_char_was_space = c.is_whitespace();

            let fg_color = if num_words_seen > self.parsed_tags.len() {
                text_color
            } else {
                tag_color
            };

            if i == self.search_box.cursor.position {
                let color = if let Some((selection_left, _)) = selection_range
                    && i == selection_left
                {
                    selection_cursor_color
                } else {
                    cursor_color
                };
                spans.push(Span::styled(
                    c.to_string(),
                    default_style.bg(color).fg(fg_color),
                ));
            } else if let Some((selection_left, selection_right)) = selection_range
                && i >= selection_left
                && i < selection_right
            {
                spans.push(Span::styled(
                    c.to_string(),
                    default_style.bg(selection_color).fg(fg_color),
                ));
            } else {
                spans.push(Span::styled(c.to_string(), default_style.fg(fg_color)));
            }
        }

        // render cursor if it's at the end of the text
        if self.search_box.cursor.position == self.search_box.text_box.text.len() {
            spans.push(Span::styled(" ", default_style.bg(cursor_color)));
        }

        let text_box = Paragraph::new(Line::from(spans))
            .block(Block::bordered().title(self.search_box.text_box.title.clone()));
        text_box.render(area, buf);
    }
}
