#[derive(Debug, thiserror::Error, PartialEq)]
pub enum AppError {
    #[error{"not found"}]
    NotFound,
    #[error{"{0}"}]
    BadRequest(String),
    #[error{"{0}"}]
    Internal(String),
    #[error{"unauthorized"}]
    Unauthorized,
}
