use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::cmp::{max, min};

use crate::clipboard;

#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    pub position: usize,
    pub selection_anchor: Option<usize>,
}

pub struct TextBox {
    pub text: String,
    pub cursor: Cursor,
}

impl TextBox {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: Cursor::default(),
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(char) => {
                if self.handle_shortcut(char, key.modifiers) {
                    return;
                }

                // add char to search text at cursor position
                self.delete_selection();
                self.text.insert(self.cursor.position, char);
                self.cursor.position += 1;
            }
            KeyCode::Backspace => {
                // delete selection if there is one, else delete char to the left
                if self.delete_selection() {
                    return;
                }

                if self.cursor.position > 0 {
                    self.text.remove(self.cursor.position - 1);
                    self.cursor.position -= 1;
                }
            }
            KeyCode::Delete => {
                // delete selection if there is one, else delete char to the right
                if self.delete_selection() {
                    return;
                }

                if self.cursor.position < self.text.len() {
                    self.text.remove(self.cursor.position);
                }
            }
            KeyCode::Left => {
                if !key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.cursor.selection_anchor = None;
                }

                if self.cursor.position == 0 {
                    return;
                }

                // if shift is held, ensure we're selecting
                if self.cursor.selection_anchor.is_none()
                    && key.modifiers.contains(KeyModifiers::SHIFT)
                {
                    self.cursor.selection_anchor = Some(self.cursor.position);
                }

                // move cursor left
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.cursor.position =
                        get_next_word_bound(&self.text, self.cursor.position, false);
                } else {
                    self.cursor.position -= 1;
                }

                // if we return to the anchor, we're no longer selecting
                if let Some(anchor) = self.cursor.selection_anchor
                    && self.cursor.position == anchor
                {
                    self.cursor.selection_anchor = None;
                }
            }
            KeyCode::Right => {
                if !key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.cursor.selection_anchor = None;
                }

                if self.cursor.position == self.text.len() {
                    return;
                }

                // if shift is held, ensure we're selecting text
                if self.cursor.selection_anchor.is_none()
                    && key.modifiers.contains(KeyModifiers::SHIFT)
                {
                    self.cursor.selection_anchor = Some(self.cursor.position);
                }

                // move cursor right
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.cursor.position =
                        get_next_word_bound(&self.text, self.cursor.position, true);
                } else {
                    self.cursor.position += 1;
                }

                // if we return to the anchor, we're no longer selecting text
                if let Some(anchor) = self.cursor.selection_anchor
                    && self.cursor.position == anchor
                {
                    self.cursor.selection_anchor = None;
                }
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
                self.cursor.position = self.text.len();
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
                self.text.insert_str(self.cursor.position, &clipboard_text);
                self.cursor.position += clipboard_text.len();
            }
            _ => return false,
        }

        true
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
        Some(self.text[start..end].to_string())
    }

    fn delete_selection(&mut self) -> bool {
        let Some((start, end)) = self.get_selection_range() else {
            return false;
        };

        self.text.drain(start..end);
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

    cursor
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric()
}
