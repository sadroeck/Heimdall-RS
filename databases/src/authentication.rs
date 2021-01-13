use api::{account::db::AccountId, character::AccountInfo};
use dashmap::DashMap;

#[derive(Default)]
pub struct AuthenticationDB {
    accounts: DashMap<AccountId, AccountInfo>,
}

impl AuthenticationDB {
    pub fn authenticate(&self, account_info: AccountInfo) {
        self.accounts.insert(account_info.account_id, account_info);
    }

    pub fn check_if_authenticated(&self, account_info: AccountInfo) -> bool {
        // If this has been set by the login-server, the account is allowed to be authenticated
        self.accounts.remove(&account_info.account_id).is_some()
    }
}
