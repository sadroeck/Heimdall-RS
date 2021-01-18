use api::config::ServerConfig;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub char_server: ServerConfig,
    pub starting_characters: StartingCharacterConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StartingCharacterConfig {
    pub novice: StartingCharacter,
    pub doram: StartingCharacter,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StartingCharacter {
    pub items: Vec<StartingItem>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StartingItem {
    pub id: u16,
    pub count: u16,
    pub position: Option<u16>,
}
