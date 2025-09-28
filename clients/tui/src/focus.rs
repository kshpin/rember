use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    Search,
    NewNote,
}

impl Default for Focus {
    fn default() -> Self {
        Self::Search
    }
}

impl Focus {
    pub fn maybe_update(&mut self, key: KeyEvent) -> bool {
        let mut updated = false;

        match self {
            Self::Search => {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('n') {
                    *self = Self::NewNote;
                    updated = true;
                }
            }
            Self::NewNote => {
                if key.code == KeyCode::Esc {
                    *self = Self::Search;
                    updated = true;
                }
            }
        }

        updated
    }
}
