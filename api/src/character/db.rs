use crate::account::db::AccountId;

use super::Character;

#[derive(Debug, thiserror::Error)]
pub enum DBError {
    #[error("No such character {0}")]
    NoSuchCharacter(CharacterId),
    #[error("No such slot {0}")]
    NoSuchSlot(u8),
}

pub type DBResult<T> = Result<T, DBError>;
pub type CharacterId = u32;

#[async_trait::async_trait]
pub trait CharacterDB {
    async fn init(&mut self) -> DBResult<()>;
    async fn create(&self, account_id: AccountId) -> DBResult<CharacterId>;
    async fn update(&self, character: &Character) -> DBResult<()>;
    async fn delete(&self, id: CharacterId) -> DBResult<()>;
    async fn get_by_account_id(&self, id: AccountId) -> DBResult<Vec<Character>>;
    async fn get_by_id(&self, id: CharacterId) -> DBResult<Character>;
    async fn get_by_slot(&self, account_id: AccountId, slot: u8) -> DBResult<Character>;
}
