use api::config::ServerConfig;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub char_server: ServerConfig,
}
