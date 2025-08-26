use serde::{Deserialize, Serialize};

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
