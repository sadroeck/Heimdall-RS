use crate::character::db::CharacterId;
use serde::{Deserialize, Serialize};

mod item;

pub use item::{Item, ItemId};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub character_id: CharacterId,
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new(character_id: CharacterId) -> Self {
        Self {
            character_id,
            items: vec![],
        }
    }
}
