use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{Buffer, Rect, Style, Widget},
    style::Color,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};
use std::cmp::{max, min};

use rust_shared::response::Note as SharedNote;

use crate::text_box::InteractiveTextBox;

pub struct Note(SharedNote);

impl Note {
    pub fn new(note: SharedNote) -> Self {
        Self(note)
    }

    pub fn text(&self) -> String {
        self.0.text.clone()
    }
}

impl From<SharedNote> for Note {
    fn from(note: SharedNote) -> Self {
        Self(note)
    }
}

pub struct SearchBox {
    pub text_box: InteractiveTextBox,
    pub parsed_tags: Vec<String>,
    pub parsed_search_text: Option<String>,
}

impl Default for SearchBox {
    fn default() -> Self {
        Self {
            text_box: InteractiveTextBox::default().title("Search".to_string()),
            parsed_tags: Vec::new(),
            parsed_search_text: None,
        }
    }
}

impl SearchBox {
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        self.text_box.handle_key_event(key);

        let (tags, search_text) = parse_search_text(&self.text_box.text_box.text);
        self.parsed_tags = tags;
        self.parsed_search_text = search_text;
    }
}

/// Parse search text into tags and search text
/// Tags are prefixed with #, any whitespace separated, and precede the search text
/// The first word that doesn't start with # marks the end of the tags
/// Example: "#tag1   #tag2 search text" -> tags = ["tag1", "tag2"], search_text = "search text"
fn parse_search_text(text: &str) -> (Vec<String>, Option<String>) {
    let mut tags = Vec::new();
    let mut search_start = None;

    let mut tag_start = None;
    for (i, char) in text.char_indices() {
        if char == '#' {
            // tag found
            tag_start = Some(i + 1);
        } else if char.is_whitespace() {
            if let Some(tag_start_val) = tag_start {
                // end of this tag
                tags.push(text[tag_start_val..i].to_string());
                tag_start = None;
            }
            // else means extra whitespaces, ignore them
        } else if tag_start.is_none() {
            // some actual character outside of a tag
            search_start = Some(i);
            break;
        }
    }

    if let Some(tag_start_val) = tag_start {
        // the last tag wasn't closed with whitespace
        tags.push(text[tag_start_val..].to_string());
    }

    (tags, search_start.map(|val| text[val..].to_string()))
}

impl Widget for &SearchBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let default_style = Style::default();
        let cursor_color = Color::White;
        let selection_color = Color::LightGreen;
        let selection_cursor_color = Color::Yellow;
        let tag_color = Color::Blue;
        let text_color = Color::White;

        let selection_range = self.text_box.cursor.selection_anchor.map(|anchor| {
            (
                min(anchor, self.text_box.cursor.position),
                max(anchor, self.text_box.cursor.position),
            )
        });

        let mut spans = Vec::new();
        let mut num_words_seen = 0;
        let mut last_char_was_space = true;
        for (i, c) in self.text_box.text_box.text.chars().enumerate() {
            if !c.is_whitespace() && last_char_was_space {
                num_words_seen += 1;
            }
            last_char_was_space = c.is_whitespace();

            let fg_color = if num_words_seen > self.parsed_tags.len() {
                text_color
            } else {
                tag_color
            };

            if i == self.text_box.cursor.position {
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
        if self.text_box.cursor.position == self.text_box.text_box.text.len() {
            spans.push(Span::styled(" ", default_style.bg(cursor_color)));
        }

        let text_box = Paragraph::new(Line::from(spans))
            .block(Block::bordered().title(self.text_box.text_box.title.clone()));
        text_box.render(area, buf);
    }
}

#[derive(Default)]
pub struct SearchResultsBox {
    pub search_results: Vec<Note>,
    pub selected_index: usize,
}

impl SearchResultsBox {
    pub fn set_notes(&mut self, notes: Vec<SharedNote>) {
        self.search_results = notes.into_iter().map(Note::from).collect();
        self.selected_index = 0;
    }

    #[allow(clippy::collapsible_else_if)]
    pub fn move_selection(&mut self, direction_down: bool) {
        if self.search_results.is_empty() {
            self.selected_index = 0;
            return;
        }

        self.selected_index = if direction_down {
            if self.selected_index >= self.search_results.len() - 1 {
                self.search_results.len() - 1
            } else {
                self.selected_index + 1
            }
        } else {
            if self.selected_index == 0 {
                0
            } else {
                self.selected_index - 1
            }
        };
    }
}

impl Widget for &SearchResultsBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.search_results
            .iter()
            .enumerate()
            .take(area.height as usize)
            .for_each(|(i, note)| {
                let style = if i == self.selected_index {
                    Style::default().bg(Color::LightGreen)
                } else {
                    Style::default()
                };

                let line = Paragraph::new(note.text()).style(style);
                line.render(Rect::new(area.x, area.y + i as u16, area.width, 1), buf)
            });
    }
}
