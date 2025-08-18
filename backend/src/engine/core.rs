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
pub struct CreateNote {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNotes {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNotesFiltered {
    pub search_text: Option<String>,
    pub tags: Vec<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum Message {
    CreateNote(CreateNote),
    GetNotes(GetNotes),
    GetNotesFiltered(GetNotesFiltered),
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
            Message::CreateNote(create_note) => {
                match self.database.create_note(&create_note.text).await {
                    Ok(note) => serde_json::to_string(&note)
                        .map(|json| json.into())
                        .map_err(|e| e.to_string()),
                    Err(e) => Err(e.to_string()),
                }
            }
            Message::GetNotes(_get_notes) => match self.database.get_all_notes().await {
                Ok(notes) => serde_json::to_string(&notes)
                    .map(|json| json.into())
                    .map_err(|e| e.to_string()),
                Err(e) => Err(e.to_string()),
            },
            Message::GetNotesFiltered(GetNotesFiltered {
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
            Message::Test(test_struct) => {
                println!("Received test message: {test_struct:?}");
                Ok("".into())
            }
            Message::Unknown(msg_type) => {
                println!("Unknown message type: {msg_type}");
                Ok(msg)
            }
        }
    }

    pub fn _database(&self) -> &Database {
        &self.database
    }
}
