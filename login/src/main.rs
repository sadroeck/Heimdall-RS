use agent::LoginAgent;
use databases::account::db::InMemoryAccountDB;
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

use crate::server::LoginServer;
use api::config::login::AccountDBConfig;
use api::{character::TcpServer as CharTcpServer, config::login::Config};

mod agent;
mod server;

pub fn init_config() -> Result<Config, impl std::error::Error> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config"))?
        .merge(config::Environment::with_prefix("APP"))?;
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

    let char_servers = config
        .char_servers
        .iter()
        .map(CharTcpServer::new)
        .map(Arc::new)
        .collect();
    match config.account_db {
        AccountDBConfig::InMemory { verbose } => {
            let addr: SocketAddr = format!(
                "{}:{}",
                config.login_server.address, config.login_server.port
            )
            .parse()?;
            async_std::task::block_on(async {
                let account_db = InMemoryAccountDB::new(verbose)
                    .await
                    .map_err(anyhow::Error::from)?;
                let login_agent = LoginAgent::new(Arc::new(account_db));
                let login_server = LoginServer::new(login_agent, char_servers);
                login_server.run(addr).await.map_err(anyhow::Error::from)
            })?;
        }
        AccountDBConfig::SQL {} => {
            todo!("Implement me");
        }
    }

    info!("Done...");
    Ok(())
}
