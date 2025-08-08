use chrono::{DateTime, Utc};
use sqlx::{PgPool, Result};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub struct NoteRepository {
    pool: PgPool,
}

impl NoteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, title: &str, body: &str) -> Result<Note> {
        /* sqlx::query_as!(
            Note,
            "INSERT INTO notes (title, body) VALUES ($1, $2) RETURNING *",
            title,
            body
        )
        .fetch_one(&self.pool)
        .await */
        unimplemented!()
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Note>> {
        /* sqlx::query_as!(Note, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_optional(&self.pool)
        .await */
        unimplemented!()
    }
}
