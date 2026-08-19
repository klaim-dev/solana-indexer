use std::{str::FromStr, sync::Arc};

use axum::{Router, extract::Path, routing::get};
use solana_pubkey::Pubkey;
use tokio::net::TcpListener;

use crate::{
    config::{Config, ConfigError},
    domain::error::AppError::{self},
    infra::extract::ApiKey,
};

mod config;
mod domain;
mod infra;

pub async fn healthz() -> Result<(), AppError> {
    Ok(())
}

pub async fn readyz() -> Result<(), AppError> {
    Ok(())
}

pub async fn status() -> Result<(), AppError> {
    Ok(())
}

async fn get_account(api_key: ApiKey, Path(input): Path<String>) -> Result<(), AppError> {
    let _api_key = api_key.into_inner();
    let _pubkey = parse_pubkey(&input)?;
    Ok(())
}

fn parse_pubkey(input: &str) -> Result<Pubkey, AppError> {
    Pubkey::from_str(input).map_err(|_| AppError::BadRequest("invalid pubkey".to_string()))
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/status", get(status))
        .route("/accounts/{pubkey}", get(get_account))
        .with_state(state)
}

#[derive(Debug, Clone)]
pub struct AppState {
    #[expect(dead_code, reason = "reserved for RPC-backed handlers")]
    config: Arc<Config>,
}

impl AppState {
    fn new(config: Config) -> AppState {
        AppState {
            config: Arc::new(config),
        }
    }
}
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

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_healthz() {
        let response = healthz().await;
        assert_eq!(response, Ok(()))
    }

    #[tokio::test]
    async fn test_readyz() {
        let response = readyz().await;
        assert_eq!(response, Ok(()))
    }

    #[tokio::test]
    async fn test_status() {
        let response = status().await;
        assert_eq!(response, Ok(()))
    }

    #[test]
    fn test_parse_pubkey() {
        let key = "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes";
        let res = parse_pubkey(key).unwrap();
        assert_eq!(res.to_string(), key);
    }

    #[test]
    fn test_parse_pubkey_rejects_non_base58_character() {
        let key = "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfze0";
        let err = parse_pubkey(key).unwrap_err();
        assert_eq!(err, AppError::BadRequest("invalid pubkey".to_string()));
    }
}
