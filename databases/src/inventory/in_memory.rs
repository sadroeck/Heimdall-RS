use crate::inventory::{DBResult, Error, InventoryDB};
use api::character::db::CharacterId;
use api::inventory::Inventory;
use dashmap::DashMap;
use tracing::debug;

pub struct InMemoryInventoryDB {
    verbose: bool,
    db: DashMap<CharacterId, Inventory>,
}

impl InMemoryInventoryDB {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            db: DashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl InventoryDB for InMemoryInventoryDB {
    async fn create(&self, inventory: Inventory) -> DBResult<()> {
        if self.verbose {
            debug!(character_id = %inventory.character_id, "Creating new inventory");
        }
        self.db.insert(inventory.character_id, inventory);
        Ok(())
    }

    async fn get(&self, character_id: u32) -> DBResult<Inventory> {
        self.db
            .get(&character_id)
            .map(|x| x.value().clone())
            .ok_or(Error::NoSuchCharacter(character_id))
    }

    async fn update(&self, inventory: &Inventory) -> DBResult<()> {
        self.db.insert(inventory.character_id, inventory.clone());
        Ok(())
    }
}
