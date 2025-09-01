use crossterm::event::{KeyCode, KeyEvent};

use rust_shared::request;

use crate::App;

impl App {
    pub async fn on_key_event(&mut self, key: KeyEvent) {
        if self.maybe_navigate(key) {
            return;
        }

        self.search.handle_key_event(key);

        if false {
            self.websocket_client
                .send(request::Message::GetNotes(request::GetNotes {
                    limit: Some(10),
                    offset: Some(0),
                }))
                .await
                .expect("msg");
        } else {
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
