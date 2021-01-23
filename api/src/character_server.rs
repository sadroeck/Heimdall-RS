#[derive(Debug, thiserror::Error)]
pub enum ConnectError {}

#[async_trait::async_trait]
pub trait CharacterServer {
    async fn connect_map_server(&self) -> Result<(), ConnectError>;
    async fn ping(&self) -> Result<(), ConnectError>;
}
