use std::sync::Arc;

use crate::authentication_db::AuthenticationDB;
use crate::config::StartingCharacterConfig;
use api::account::db::AccountId;
use api::character::{attributes, NewCharacter, MAX_CHARACTERS_PER_ACCOUNT};
use api::inventory::Inventory;
use api::{
    character::{
        db::{CharacterDB, DBError as CharacterDBError, DBResult},
        AccountInfo, Character,
    },
    pincode::{PincodeInfo, PincodeStatus},
};
use databases::inventory::{Error as InventoryDBError, InventoryDB};
use tracing::error;
use tracing_attributes::instrument;

#[derive(Debug, thiserror::Error)]
pub enum CharCreationError {
    #[error("Slot {0} is invalid")]
    InvalidSlot(u8),
    #[error("Too many characters exist: {0}")]
    TooManyCharacters(u8),
    #[error("Invalid starting class {0:?}")]
    InvalidClass(attributes::Class),
    #[error("No such account {0}")]
    NoSuchAccount(AccountId),
    #[error("Could not access inventory: {0}")]
    InventoryDB(#[from] InventoryDBError),
    #[error("Internal {0}")]
    CharacterDB(#[from] CharacterDBError),
}

pub struct CharacterSession {
    starting_char_config: Arc<StartingCharacterConfig>,
    authentication_db: Arc<AuthenticationDB>,
    character_db: Arc<dyn CharacterDB + Send + Sync>,
    inventory_db: Arc<dyn InventoryDB + Send + Sync>,
    account_info: Option<AccountInfo>,
}

impl CharacterSession {
    pub fn new(
        starting_char_config: Arc<StartingCharacterConfig>,
        authentication_db: Arc<AuthenticationDB>,
        character_db: Arc<dyn CharacterDB + Send + Sync>,
        inventory_db: Arc<dyn InventoryDB + Send + Sync>,
    ) -> Self {
        Self {
            starting_char_config,
            authentication_db,
            character_db,
            inventory_db,
            account_info: None,
        }
    }

    #[instrument(skip(self), level = "debug")]
    pub async fn is_authenticated(&mut self, account_info: AccountInfo) -> bool {
        // TODO: remove
        self.account_info = Some(account_info);

        if let Some(info) = self.account_info.as_ref() {
            // TODO: Re-enable
            // error!(account_id = %account_info.account_id, "Already authenticated");
            // false
            true
        } else {
            if self.authentication_db.check_if_authenticated(account_info) {
                self.account_info = Some(account_info);
                true
            } else {
                error!(account_id = %account_info.account_id, "Not authenticated");
                false
            }
        }
    }

    #[instrument(skip(self), level = "debug")]
    pub async fn get_pincode_info(&self) -> DBResult<PincodeInfo> {
        Ok(PincodeInfo {
            status: PincodeStatus::Correct,
            account_id: self.account_info.unwrap().account_id,
        })
    }

    #[instrument(skip(self), level = "debug")]
    pub async fn get_characters(&self) -> DBResult<Vec<Character>> {
        self.character_db
            .get_by_account_id(self.account_info.unwrap().account_id)
            .await
    }

    #[instrument(skip(self), level = "debug")]
    pub async fn create_character(
        &self,
        new_character: NewCharacter,
    ) -> Result<Character, CharCreationError> {
        let NewCharacter {
            name,
            slot,
            stats,
            appearance,
            class,
            sex,
        } = new_character;
        if slot as usize >= MAX_CHARACTERS_PER_ACCOUNT {
            return Err(CharCreationError::InvalidSlot(new_character.slot));
        }

        // Check preconditions
        let account_id = self.account_info.unwrap().account_id;
        if self
            .character_db
            .get_by_account_id(account_id)
            .await
            .map_err(|err| {
                error!(account_id = %account_id, %err, "Could not retrieve characters");
                CharCreationError::NoSuchAccount(account_id)
            })?
            .len()
            >= MAX_CHARACTERS_PER_ACCOUNT
        {
            return Err(CharCreationError::TooManyCharacters(
                MAX_CHARACTERS_PER_ACCOUNT as u8,
            ));
        }
        // Just ignore the slot & index ourselves
        let starting_config = match class {
            attributes::Class::Novice => Ok(&self.starting_char_config.novice),
            attributes::Class::Summoner => Ok(&self.starting_char_config.doram),
            invalid => Err(CharCreationError::InvalidClass(invalid)),
        }?;

        // Create empty character
        let char_id = self.character_db.create(account_id).await?;

        // Initialize inventory
        let mut inventory = Inventory::new(char_id);
        inventory.items = starting_config.items.clone();
        self.inventory_db.create(inventory).await?;

        // Retrieve character
        let mut char = self.character_db.get_by_id(char_id).await?;

        // Update with creation info
        char.name = name;
        char.slot = slot as u16;
        char.stats = stats;
        char.appearance = appearance;
        char.class = class;
        char.sex = sex;
        self.character_db.update(&char).await?;

        Ok(char)
    }
}
