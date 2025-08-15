use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Result};
use uuid::Uuid;

// add import for tags
use super::tags::Tag;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: Uuid,
    pub text: String,
    pub created_at: NaiveDateTime,
}

pub struct NoteDate {
    pub id: Uuid,
    pub note_id: Uuid,
    pub label: String,
    pub date: NaiveDateTime,
}

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

    pub async fn create(&self, text: &str) -> Result<Note> {
        sqlx::query_as!(
            Note,
            "INSERT INTO notes (text) VALUES ($1) RETURNING *",
            text
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Note>> {
        sqlx::query_as!(Note, "SELECT * FROM notes WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_all(&self) -> Result<Vec<Note>> {
        sqlx::query_as!(Note, "SELECT * FROM notes")
            .fetch_all(&self.pool)
            .await
    }
}
