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

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let engine = engine::core::Engine::with_database_url(&db_url)
        .await
        .expect("Failed to initialize engine with database");

    let addr = "0.0.0.0:3210";
    server::listener::start(addr, move |msg| {
        let engine = engine.clone();
        async move { engine.handle_message(msg).await }
    })
    .await;
}
