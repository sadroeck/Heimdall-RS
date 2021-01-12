use api::account::{
    db::{AccountDB, AccountId, DBError, DBResult, UserId},
    mmo_account::{MmoAccount, Password},
};
use async_std::sync::RwLock;
use std::collections::{hash_map::Entry, HashMap};
use tracing::{debug, info, warn};

pub struct InMemoryAccountDB {
    verbose: bool,
    accounts: RwLock<HashMap<AccountId, MmoAccount>>,
}

impl InMemoryAccountDB {
    pub async fn new(verbose: bool) -> DBResult<Self> {
        let mut s = Self {
            verbose,
            accounts: RwLock::new(HashMap::new()),
        };
        s.init().await?;
        Ok(s)
    }
}

#[async_trait::async_trait]
impl AccountDB for InMemoryAccountDB {
    async fn init(&mut self) -> DBResult<()> {
        info!("Initializing InMemory account DB");
        let mut test_account = MmoAccount::default();
        test_account.account_id = 2_000_042;
        test_account.user_id = "sadroeck".to_string();
        test_account.password = Password::Cleartext("olasenor".to_string());
        self.accounts.write().await.insert(2_000_042, test_account);
        Ok(())
    }

    async fn create_account(&self) -> DBResult<MmoAccount> {
        let mut retries: i32 = 10;
        loop {
            let account_id = fastrand::u32(..);
            if self.verbose {
                debug!(%account_id, "Creating new account");
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

    async fn delete_account(&self, account_id: AccountId) -> DBResult<()> {
        if self.verbose {
            debug!(%account_id, "Deleting account");
        }
        self.accounts.write().await.remove(&account_id);
        Ok(())
    }

    async fn get_account_by_id(&self, account_id: AccountId) -> DBResult<MmoAccount> {
        if self.verbose {
            debug!(%account_id, "Getting account");
        }
        self.accounts
            .read()
            .await
            .get(&account_id)
            .cloned()
            .ok_or(DBError::NoSuchAccount(account_id))
    }

    async fn get_account_by_user(&self, user_id: &UserId) -> DBResult<MmoAccount> {
        if self.verbose {
            debug!(%user_id, "Getting account");
        }
        // TODO: Replace full scan with secondary index UserID -> AccountID
        self.accounts
            .read()
            .await
            .iter()
            .find(|(_, account)| account.user_id == *user_id)
            .map(|(_, account)| account.clone())
            .ok_or(DBError::NoSuchUser(user_id.clone()))
    }

    async fn save_account(&self, account: &MmoAccount) -> DBResult<()> {
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

    async fn enable_webtoken(&self, account_id: AccountId) -> DBResult<()> {
        if self.verbose {
            debug!("Enabling webtoken for account {}", account_id);
        }
        todo!()
    }

    async fn disable_webtoken(&self, account_id: AccountId) -> DBResult<()> {
        if self.verbose {
            debug!("Disabling webtoken for account {}", account_id);
        }
        todo!()
    }

    async fn remove_webtokens(&self) -> DBResult<()> {
        if self.verbose {
            debug!("Removing webtokens");
        }
        todo!()
    }
}
