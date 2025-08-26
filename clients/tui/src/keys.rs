use crossterm::event::{KeyCode, KeyEvent};

use rust_shared as shared;

use crate::App;

impl App {
    pub async fn on_key_event(&mut self, key: KeyEvent) {
        if self.maybe_navigate(key) {
            return;
        }

        self.search_box.handle_key_event(key);
        self.websocket_client
            .send(shared::request::Message::GetNotesFiltered(
                shared::request::GetNotesFiltered {
                    search_text: None,
                    tags: vec![],
                    limit: Some(10),
                    offset: Some(0),
                },
            ))
            .await
            .expect("msg");
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
