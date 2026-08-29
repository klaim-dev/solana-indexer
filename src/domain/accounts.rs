#[derive(Debug, Clone, PartialEq)]
pub struct SolanaAccount {
    pub pubkey: String,
    pub owner: String,
    pub lamports: u64,
    pub slot_update: u32,
}
