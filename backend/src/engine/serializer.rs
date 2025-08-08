use crate::engine::core::Message;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Utf8Bytes;

fn extract_data<T: DeserializeOwned>(json: Value) -> Result<T, String> {
    T::deserialize(&json["data"]).map_err(|e| format!("Failed to deserialize data field: {e}"))
}

/// Extracts the data from the json message and deserializes it into a Message enum
/// based on the type of the message.
///
/// ```json
/// {
///     "type": "message_type",
///     "data": {
///         // actual data, whose format is determined by the message type
///     }
/// }
/// ```
pub fn from_message(msg: Utf8Bytes) -> Result<Message, String> {
    let json: Value =
        serde_json::from_str(&msg).map_err(|e| format!("Failed to parse JSON: {e}"))?;

    match json["type"].as_str() {
        Some("test") => {
            let test_struct = extract_data(json)?;
            Ok(Message::Test(test_struct))
        }
        Some(class) => Ok(Message::Unknown(class.to_string())),
        None => Err("Missing type field in message".to_string()),
    }
}
