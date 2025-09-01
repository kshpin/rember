use crossterm::event::{KeyCode, KeyEvent};

use rust_shared::request;

use crate::App;

impl App {
    pub async fn on_key_event(&mut self, key: KeyEvent) {
        if self.maybe_navigate(key) {
            return;
        }

        self.search.search_box.handle_key_event(key);

        let (tags, search_text) = parse_search_text(&self.search.search_box.text_box.text);
        self.search.parsed_tags = tags.clone();
        self.search.parsed_search_text = search_text.clone();

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
                        search_text,
                        tags: Some(tags),
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
