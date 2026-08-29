use std::{collections::HashMap, str::FromStr};

use solana_pubkey::Pubkey;

use crate::{
    app::state::AppState,
    domain::{accounts::SolanaAccount, error::AppError},
};

pub fn parse_pubkey(input: &str) -> Result<Pubkey, AppError> {
    Pubkey::from_str(input).map_err(|_| AppError::InvalidAccountPubkey {
        input_pubkey: input.to_string(),
    })
}

async fn create(state: &AppState, account: SolanaAccount) -> Result<(), AppError> {
    let mut accounts_map = state.accounts.write().await;
    create_in_map(&mut accounts_map, account)
}

fn create_in_map(
    accounts: &mut HashMap<Pubkey, SolanaAccount>,
    account: SolanaAccount,
) -> Result<(), AppError> {
    let pubkey = parse_pubkey(&account.pubkey)?;
    if accounts.contains_key(&pubkey) {
        return Err(AppError::AccountAlreadyExists {
            pubkey: account.pubkey.clone(),
        });
    };

    accounts.insert(pubkey, account);
    Ok(())
}

async fn get(state: &AppState, pubkey: String) -> Result<SolanaAccount, AppError> {
    let accounts_map = state.accounts.read().await;
    get_from_map(&accounts_map, pubkey)
}

fn get_from_map(
    accounts: &HashMap<Pubkey, SolanaAccount>,
    pubkey: String,
) -> Result<SolanaAccount, AppError> {
    let pubkey = parse_pubkey(&pubkey)?;
    let account = accounts
        .get(&pubkey)
        .ok_or(AppError::AccountNotExist {
            input_pubkey: pubkey.to_string(),
        })?
        .clone();
    Ok(account)
}

async fn list(state: &AppState) -> Result<Vec<SolanaAccount>, AppError> {
    let accounts_map = state.accounts.read().await;
    list_from_map(&accounts_map)
}
fn list_from_map(
    accounts: &HashMap<Pubkey, SolanaAccount>,
) -> Result<Vec<SolanaAccount>, AppError> {
    let list = accounts.values().cloned().collect::<Vec<SolanaAccount>>();
    Ok(list)
}

#[cfg(test)]
mod tests {

    use crate::config::Config;

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
        assert_eq!(err, AppError::InvalidAccountPubkey { input_pubkey: key.to_string() });
    }

    #[tokio::test]
    async fn test_create_happy_path() {
        let config = Config::from_source(|_| Some("http://test-rpc".to_string())).unwrap();
        let state = AppState::new(config);
        let account_pubkey =
            Pubkey::from_str("6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes").unwrap();
        let account = SolanaAccount {
            pubkey: "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes".to_string(),
            owner: "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes".to_string(),
            lamports: 1_000_000_000,
            slot_update: 941,
        };

        create(&state, account).await.unwrap();

        let accounts = state.accounts.read().await;
        let (pubkey, _) = accounts.get_key_value(&account_pubkey).unwrap();
        assert_eq!(account_pubkey, *pubkey);
    }

    #[tokio::test]
    async fn test_get_happy_path() {
        let config = Config::from_source(|_| Some("http://test-rpc".to_string())).unwrap();
        let state = AppState::new(config);
        let account_pubkey =
            Pubkey::from_str("6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes").unwrap();
        let account = SolanaAccount {
            pubkey: "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes".to_string(),
            owner: "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes".to_string(),
            lamports: 1_000_000_000,
            slot_update: 941,
        };

        create(&state, account.clone()).await.unwrap();
        let created_account = get(&state, account.pubkey).await.unwrap();
        assert_eq!(created_account.pubkey, account_pubkey.to_string());
        assert_eq!(created_account.slot_update, account.slot_update);
        assert_eq!(created_account.owner, account.owner);
        assert_eq!(created_account.lamports, account.lamports);
    }

    #[tokio::test]
    async fn test_get_account_not_exist() {
        {
            let config = Config::from_source(|_| Some("http://test-rpc".to_string())).unwrap();
            let state = AppState::new(config);
            let account_pubkey = "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes".to_string();
            let err = get(&state, account_pubkey.clone()).await.unwrap_err();
            assert_eq!(
                err,
                AppError::AccountNotExist {
                    input_pubkey: account_pubkey
                }
            );
        }
    }

    #[tokio::test]
    async fn test_list_happy_path() {
        let config = Config::from_source(|_| Some("http://test-rpc".to_string())).unwrap();
        let state = AppState::new(config);
        let account1 = SolanaAccount {
            pubkey: "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes".to_string(),
            owner: "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes".to_string(),
            lamports: 1_000_000_000,
            slot_update: 941,
        };

        let account2 = SolanaAccount {
            pubkey: "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzea".to_string(),
            owner: "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzea".to_string(),
            lamports: 1_000_000_000,
            slot_update: 941,
        };

        create(&state, account1.clone()).await.unwrap();
        create(&state, account2.clone()).await.unwrap();

        let list = list(&state).await.unwrap();
        assert_eq!(list.len(), 2);

        assert!(list.contains(&account1));
        assert!(list.contains(&account2));
    }

    #[tokio::test]
    async fn test_create_duplicate_account() {
        let config = Config::from_source(|_| Some("http://test-rpc".to_string())).unwrap();
        let state = AppState::new(config);
        let account = SolanaAccount {
            pubkey: "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes".to_string(),
            owner: "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes".to_string(),
            lamports: 1_000_000_000,
            slot_update: 941,
        };

        create(&state, account.clone()).await.unwrap();
        let err = create(&state, account.clone()).await.unwrap_err();

        assert_eq!(
            err,
            AppError::AccountAlreadyExists {
                pubkey: account.pubkey
            }
        );

        let accounts = state.accounts.read().await;
        assert_eq!(accounts.len(), 1);
    }
}
