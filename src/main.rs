use async_std::net::Ipv4Addr;
use std::{error::Error, sync::Arc};

use account::InMemoryAccountDB;
use login_server::LoginServer;
use tracing::info;

mod account;
pub(crate) mod api;
mod config;
mod login_server;
mod session;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize config
    let config = config::init_config()?;

    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(true)
        .with_env_filter("debug")
        .try_init()
        .expect("Could not initialize logging");

    info!("Running with config:\n{:#?}", config);

    match config.account_db {
        config::AccountDB::InMemory { verbose } => {
            let account_db = InMemoryAccountDB::new(verbose);
            let login_server = LoginServer::new(Arc::new(account_db));
            async_std::task::block_on(login_server.run((Ipv4Addr::LOCALHOST, 6061)))?;
        }
        config::AccountDB::SQL {} => {
            todo!("Implement me");
        }
    }

    info!("Done...");
    Ok(())
}
