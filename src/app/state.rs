use std::{collections::HashMap, sync::Arc};

use solana_pubkey::Pubkey;
use tokio::sync::RwLock;

use crate::{config::Config, domain::accounts::SolanaAccount};

#[derive(Debug, Clone)]
pub struct AppState {
    #[expect(dead_code, reason = "reserved for RPC-backed handlers")]
    pub config: Arc<Config>,
    pub accounts: Arc<RwLock<HashMap<Pubkey, SolanaAccount>>>,
}

impl AppState {
    pub fn new(config: Config) -> AppState {
        AppState {
            config: Arc::new(config),
            accounts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
