use serde::Deserialize;

use super::ServerConfig;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub account_db: AccountDBConfig,
    pub login_server: ServerConfig,
    pub char_servers: Vec<ServerConfig>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum AccountDBConfig {
    InMemory { verbose: bool },
    SQL {},
}
