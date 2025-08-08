use chrono::NaiveDateTime;
use sqlx::{PgPool, Result};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Note {
    pub id: Uuid,
    pub text: String,
    pub created_at: NaiveDateTime,
}

#[derive(Clone)]
pub struct NoteRepository {
    pool: PgPool,
}

impl NoteRepository {
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
        /* sqlx::query_as!(Note, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_optional(&self.pool)
        .await */
        unimplemented!()
    }
}
