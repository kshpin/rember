use sqlx::Result;

use super::{Database, notes::Note, tags::Tag};

impl Database {
    pub async fn create_note(&self, text: &str, tags: &Vec<String>) -> Result<Note> {
        self.notes.create(text, tags).await
    }

    pub async fn create_tag(&self, name: &str) -> Result<Tag> {
        self.tags.create(name).await
    }

    pub async fn get_all_notes(&self) -> Result<Vec<Note>> {
        self.notes.get_all().await
    }

    pub async fn get_all_tags(&self) -> Result<Vec<Tag>> {
        self.tags.get_all().await
    }

    pub async fn get_notes_filtered(
        &self,
        search_text: Option<String>,
        tags: Option<Vec<String>>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Note>> {
        self.notes
            .get_filtered(search_text, tags.unwrap_or_default(), limit, offset)
            .await
    }
}
