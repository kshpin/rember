use crate::engine::{database::Database, serializer};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio_tungstenite::tungstenite::Utf8Bytes;

#[derive(Debug, Deserialize, Serialize)]
pub struct TestStruct {
    pub field1: String,
    pub field2: String,
}

#[derive(Debug)]
pub enum Message {
    Test(TestStruct),
    Unknown(String),
}

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

    pub async fn handle_message(&self, msg: Utf8Bytes) -> Result<Utf8Bytes, String> {
        let parsed_message = serializer::from_message(msg.clone())?;

        match parsed_message {
            Message::Test(test_struct) => {
                println!("Received test message: {test_struct:?}");
                // Handle test message logic here
            }
            Message::Unknown(msg_type) => {
                println!("Unknown message type: {msg_type}");
            }
        }

        Ok(msg)
    }

    pub fn database(&self) -> &Database {
        &self.database
    }
}
