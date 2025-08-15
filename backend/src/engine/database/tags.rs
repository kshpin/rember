use sqlx::{PgPool, Result};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}

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

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Tag>> {
        sqlx::query_as!(Tag, "SELECT * FROM tags WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
    }
}
