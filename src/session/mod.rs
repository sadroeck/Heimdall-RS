use std::time::SystemTime;

use async_std::net::TcpStream;

use crate::account::{AccountId, Password, Sex};

const CLIENT_HASH_SIZE: usize = 16;

#[derive(Debug)]
pub struct Session {
    account_id: AccountId,
    sex: Sex,
    user_id: String,
    password: Password,
    last_login: SystemTime,
    client_hash: Option<[u8; CLIENT_HASH_SIZE]>,
    group_id: u8,
    /// Optional MD5 key
    session_key: Option<[u8; 16]>,
    socket: TcpStream,
}
