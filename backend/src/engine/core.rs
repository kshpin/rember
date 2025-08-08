use crate::engine::database::Database;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio_tungstenite::tungstenite::Utf8Bytes;

#[derive(Serialize, Deserialize, Debug)]
pub struct TestStruct {
    pub field1: String,
    pub field2: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
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
        sqlx::migrate!("src/engine/database/migrations")
            .run(&db_pool)
            .await?;
        let database = Database::with_pool(db_pool);
        Ok(Self::new(database))
    }

    /// Expects a json message with the following format:
    ///
    /// ```json
    /// {
    ///     "type": "message_type",
    ///     "data": {
    ///         // actual data, whose format is determined by the message type
    ///     }
    /// }
    /// ```
    pub async fn handle_message(&self, msg: Utf8Bytes) -> Result<Utf8Bytes, String> {
        let parsed_message = serde_json::from_str(&msg).map_err(|e| e.to_string())?;

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
