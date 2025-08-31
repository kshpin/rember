use sqlx::{PgPool, Result};
use uuid::Uuid;

pub use rust_shared::response::Tag;

#[derive(Clone)]
pub struct TagsRepository {
    pool: PgPool,
}

impl TagsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, name: &str) -> Result<Tag> {
        sqlx::query_as!(Tag, "INSERT INTO tags (name) VALUES ($1) RETURNING *", name)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_all(&self) -> Result<Vec<Tag>> {
        sqlx::query_as!(Tag, "SELECT * FROM tags")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Tag>> {
        sqlx::query_as!(Tag, "SELECT * FROM tags WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
    }
}
