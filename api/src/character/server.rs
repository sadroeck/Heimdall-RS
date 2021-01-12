use std::{
    net::Ipv4Addr,
    sync::atomic::{AtomicUsize, Ordering},
};

use super::{CharacterServer, ServerInfo, ServerType};
use crate::config::login::CharacterServerConfig;

pub struct TcpServer {
    name: String,
    ip_addr: Ipv4Addr,
    port: u16,
    server_type: ServerType,
    active_users: AtomicUsize,
}

#[async_trait::async_trait]
impl CharacterServer for TcpServer {
    fn info(&self) -> ServerInfo {
        ServerInfo {
            ip_addr: self.ip_addr,
            port: self.port,
            name: self.name.clone(),
            active_users: self.active_users.load(Ordering::Relaxed),
            server_type: self.server_type,
            server_activity: super::ServerActivity::Smooth,
        }
    }
}

impl TcpServer {
    pub fn new(config: &CharacterServerConfig) -> Self {
        let ip_addr = config.address.parse().unwrap();
        Self {
            ip_addr,
            name: config.name.clone(),
            port: config.port,
            server_type: ServerType::Normal,
            active_users: AtomicUsize::new(0),
        }
    }
}
