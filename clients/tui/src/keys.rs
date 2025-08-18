use crossterm::event::{KeyCode, KeyEvent};

use crate::App;

impl App {
    pub async fn on_key_event(&mut self, key: KeyEvent) {
        if self.maybe_navigate(key) {
            return;
        }

        match key.code {
            KeyCode::Char(key) => {
                // add key to search text at cursor position
                self.search_text.insert(self.search_cursor_position, key);
                self.search_cursor_position += 1;
            }
            KeyCode::Backspace => {
                if self.search_cursor_position > 0 {
                    self.search_text.remove(self.search_cursor_position - 1);
                    self.search_cursor_position -= 1;
                }
            }
            KeyCode::Left => {
                if self.search_cursor_position > 0 {
                    self.search_cursor_position -= 1;
                }
            }
            KeyCode::Right => {
                if self.search_cursor_position < self.search_text.len() {
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
