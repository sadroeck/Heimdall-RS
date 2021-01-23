use api::character::attributes::Location;
use api::config::ServerConfig;
use api::inventory::Item;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub char_server: ServerConfig,
    pub character_db: CharacterDBConfig,
    pub starting_characters: StartingCharacterConfig,
    pub maps: MapConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StartingCharacterConfig {
    pub novice: StartingCharacter,
    pub doram: StartingCharacter,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StartingCharacter {
    pub items: Vec<Item>,
    #[serde(default)]
    pub location: Location,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MapConfig {
    pub names_file: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum CharacterDBConfig {
    InMemory { verbose: bool },
}
