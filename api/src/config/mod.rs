use serde::Deserialize;

pub mod login;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub name: String,
    pub address: String,
    pub port: u16,
}
