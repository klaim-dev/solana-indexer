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
    #[error{"invalid solana pubkey: {input_pubkey}"}]
    InvalidAccountPubkey { input_pubkey: String },
    #[error{"the account already exists: {pubkey}"}]
    AccountAlreadyExists { pubkey: String },
    #[error{"the account does not exist yet: {input_pubkey}"}]
    AccountNotExist { input_pubkey: String },
}
