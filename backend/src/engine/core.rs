use crate::engine::database::Database;
use sqlx::PgPool;
use tokio_tungstenite::tungstenite::Utf8Bytes;

use rust_shared as shared;

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
            shared::Message::CreateNote(create_note) => {
                match self.database.create_note(&create_note.text).await {
                    Ok(note) => serde_json::to_string(&note)
                        .map(|json| json.into())
                        .map_err(|e| e.to_string()),
                    Err(e) => Err(e.to_string()),
                }
            }
            shared::Message::GetNotes(_get_notes) => match self.database.get_all_notes().await {
                Ok(notes) => serde_json::to_string(&notes)
                    .map(|json| json.into())
                    .map_err(|e| e.to_string()),
                Err(e) => Err(e.to_string()),
            },
            shared::Message::GetNotesFiltered(shared::GetNotesFiltered {
                search_text,
                tags,
                limit,
                offset,
            }) => match self
                .database
                .get_notes_filtered(search_text, tags, limit, offset)
                .await
            {
                Ok(notes) => serde_json::to_string(&notes)
                    .map(|json| json.into())
                    .map_err(|e| e.to_string()),
                Err(e) => Err(e.to_string()),
            },
            shared::Message::Test(test_struct) => {
                println!("Received test message: {test_struct:?}");
                Ok("".into())
            }
            shared::Message::Unknown(msg_type) => {
                println!("Unknown message type: {msg_type}");
                Ok(msg)
            }
        }
    }

    pub fn _database(&self) -> &Database {
        &self.database
    }
}
