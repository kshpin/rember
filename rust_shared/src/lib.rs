pub mod request;
pub mod response;

use serde::{Serialize, de::DeserializeOwned};

pub fn serialize<T: Serialize>(value: T, pretty: bool) -> String {
    let serialize = if pretty {
        serde_json::to_string_pretty
    } else {
        serde_json::to_string
    };

    serialize(&value).expect("failed to serialize")
}

pub fn deserialize<T: DeserializeOwned>(value: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(value)
}
