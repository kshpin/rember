use chrono::NaiveDateTime;
use sqlx::{PgPool, Result};
use uuid::Uuid;

pub use rust_shared::response::Note;

// add import for tags
use super::tags::Tag;

#[allow(dead_code)]
pub struct NoteDate {
    pub id: Uuid,
    pub note_id: Uuid,
    pub label: String,
    pub date: NaiveDateTime,
}

#[allow(dead_code)]
pub struct NoteWithDetails {
    pub note: Note,
    pub tags: Vec<Tag>,
    pub note_dates: Vec<NoteDate>,
    pub note_links: Vec<Note>,
}

#[derive(Clone)]
pub struct NotesRepository {
    pool: PgPool,
}

impl NotesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, text: &str, tags: &Vec<String>) -> Result<Note> {
        let note = sqlx::query_as!(
            Note,
            "INSERT INTO notes (text) VALUES ($1) RETURNING *",
            text
        )
        .fetch_one(&self.pool)
        .await?;

        for tag in tags {
            let tag = sqlx::query_as!(Tag, "SELECT * FROM tags WHERE name = $1", tag)
                .fetch_one(&self.pool)
                .await?;

            sqlx::query!(
                "INSERT INTO note_tags (note_id, tag_id) VALUES ($1, $2)",
                note.id,
                tag.id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(note)
    }

    pub async fn get_all(&self) -> Result<Vec<Note>> {
        sqlx::query_as!(Note, "SELECT * FROM notes")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_filtered(
        &self,
        search_text: Option<String>,
        tags: Vec<String>,
        _limit: Option<u32>,
        _offset: Option<u32>,
    ) -> Result<Vec<Note>> {
        sqlx::query_file_as!(
            Note,
            "queries/note_list_search.sql",
            search_text.unwrap_or_default(),
            &tags,
        )
        .fetch_all(&self.pool)
        .await
    }
}
