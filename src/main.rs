use std::str::FromStr;

use axum::{Router, extract::Path, routing::get};
use solana_pubkey::Pubkey;

use crate::{
    domain::error::AppError::{self},
    infra::extract::ApiKey,
};

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

pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/status", get(status))
        .route("/accounts/{pubkey}", get(get_account))
}

#[tokio::main]
async fn main() {
    let _app = app();
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
