mod engine;
mod server;

#[tokio::main]
async fn main() {
    // only for `cargo run` development
    // in production, the environment variables are set by the compose file
    dotenvy::dotenv().ok();

    let log_level = std::env::var("LOG_LEVEL")
        .unwrap_or("info".to_string())
        .parse::<tracing::Level>()
        .unwrap_or(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .init();

    let addr = "0.0.0.0:3210";
    server::listener::start(addr, engine::serializer::handle_message).await;
}
