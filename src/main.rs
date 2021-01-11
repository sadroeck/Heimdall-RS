use login_agent::LoginAgent;
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

use account::InMemoryAccountDB;
use api::character::TcpServer as CharTcpServer;
use login_server::LoginServer;

mod account;
pub(crate) mod api;
mod config;
mod login_agent;
mod login_server;
mod session;

fn main() -> Result<(), anyhow::Error> {
    // Initialize config
    let config = config::init_config()?;

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
        config::AccountDB::InMemory { verbose } => {
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
        config::AccountDB::SQL {} => {
            todo!("Implement me");
        }
    }

    info!("Done...");
    Ok(())
}
