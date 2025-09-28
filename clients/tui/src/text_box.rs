use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::{Buffer, Rect, Style, Widget},
    style::Color,
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};
use std::cmp::{max, min};

use crate::clipboard;

#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    pub position: usize,
    pub selection_anchor: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct TextBox {
    pub title: String,
    pub text: String,
}

#[derive(Debug, Clone, Default)]
pub struct InteractiveTextBox {
    pub text_box: TextBox,
    pub cursor: Cursor,
}

impl TextBox {
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
}

impl InteractiveTextBox {
    pub fn title(mut self, title: String) -> Self {
        self.text_box.title = title;
        self
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(char) => {
                if self.handle_shortcut(char, key.modifiers) {
                    return;
                }

                // add char to search text at cursor position
                self.delete_selection();
                self.text_box.text.insert(self.cursor.position, char);
                self.cursor.position += 1;
            }
            KeyCode::Backspace => {
                // delete selection if there is one, else delete char to the left
                if self.delete_selection() {
                    return;
                }

                if self.cursor.position > 0 {
                    self.text_box.text.remove(self.cursor.position - 1);
                    self.cursor.position -= 1;
                }
            }
            KeyCode::Delete => {
                // delete selection if there is one, else delete char to the right
                if self.delete_selection() {
                    return;
                }

                if self.cursor.position < self.text_box.text.len() {
                    self.text_box.text.remove(self.cursor.position);
                }
            }
            KeyCode::Left => {
                self.handle_horizontal_move(false, key.modifiers);
            }
            KeyCode::Right => {
                self.handle_horizontal_move(true, key.modifiers);
            }
            _ => {}
        }
    }

    fn handle_shortcut(&mut self, char: char, modifiers: KeyModifiers) -> bool {
        if !modifiers.contains(KeyModifiers::CONTROL) {
            return false;
        }

        match char {
            'a' => {
                // select all
                self.cursor.selection_anchor = Some(0);
                self.cursor.position = self.text_box.text.len();
            }
            'c' => {
                // copy selection to clipboard
                if let Some(selection) = self.get_selection() {
                    clipboard::set_text(selection);
                }
            }
            'x' => {
                // cut selection to clipboard
                if let Some(selection) = self.get_selection() {
                    clipboard::set_text(selection);
                    self.delete_selection();
                }
            }
            'v' => {
                // paste from clipboard
                self.delete_selection();
                let clipboard_text = clipboard::get_text();
                self.text_box
                    .text
                    .insert_str(self.cursor.position, &clipboard_text);
                self.cursor.position += clipboard_text.len();
            }
            _ => return false,
        }

        true
    }

    fn handle_horizontal_move(&mut self, direction_right: bool, modifiers: KeyModifiers) {
        let selecting = modifiers.contains(KeyModifiers::SHIFT);

        // if not selecting and we have a selection, collapse to the respective edge
        if !selecting {
            if let Some((start, end)) = self.get_selection_range() {
                self.cursor.position = if direction_right { end } else { start };
                self.cursor.selection_anchor = None;
                return;
            }
            // ensure selection cleared when moving without Shift
            self.cursor.selection_anchor = None;
        } else if self.cursor.selection_anchor.is_none() {
            // starting selection: anchor at current position
            self.cursor.selection_anchor = Some(self.cursor.position);
        }

        // bounds
        if (!direction_right && self.cursor.position == 0)
            || (direction_right && self.cursor.position == self.text_box.text.len())
        {
            // if we just started selecting into a bound, cancel selection
            if selecting && self.cursor.selection_anchor == Some(self.cursor.position) {
                self.cursor.selection_anchor = None;
            }
            return;
        }

        // move by word or char
        self.cursor.position = if modifiers.contains(KeyModifiers::CONTROL) {
            get_next_word_bound(&self.text_box.text, self.cursor.position, direction_right)
        } else if direction_right {
            self.cursor.position + 1
        } else {
            self.cursor.position - 1
        };

        // if we returned to the anchor while selecting, stop selecting
        if let Some(anchor) = self.cursor.selection_anchor
            && self.cursor.position == anchor
        {
            self.cursor.selection_anchor = None;
        }
    }

    fn get_selection_range(&self) -> Option<(usize, usize)> {
        self.cursor.selection_anchor.map(|anchor| {
            (
                min(anchor, self.cursor.position),
                max(anchor, self.cursor.position),
            )
        })
    }

    fn get_selection(&self) -> Option<String> {
        let (start, end) = self.get_selection_range()?;
        Some(self.text_box.text[start..end].to_string())
    }

    fn delete_selection(&mut self) -> bool {
        let Some((start, end)) = self.get_selection_range() else {
            return false;
        };

        self.text_box.text.drain(start..end);
        self.cursor.selection_anchor = None;
        self.cursor.position = start;

        true
    }
}

fn get_next_word_bound(text: &str, mut cursor: usize, direction_right: bool) -> usize {
    let text = text.as_bytes();
    let direction_left = !direction_right;

    // limit check
    if direction_left && cursor == 0 {
        return 0;
    }
    if direction_right && cursor >= text.len() {
        return text.len();
    }

    // classify current position
    // if moving right, and we're at a word bound, we're already in the next block
    // if moving left, and we're at a word bound, we're still in the current block
    let cur_char_type_is_word = (direction_right && is_word_char(text[cursor] as char))
        || (direction_left && is_word_char(text[cursor - 1] as char));

    // move cursor to next word bound
    let is_word_char = |c: u8| is_word_char(c as char);
    while (direction_left && cursor > 0 && cur_char_type_is_word == is_word_char(text[cursor - 1]))
        || (direction_right
            && cursor < text.len()
            && cur_char_type_is_word == is_word_char(text[cursor]))
    {
        if direction_left {
            cursor -= 1;
        } else {
            cursor += 1;
        }
    }

    // cursor should end up at the word side of the bound
    // if we started on the word side, only need to move the cursor once
    if cur_char_type_is_word {
        return cursor;
    }

    // move cursor to next word bound again
    let cur_char_type_is_word = !cur_char_type_is_word;
    while (direction_left && cursor > 0 && cur_char_type_is_word == is_word_char(text[cursor - 1]))
        || (direction_right
            && cursor < text.len()
            && cur_char_type_is_word == is_word_char(text[cursor]))
    {
        if direction_left {
            cursor -= 1;
        } else {
            cursor += 1;
        }
    }

    cursor
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '#'
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
