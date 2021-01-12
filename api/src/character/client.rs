use std::sync::atomic::AtomicUsize;
use std::{net::Ipv4Addr, sync::atomic::Ordering};

use super::{CharacterServer, ServerInfo, ServerType};

pub struct TcpClient {
    name: String,
    ip_addr: Ipv4Addr,
    port: u16,
    server_type: ServerType,
    active_users: AtomicUsize,
}

#[async_trait::async_trait]
impl CharacterServer for TcpClient {
    fn info(&self) -> ServerInfo {
        let active_users = self.active_users.load(Ordering::Relaxed);
        // TODO: determine dynamically
        let server_activity = super::ServerActivity::Normal;
        ServerInfo {
            ip_addr: self.ip_addr,
            port: self.port,
            name: self.name.clone(),
            active_users,
            server_type: self.server_type,
            server_activity,
        }
    }
}
