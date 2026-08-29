use axum::{Router, extract::Path, routing::get};
use axum::{http::StatusCode, response::IntoResponse};

use crate::{
    AppState, app::account::parse_pubkey, domain::error::AppError, infra::extract::ApiKey,
};

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

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/status", get(status))
        .route("/accounts/{pubkey}", get(get_account))
        .with_state(state)
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest(..) => StatusCode::BAD_REQUEST,
            AppError::Internal(..) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::InvalidAccountPubkey { .. } => StatusCode::BAD_REQUEST,
            AppError::AccountAlreadyExists { .. } => StatusCode::BAD_REQUEST,
            AppError::AccountNotExist { .. } => StatusCode::BAD_REQUEST,
        };

        let message = self.to_string();

        (status_code, message).into_response()
    }
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
    fn test_app_error_not_found() {
        let err = AppError::NotFound;
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_app_error_bad_request() {
        let err = AppError::BadRequest("bad_request".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_app_error_internal() {
        let err = AppError::Internal("internal".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
