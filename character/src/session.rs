use std::sync::Arc;

use api::character::{AccountInfo, CharacterSelectWindowInfo, Response};
use databases::authentication::AuthenticationDB;
use flume::Sender;
use tracing::error;

pub struct CharacterSession {
    response_tx: Sender<Response>,
    authentication_db: Arc<AuthenticationDB>,
    account_info: Option<AccountInfo>,
}

impl CharacterSession {
    pub fn new(response_tx: Sender<Response>, authentication_db: Arc<AuthenticationDB>) -> Self {
        Self {
            response_tx,
            authentication_db,
            account_info: None,
        }
    }

    pub async fn is_authenticated(&mut self, account_info: AccountInfo) -> bool {
        if let Some(account_info) = self.account_info.as_ref() {
            error!(account_id = %account_info.account_id, "Already authenticated");
            false
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

    pub async fn list_characters(&self) {
        // todo: fetch proper info
        let info = CharacterSelectWindowInfo {
            normal_slots: 15,
            vip_slots: 0,
            billing_slots: 0,
            producible_slots: 15,
            valid_slots: 15,
        };
        self.response_tx
            .send_async(Response::CharacterSelectWindowInfo(info))
            .await
            .unwrap_or(());
    }
}
