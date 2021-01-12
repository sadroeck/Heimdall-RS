use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub account_db: AccountDBConfig,
    pub login_server: LoginServerConfig,
    pub char_servers: Vec<CharacterServerConfig>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum AccountDBConfig {
    InMemory { verbose: bool },
    SQL {},
}

#[derive(Deserialize, Debug)]
pub struct LoginServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct CharacterServerConfig {
    pub name: String,
    pub address: String,
    pub port: u16,
}
