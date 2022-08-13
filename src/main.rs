use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{app, config};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // NOTE: Logging is controlled via the RUST_LOG environment variable.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = config::get_configuration();
    let connection = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    app::run(listener, connection)?.await
}
