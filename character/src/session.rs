use std::sync::Arc;

use crate::authentication_db::AuthenticationDB;
use api::{
    character::{
        db::{CharacterDB, DBResult},
        AccountInfo, Character,
    },
    pincode::{PincodeInfo, PincodeStatus},
};
use tracing::error;
use tracing_attributes::instrument;

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
}
