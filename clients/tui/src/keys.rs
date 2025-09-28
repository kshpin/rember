use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use rust_shared::request;

use crate::App;
use crate::focus::Focus;

impl App {
    pub async fn on_key_event(&mut self, key: KeyEvent) {
        if self.maybe_exit(key) || self.focus.maybe_update(key) {
            return;
        }

        if key.code == KeyCode::Up || key.code == KeyCode::Down {
            self.search_results
                .move_selection(key.code == KeyCode::Down);
            return;
        }

        match self.focus {
            Focus::Search => self.search.handle_key_event(key),
            Focus::NewNote => self.new_note.handle_key_event(key),
        }

        // if any displayable key is pressed, request the notes
        // this includes all chars, backspace, delete, etc.
        let should_request = self.focus == Focus::Search
            && (key.code.is_backspace()
                || key.code.is_delete()
                || matches!(key.code, KeyCode::Char(_)));

        if should_request {
            self.websocket_client
                .send(request::Message::GetNotesFiltered(
                    request::GetNotesFiltered {
                        search_text: self.search.parsed_search_text.clone(),
                        tags: Some(self.search.parsed_tags.clone()),
                        limit: Some(10),
                        offset: Some(0),
                    },
                ))
                .await
                .expect("msg");
        }
    }

    fn maybe_exit(&mut self, key: KeyEvent) -> bool {
        if self.focus == Focus::Search
            && key.modifiers.contains(KeyModifiers::CONTROL)
            && key.code == KeyCode::Char('q')
        {
            self.quit();
            return true;
        }

        false
    }
}
