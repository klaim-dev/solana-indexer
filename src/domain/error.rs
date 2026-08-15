use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error{"not found"}]
    NotFound,
    #[error{"{0}"}]
    BadRequest(String),
    #[error{"{0}"}]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest(..) => StatusCode::BAD_REQUEST,
            AppError::Internal(..) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match self {
            AppError::NotFound => "not found".to_string(),
            AppError::BadRequest(message) => message,
            AppError::Internal(_) => "internal error".to_string(),
        };

        (status_code, message).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
