use async_std::net::IpAddr;

#[derive(Debug, thiserror::Error)]
pub enum ConnectError {}

#[async_trait::async_trait]
pub trait CharacterServer {
    async fn connect_map_server(&self, name: &str, ip: IpAddr) -> Result<(), ConnectError>;
    async fn ping(&self) -> Result<(), ConnectError>;
}
