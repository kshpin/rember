use crate::engine::database::Database;
use sqlx::PgPool;

use rust_shared::{request, response};

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
        sqlx::migrate!().run(&db_pool).await?;
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
    pub async fn handle_message(&self, msg: request::Message) -> response::Message {
        match msg {
            request::Message::CreateNote(create_note) => {
                match self.database.create_note(&create_note.text).await {
                    Ok(note) => response::Message::Notes(vec![note]),
                    Err(e) => response::Message::Unknown(e.to_string()),
                }
            }
            request::Message::GetNotes(_get_notes) => match self.database.get_all_notes().await {
                Ok(notes) => response::Message::Notes(notes),
                Err(e) => response::Message::Unknown(e.to_string()),
            },
            request::Message::GetNotesFiltered(request::GetNotesFiltered {
                search_text,
                tags,
                limit,
                offset,
            }) => match self
                .database
                .get_notes_filtered(search_text, tags, limit, offset)
                .await
            {
                Ok(notes) => response::Message::Notes(notes),
                Err(e) => response::Message::Unknown(e.to_string()),
            },
            request::Message::Test(test_struct) => {
                println!("Received test message: {test_struct:?}");
                response::Message::Unknown("test message received".to_string())
            }
            request::Message::Unknown(msg_type) => {
                println!("Unknown message type: {msg_type}");
                response::Message::Unknown(msg_type)
            }
        }
    }

    pub fn _database(&self) -> &Database {
        &self.database
    }
}
