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
    pub async fn handle_message(&self, msg: shared::request::Message) -> shared::response::Message {
        match msg {
            shared::request::Message::CreateNote(create_note) => {
                match self.database.create_note(&create_note.text).await {
                    Ok(note) => shared::response::Message::Unknown("note created".to_string()),
                    Err(e) => shared::response::Message::Unknown(e.to_string()),
                }
            }
            shared::request::Message::GetNotes(_get_notes) => {
                match self.database.get_all_notes().await {
                    Ok(notes) => shared::response::Message::Unknown("notes fetched".to_string()),
                    Err(e) => shared::response::Message::Unknown(e.to_string()),
                }
            }
            shared::request::Message::GetNotesFiltered(shared::request::GetNotesFiltered {
                search_text,
                tags,
                limit,
                offset,
            }) => match self
                .database
                .get_notes_filtered(search_text, tags, limit, offset)
                .await
            {
                Ok(notes) => shared::response::Message::Unknown("notes fetched".to_string()),
                Err(e) => shared::response::Message::Unknown(e.to_string()),
            },
            shared::request::Message::Test(test_struct) => {
                println!("Received test message: {test_struct:?}");
                shared::response::Message::Unknown("test message received".to_string())
            }
            shared::request::Message::Unknown(msg_type) => {
                println!("Unknown message type: {msg_type}");
                shared::response::Message::Unknown(msg_type)
            }
        }
    }

    pub fn _database(&self) -> &Database {
        &self.database
    }
}
