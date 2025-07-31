use tokio_tungstenite::tungstenite::Utf8Bytes;

pub async fn handle_message(msg: Utf8Bytes) -> Result<Utf8Bytes, ()> {
    Ok(msg)
}
