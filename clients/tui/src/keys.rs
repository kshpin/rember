use clipboard_rs::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::cmp::{max, min};

use crate::App;

#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    pub position: usize,
    pub selection_anchor: Option<usize>,
}

impl App {
    pub async fn on_key_event(&mut self, key: KeyEvent) {
        if self.maybe_navigate(key) {
            return;
        }

        match key.code {
            KeyCode::Char(char) => {
                // handle keyboard shortcuts
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    #[allow(clippy::single_match)]
                    match char {
                        'a' => {
                            // select all

                            self.search_cursor.selection_anchor = Some(0);
                            self.search_cursor.position = self.search_text.len();
                        }
                        'c' => {
                            // copy selection to clipboard

                            let Some(selection_anchor) = self.search_cursor.selection_anchor else {
                                return;
                            };

                            let (start, end) = (
                                min(selection_anchor, self.search_cursor.position),
                                max(selection_anchor, self.search_cursor.position),
                            );

                            let selection = &self.search_text[start..end];
                            self.clipboard
                                .set_text(selection.to_string())
                                .expect("failed to copy to clipboard");
                        }
                        'x' => {
                            // cut selection to clipboard

                            let Some(selection_anchor) = self.search_cursor.selection_anchor else {
                                return;
                            };

                            let (start, end) = (
                                min(selection_anchor, self.search_cursor.position),
                                max(selection_anchor, self.search_cursor.position),
                            );

                            let selection = &self.search_text[start..end];
                            self.clipboard
                                .set_text(selection.to_string())
                                .expect("failed to copy to clipboard");

                            self.search_text.drain(start..end);
                            self.search_cursor.selection_anchor = None;
                            self.search_cursor.position = start;
                        }
                        'v' => {
                            // paste from clipboard

                            let pos;
                            if let Some(anchor) = self.search_cursor.selection_anchor {
                                // delete selection if there is one
                                let (start, end) = (
                                    min(anchor, self.search_cursor.position),
                                    max(anchor, self.search_cursor.position),
                                );
                                self.search_text.drain(start..end);
                                self.search_cursor.selection_anchor = None;
                                pos = start;
                            } else {
                                pos = self.search_cursor.position;
                            }

                            let clipboard_text = self
                                .clipboard
                                .get_text()
                                .expect("failed to get clipboard text");
                            self.search_text.insert_str(pos, &clipboard_text);
                            self.search_cursor.position = pos + clipboard_text.len();
                        }
                        _ => {}
                    }
                    return;
                }

                // add char to search text at cursor position
                let pos;
                if let Some(anchor) = self.search_cursor.selection_anchor {
                    // delete selection if there is one
                    let (start, end) = (
                        min(anchor, self.search_cursor.position),
                        max(anchor, self.search_cursor.position),
                    );
                    self.search_text.drain(start..end);
                    self.search_cursor.selection_anchor = None;
                    pos = start;
                } else {
                    pos = self.search_cursor.position;
                }

                self.search_text.insert(pos, char);
                self.search_cursor.position = pos + 1;
            }
            KeyCode::Backspace => {
                if let Some(anchor) = self.search_cursor.selection_anchor {
                    let (start, end) = (
                        min(anchor, self.search_cursor.position),
                        max(anchor, self.search_cursor.position),
                    );
                    self.search_text.drain(start..end);
                    self.search_cursor.selection_anchor = None;
                    self.search_cursor.position = start;
                } else if self.search_cursor.position > 0 {
                    self.search_text.remove(self.search_cursor.position - 1);
                    self.search_cursor.position -= 1;
                }
            }
            KeyCode::Left => {
                if !key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.search_cursor.selection_anchor = None;
                }

                if self.search_cursor.position == 0 {
                    return;
                }

                // if shift is held, ensure we're selecting
                if self.search_cursor.selection_anchor.is_none()
                    && key.modifiers.contains(KeyModifiers::SHIFT)
                {
                    self.search_cursor.selection_anchor = Some(self.search_cursor.position);
                }

                // move cursor left
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.search_cursor.position =
                        get_next_word_bound(&self.search_text, self.search_cursor.position, false);
                } else {
                    self.search_cursor.position -= 1;
                }

                // if we return to the anchor, we're no longer selecting
                if let Some(anchor) = self.search_cursor.selection_anchor
                    && self.search_cursor.position == anchor
                {
                    self.search_cursor.selection_anchor = None;
                }
            }
            KeyCode::Right => {
                if !key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.search_cursor.selection_anchor = None;
                }

                if self.search_cursor.position == self.search_text.len() {
                    return;
                }

                // if shift is held, ensure we're selecting text
                if self.search_cursor.selection_anchor.is_none()
                    && key.modifiers.contains(KeyModifiers::SHIFT)
                {
                    self.search_cursor.selection_anchor = Some(self.search_cursor.position);
                }

                // move cursor right
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.search_cursor.position =
                        get_next_word_bound(&self.search_text, self.search_cursor.position, true);
                } else {
                    self.search_cursor.position += 1;
                }

                // if we return to the anchor, we're no longer selecting text
                if let Some(anchor) = self.search_cursor.selection_anchor
                    && self.search_cursor.position == anchor
                {
                    self.search_cursor.selection_anchor = None;
                }
            }
            _ => {}
        }
    }

    fn maybe_navigate(&mut self, key: KeyEvent) -> bool {
        // determine from current navigation state and key event whether this is a navigation event
        // if so, update the navigation state and return true

        if key.code == KeyCode::Esc {
            self.quit();
            return true;
        }

        false
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
