use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::App;

impl App {
    pub async fn on_key_event(&mut self, key: KeyEvent) {
        if self.maybe_navigate(key) {
            return;
        }

        match key.code {
            KeyCode::Char(char) => {
                // add char to search text at cursor position
                self.search_text.insert(self.search_cursor_position, char);
                self.search_cursor_position += 1;
            }
            KeyCode::Backspace => {
                if self.search_cursor_position > 0 {
                    self.search_text.remove(self.search_cursor_position - 1);
                    self.search_cursor_position -= 1;
                }
            }
            KeyCode::Left => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.search_cursor_position =
                        get_next_word_bound(&self.search_text, self.search_cursor_position, false);
                } else if self.search_cursor_position > 0 {
                    self.search_cursor_position -= 1;
                }
            }
            KeyCode::Right => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.search_cursor_position =
                        get_next_word_bound(&self.search_text, self.search_cursor_position, true);
                } else if self.search_cursor_position < self.search_text.len() {
                    self.search_cursor_position += 1;
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
