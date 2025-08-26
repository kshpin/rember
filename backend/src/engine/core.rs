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
    pub async fn handle_message(&self, msg: shared::MessageRequest) -> shared::MessageResponse {
        match msg {
            shared::MessageRequest::CreateNote(create_note) => {
                match self.database.create_note(&create_note.text).await {
                    Ok(note) => shared::MessageResponse::Unknown("note created".to_string()),
                    Err(e) => shared::MessageResponse::Unknown(e.to_string()),
                }
            }
            shared::MessageRequest::GetNotes(_get_notes) => {
                match self.database.get_all_notes().await {
                    Ok(notes) => shared::MessageResponse::Unknown("notes fetched".to_string()),
                    Err(e) => shared::MessageResponse::Unknown(e.to_string()),
                }
            }
            shared::MessageRequest::GetNotesFiltered(shared::GetNotesFiltered {
                search_text,
                tags,
                limit,
                offset,
            }) => match self
                .database
                .get_notes_filtered(search_text, tags, limit, offset)
                .await
            {
                Ok(notes) => shared::MessageResponse::Unknown("notes fetched".to_string()),
                Err(e) => shared::MessageResponse::Unknown(e.to_string()),
            },
            shared::MessageRequest::Test(test_struct) => {
                println!("Received test message: {test_struct:?}");
                shared::MessageResponse::Unknown("test message received".to_string())
            }
            shared::MessageRequest::Unknown(msg_type) => {
                println!("Unknown message type: {msg_type}");
                shared::MessageResponse::Unknown(msg_type)
            }
        }
    }

    pub fn _database(&self) -> &Database {
        &self.database
    }
}
