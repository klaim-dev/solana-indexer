use tokio::net::TcpListener;

use crate::{
    app::state::AppState,
    config::{Config, ConfigError},
    infra::http::app,
};

mod app;
mod config;
mod domain;
mod infra;

#[derive(Debug, thiserror::Error)]
pub enum StartupError {
    #[error("config error: {0}")]
    Config(#[from] ConfigError),
    #[error("io error {0}")]
    Io(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), StartupError> {
    let _ = dotenvy::dotenv();
    run().await?;
    Ok(())
}

async fn run() -> Result<(), StartupError> {
    let config = Config::from_env()?;
    let state = AppState::new(config);
    let app = app(state);
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
