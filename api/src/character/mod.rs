use std::net::Ipv4Addr;

pub use client::TcpClient;
pub use codec::*;
use db::CharacterId;
pub use request::*;
pub use response::*;
pub use server::TcpServer;

mod client;
mod codec;
pub mod db;
mod request;
mod response;
mod server;

#[async_trait::async_trait]
pub trait CharacterServer {
    fn info(&self) -> ServerInfo;
}

#[derive(Clone)]
pub struct ServerInfo {
    pub(crate) ip_addr: Ipv4Addr,
    pub(crate) port: u16,
    pub(crate) name: String,
    pub(crate) active_users: usize,
    pub(crate) server_type: ServerType,
    pub(crate) server_activity: ServerActivity,
}

#[derive(Clone, Copy, Debug)]
pub enum ServerActivity {
    /// No status color
    Hidden,
    /// Status color = green
    Smooth,
    /// Status color = yellow
    Normal,
    /// Status color = red
    Busy,
    /// Status color = purple
    Crowded,
}

impl Into<u16> for ServerActivity {
    fn into(self) -> u16 {
        match self {
            Self::Hidden => 4,
            Self::Smooth => 0,
            Self::Normal => 1,
            Self::Busy => 2,
            Self::Crowded => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ServerType {
    Normal,
    Maintenance,
    AdultOnly,
    Paying,
    F2P,
}

impl Into<u16> for ServerType {
    fn into(self) -> u16 {
        match self {
            Self::Normal => 0,
            Self::Maintenance => 1,
            Self::AdultOnly => 2,
            Self::Paying => 3,
            Self::F2P => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Character {
    id: CharacterId,
}

impl Character {
    pub fn new(id: CharacterId) -> Self {
        Self { id }
    }
}
