use sqlx::Result;

use super::{Database, notes::Note};

impl Database {
    pub async fn create_note(&self, text: &str) -> Result<Note> {
        self.notes.create(text).await
    }

    pub async fn get_all_notes(&self) -> Result<Vec<Note>> {
        self.notes.get_all().await
    }
}
