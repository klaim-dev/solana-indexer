use std::sync::Arc;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    #[expect(dead_code, reason = "reserved for RPC-backed handlers")]
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(config: Config) -> AppState {
        AppState {
            config: Arc::new(config),
        }
    }
}
