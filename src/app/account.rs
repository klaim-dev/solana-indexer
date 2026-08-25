use std::str::FromStr;

use solana_pubkey::Pubkey;

use crate::domain::error::AppError;

pub fn parse_pubkey(input: &str) -> Result<Pubkey, AppError> {
    Pubkey::from_str(input).map_err(|_| AppError::BadRequest("invalid pubkey".to_string()))
}

#[cfg(test)]
mod tests {

    use super::*;

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
