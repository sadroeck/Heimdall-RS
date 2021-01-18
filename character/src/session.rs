use std::sync::Arc;

use crate::authentication_db::AuthenticationDB;
use api::account::db::AccountId;
use api::character::{attributes, NewCharacter, MAX_CHARACTERS_PER_ACCOUNT};
use api::{
    character::{
        db::{CharacterDB, DBResult},
        AccountInfo, Character,
    },
    pincode::{PincodeInfo, PincodeStatus},
};
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
}

pub struct CharacterSession {
    authentication_db: Arc<AuthenticationDB>,
    character_db: Arc<dyn CharacterDB + Send + Sync>,
    account_info: Option<AccountInfo>,
}

impl CharacterSession {
    pub fn new(
        authentication_db: Arc<AuthenticationDB>,
        character_db: Arc<dyn CharacterDB + Send + Sync>,
    ) -> Self {
        Self {
            authentication_db,
            character_db,
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
        if new_character.slot as usize >= MAX_CHARACTERS_PER_ACCOUNT {
            return Err(CharCreationError::InvalidSlot(new_character.slot));
        }

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
        match new_character.class {
            attributes::Class::Novice | attributes::Class::Summoner => Ok(()),
            invalid => Err(CharCreationError::InvalidClass(invalid)),
        }?;

        // TODO: Initialize doram class properly
        let experience = attributes::Experience::default();

        todo!()
    }
}
