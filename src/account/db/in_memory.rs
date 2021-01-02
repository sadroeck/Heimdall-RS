use std::collections::{hash_map::Entry, HashMap};

use async_std::sync::RwLock;
use tracing::{debug, info, warn};

use crate::account::mmo_account::MmoAccount;

use super::{AccountDB, AccountId, DBError, UserId};

pub struct InMemoryAccountDB {
    verbose: bool,
    accounts: RwLock<HashMap<AccountId, MmoAccount>>,
}

impl InMemoryAccountDB {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            accounts: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl AccountDB for InMemoryAccountDB {
    async fn init() -> super::DBResult<()> {
        info!("Initializing InMemory account DB");
        Ok(())
    }

    async fn create_account(&self) -> super::DBResult<MmoAccount> {
        let mut retries: i32 = 10;
        loop {
            let account_id = fastrand::u32(..);
            if self.verbose {
                debug!("Creating new account {}", account_id);
            }
            match self.accounts.write().await.entry(account_id) {
                Entry::Vacant(entry) => {
                    let account = MmoAccount::new(account_id);
                    entry.insert(account.clone());
                    return Ok(MmoAccount::new(account_id));
                }
                Entry::Occupied(_) => {
                    debug!("Account {} already exists, retrying", account_id);
                    retries -= 1;
                    if retries < 1 {
                        warn!("Could not create account");
                        return Err(DBError::AccountAlreadyExists(account_id));
                    }
                }
            }
        }
    }

    async fn delete_account(&self, account_id: AccountId) -> super::DBResult<()> {
        if self.verbose {
            debug!("Deleting account {}", account_id);
        }
        self.accounts.write().await.remove(&account_id);
        Ok(())
    }

    async fn get_account_by_id(&self, account_id: AccountId) -> super::DBResult<MmoAccount> {
        if self.verbose {
            debug!("Getting account id={}", account_id);
        }
        self.accounts
            .read()
            .await
            .get(&account_id)
            .cloned()
            .ok_or(DBError::NoSuchAccount(account_id))
    }

    async fn get_account_by_user(&self, user_id: UserId) -> super::DBResult<MmoAccount> {
        if self.verbose {
            debug!("Getting account user={:?}", user_id);
        }
        // TODO: Replace full scan with secondary index UserID -> AccountID
        self.accounts
            .read()
            .await
            .iter()
            .find(|(_, account)| account.user_id == user_id)
            .map(|(_, account)| account.clone())
            .ok_or(DBError::NoSuchUser(user_id))
    }

    async fn save_account(&self, account: &MmoAccount) -> super::DBResult<()> {
        if self.verbose {
            debug!("Saving account {}", account.account_id);
        }
        self.accounts
            .write()
            .await
            .insert(account.account_id, account.clone())
            .ok_or(DBError::NoSuchAccount(account.account_id))
            .map(|_| ())
    }

    async fn enable_webtoken(&self, account_id: AccountId) -> super::DBResult<()> {
        if self.verbose {
            debug!("Enabling webtoken for account {}", account_id);
        }
        todo!()
    }

    async fn disable_webtoken(&self, account_id: AccountId) -> super::DBResult<()> {
        if self.verbose {
            debug!("Disabling webtoken for account {}", account_id);
        }
        todo!()
    }

    async fn remove_webtokens(&self) -> super::DBResult<()> {
        if self.verbose {
            debug!("Removing webtokens");
        }
        todo!()
    }
}
