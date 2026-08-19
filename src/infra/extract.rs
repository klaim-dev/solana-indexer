use axum::extract::FromRequestParts;

use crate::domain::error::AppError;

pub struct ApiKey(String);

impl ApiKey {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl<S> FromRequestParts<S> for ApiKey
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let header = parts.headers.get("api-key").ok_or(AppError::Unauthorized)?;
        let value = header
            .to_str()
            .map_err(|_| AppError::BadRequest("api-key malformed".to_string()))?;

        Ok(Self(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{HeaderValue, Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::{AppState, Config, app};

    const VALID_PUBKEY: &str = "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes";
    const INVALID_PUBKEY: &str = "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfze0";

    fn test_state() -> AppState {
        let config = Config::from_source(|_| Some("http://test-rpc".to_string())).unwrap();
        AppState {
            config: Arc::new(config),
        }
    }

    #[tokio::test]
    async fn get_account_accepts_api_key_and_valid_pubkey() {
        let request = Request::builder()
            .uri(format!("/accounts/{VALID_PUBKEY}"))
            .header("api-key", "secret123")
            .body(Body::empty())
            .unwrap();

        let response = app(test_state()).oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_account_without_api_key_returns_unauthorized() {
        let request = Request::builder()
            .uri(format!("/accounts/{VALID_PUBKEY}"))
            .body(Body::empty())
            .unwrap();

        let response = app(test_state()).oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn get_account_with_malformed_api_key_returns_bad_request() {
        let malformed_value = HeaderValue::from_bytes(b"abc\xFF").unwrap();
        let request = Request::builder()
            .uri(format!("/accounts/{VALID_PUBKEY}"))
            .header("api-key", malformed_value)
            .body(Body::empty())
            .unwrap();

        let response = app(test_state()).oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn get_account_with_invalid_pubkey_returns_bad_request() {
        let request = Request::builder()
            .uri(format!("/accounts/{INVALID_PUBKEY}"))
            .header("api-key", "secret123")
            .body(Body::empty())
            .unwrap();

        let response = app(test_state()).oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
