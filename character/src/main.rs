use crate::config::Config;
use server::CharacterServer;
use std::net::SocketAddr;
use tracing::info;

mod authentication_db;
mod config;
mod server;
mod session;

pub fn init_config() -> Result<Config, impl std::error::Error> {
    let mut settings = ::config::Config::default();
    settings
        .merge(::config::File::with_name("config"))?
        .merge(::config::Environment::with_prefix("APP"))?;
    settings.try_into()
}

fn main() -> Result<(), anyhow::Error> {
    // Initialize config
    let config = init_config()?;

    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(true)
        .with_env_filter("debug")
        .try_init()
        .expect("Could not initialize logging");

    info!("Running with config:\n{:#?}", config);

    async_std::task::block_on(async {
        let addr: SocketAddr =
            format!("{}:{}", config.char_server.address, config.char_server.port).parse()?;
        let character_server = CharacterServer::new();
        character_server
            .run(config, addr)
            .await
            .map_err(anyhow::Error::from)
    })?;

    info!("Done...");
    Ok(())
}
