use serde::{Deserialize, Serialize};

const fn default_is_identified() -> bool {
    true
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: ItemId,
    pub slot: u16,
    pub amount: u16,
    #[serde(default = "default_is_identified")]
    pub identified: bool,
    pub equipped_slot: Option<u8>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ItemId(u32);
