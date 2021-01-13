use crate::account::db::AccountId;

use super::Character;

#[derive(Debug, thiserror::Error)]
pub enum DBError {
    #[error("No such character {0}")]
    NoSuchCharacter(CharacterId),
}

pub type DBResult<T> = Result<T, DBError>;
pub type CharacterId = u32;

#[async_trait::async_trait]
pub trait CharacterDB {
    async fn init(&mut self) -> DBResult<()>;
    async fn create(&self, account_id: AccountId) -> DBResult<Character>;
    async fn delete(&self, id: CharacterId) -> DBResult<()>;
    async fn get_by_account_id(&self, id: AccountId) -> DBResult<Vec<Character>>;
    async fn get_by_id(&self, id: CharacterId) -> DBResult<Character>;
}
