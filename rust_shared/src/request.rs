use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestStruct {
    pub field1: String,
    pub field2: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNote {
    pub text: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTag {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNotes {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNotesFiltered {
    pub search_text: Option<String>,
    pub tags: Option<Vec<String>>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum Message {
    CreateNote(CreateNote),
    CreateTag(CreateTag),
    GetNotes(GetNotes),
    GetTags,
    GetNotesFiltered(GetNotesFiltered),
    Test(TestStruct),
    Unknown(String),
}
