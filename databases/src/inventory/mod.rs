mod in_memory;

use api::character::db::CharacterId;
use api::inventory::Inventory;

pub use in_memory::InMemoryInventoryDB;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No such character {0}")]
    NoSuchCharacter(CharacterId),
}

pub type DBResult<T> = Result<T, Error>;

#[async_trait::async_trait]
pub trait InventoryDB {
    async fn create(&self, inventory: Inventory) -> DBResult<()>;
    async fn get(&self, character_id: CharacterId) -> DBResult<Inventory>;
    async fn update(&self, inventory: &Inventory) -> DBResult<()>;
}
