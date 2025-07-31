use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Utf8Bytes;

#[derive(Debug, Deserialize, Serialize)]
struct TestStruct {
    field1: String,
    field2: String,
}

fn extract_data<T: DeserializeOwned>(json: Value) -> T {
    T::deserialize(&json["data"]).unwrap()
}

/// Extracts the data from the json message and deserializes it into a type
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
pub async fn handle_message(msg: Utf8Bytes) -> Result<Utf8Bytes, ()> {
    let json: Value = serde_json::from_str(&msg).unwrap();

    match json["type"].as_str() {
        None => {
            println!("unknown class: {json:?}");
        }
        Some("test") => {
            let test_struct: TestStruct = extract_data(json);
            println!("test_struct: {test_struct:?}");
        }
        Some(class) => {
            println!("unknown class: {class}");
        }
    }
    Ok(msg)
}
