#[derive(Debug)]
pub struct Config {
    #[expect(dead_code, reason = "reserved for RPC-backed handlers")]
    pub rpc_url: String,
}

impl Config {
    pub fn from_source<F>(get: F) -> Result<Self, ConfigError>
    where
        F: Fn(&str) -> Option<String>,
    {
        let rpc_url = get("SOLANA_RPC_URL").ok_or(ConfigError::MissingRpcUrl)?;

        if rpc_url.trim().is_empty() {
            return Err(ConfigError::InvalidRpcUrl);
        }

        let config = Config { rpc_url };
        Ok(config)
    }

    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_source(|key| std::env::var(key).ok())
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ConfigError {
    #[error("missing rpc url")]
    MissingRpcUrl,
    #[error("invalid rpc url")]
    InvalidRpcUrl,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_source_missing_rpc_url() {
        let err = Config::from_source(|_| None).unwrap_err();
        assert_eq!(err, ConfigError::MissingRpcUrl);
    }
}
