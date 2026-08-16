use axum::{Router, routing::get};

use crate::domain::error::AppError::{self};

mod domain;

pub async fn healthz() -> Result<(), AppError> {
    Ok(())
}

pub async fn readyz() -> Result<(), AppError> {
    Ok(())
}

pub async fn status() -> Result<(), AppError> {
    Ok(())
}

#[tokio::main]
async fn main() {
    let _app: Router<()> = Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/status", get(status));
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
}
