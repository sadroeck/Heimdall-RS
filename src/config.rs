use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub(crate) account_db: AccountDB,
    pub(crate) login_server: LoginServer,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum AccountDB {
    InMemory { verbose: bool },
    SQL {},
}

#[derive(Deserialize, Debug)]
pub struct LoginServer {
    pub(crate) address: String,
    pub(crate) port: u16,
}

pub fn init_config() -> Result<Config, impl Error> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config"))?
        .merge(config::Environment::with_prefix("APP"))?;
    settings.try_into()
}
