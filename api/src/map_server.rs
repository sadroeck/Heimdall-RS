use crate::account::db::AccountId;

#[derive(Debug, thiserror::Error)]
pub enum MapServerError {}

#[async_trait::async_trait]
pub trait MapServer {
    async fn character_selected(&self) -> Result<(), MapServerError>;
    async fn change_server(&self) -> Result<(), MapServerError>;

    async fn get_maps(&self) -> Result<Vec<u32>, MapServerError>;
    async fn get_accounts(&self) -> Result<Vec<AccountId>, MapServerError>;

    async fn send_player_count(&self) -> Result<(), MapServerError>;
}
