use super::mmo_account::MmoAccount;

mod in_memory;

pub use in_memory::InMemoryAccountDB;

#[derive(Debug, thiserror::Error)]
pub enum DBError {
    #[error("No such account {0}")]
    NoSuchAccount(AccountId),
    #[error("No such user {0}")]
    NoSuchUser(UserId),
    #[error("Account {0:?} already exists")]
    AccountAlreadyExists(AccountId),
}

pub type DBResult<T> = Result<T, DBError>;
pub type AccountId = u32;
pub type UserId = String;

#[async_trait::async_trait]
pub trait AccountDB {
    async fn init(&mut self) -> DBResult<()>;

    // Accounts
    async fn create_account(&self) -> DBResult<MmoAccount>;
    async fn delete_account(&self, account_id: AccountId) -> DBResult<()>;
    async fn get_account_by_id(&self, account_id: AccountId) -> DBResult<MmoAccount>;
    async fn get_account_by_user(&self, user_id: &UserId) -> DBResult<MmoAccount>;
    async fn save_account(&self, account: &MmoAccount) -> DBResult<()>;

    // Webtokens
    async fn enable_webtoken(&self, account_id: AccountId) -> DBResult<()>;
    async fn disable_webtoken(&self, account_id: AccountId) -> DBResult<()>;
    async fn remove_webtokens(&self) -> DBResult<()>;
}
