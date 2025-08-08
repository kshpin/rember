use crate::engine::{database::Database, serializer};
use sqlx::PgPool;
use tokio_tungstenite::tungstenite::Utf8Bytes;

#[derive(Clone)]
pub struct Engine {
    database: Database,
}

impl Engine {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn with_database_url(db_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = PgPool::connect(db_url).await?;
        let database = Database::with_pool(db_pool);
        Ok(Self::new(database))
    }

    pub async fn handle_message(&self, msg: Utf8Bytes) -> Result<Utf8Bytes, ()> {
        serializer::handle_message(msg).await
    }

    pub fn database(&self) -> &Database {
        &self.database
    }
}
