use std::sync::Arc;

use async_std::{
    io::Error as IOError,
    net::{Shutdown, SocketAddr, TcpListener},
    task,
};
use async_std::{net::TcpStream, stream::StreamExt};
use tracing::{debug, error, info, warn};

use crate::{
    account::{self, AccountDB},
    api::login::{LoginCredentials, Request},
};

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("{0}")]
    IO(#[from] IOError),
}

#[derive(Debug)]
pub struct LoginServer<A: AccountDB + Send + Sync + 'static> {
    account_db: Arc<A>,
}

impl<A: AccountDB + Send + Sync> LoginServer<A> {
    pub fn new(account_db: Arc<A>) -> Self {
        Self { account_db }
    }

    pub async fn run(self, addr: impl Into<SocketAddr>) -> Result<(), ServerError> {
        let addr = addr.into();
        let listener = TcpListener::bind(addr).await?;
        info!("Listening on {}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream: TcpStream = stream?;
            let account_db = self.account_db.clone();
            task::spawn(process_connection(account_db, stream));
        }
        Ok(())
    }
}

async fn process_connection<A: AccountDB + Send + Sync>(account_db: Arc<A>, stream: TcpStream) {
    if let Ok(peer_addr) = stream.peer_addr() {
        // TODO: Check IP blacklist
        debug!("Received incoming connection from {}", peer_addr);
    }

    match Request::parse(&stream).await {
        Ok(request) => {
            debug!("Received request: {:#?}", request);
            match request {
                Request::KeepAlive => {}
                Request::UpdateClientHash(hash) => {
                    debug!("Updating client hash {:?}", hash);
                    todo!("handle update client hash");
                }
                Request::ClientLogin(credentials) => match credentials {
                    LoginCredentials::OTP { .. } => {
                        todo!("Handle OTPs");
                    }
                    LoginCredentials::Hashed {
                        username, password, ..
                    } => {
                        debug!("Logging in username={} pass={:?}",
                            String::from_utf8_lossy(&username[..]),
                           password,
                        );
                        match account_db.get_account_by_user(username).await {
                            Ok(account) => {
                                if let account::Password::MD5Hashed(hashed) = account.password {
                                    if password == hashed {
                                        debug!("Successful login");
                                    } else {
                                        warn!("Invalid password");
                                        todo!("handle response");
                                    }
                                } else {
                                    warn!("Invalid password type");
                                    todo!("handle response");
                                }
                            }
                            Err(err) => {
                                error!(%err);
                            }
                        }
                    }
                    LoginCredentials::ClearText {
                        username, password, ..
                    } => {
                        debug!("Logging in username={} pass={}",
                            String::from_utf8_lossy(&username[..]),
                            String::from_utf8_lossy(&password[..])
                        );
                        match account_db.get_account_by_user(username).await {
                            Ok(account) => {
                                if let account::Password::Cleartext(cleartext) = account.password {
                                    if password == cleartext {
                                        debug!("Successful login");
                                    } else {
                                        warn!("Invalid password");
                                        todo!("handle response");
                                    }
                                } else {
                                    warn!("Invalid password type");
                                    todo!("handle response");
                                }
                            }
                            Err(err) => {
                                error!(%err);
                            }
                        }
                    }
                },
                Request::CodeKey => {}
                Request::OneTimeToken => {}
                Request::ConnectChar => {}
            }
        }
        Err(err) => {
            error!("Could not parse request: {}", err);
            stream
                .shutdown(Shutdown::Both)
                .expect("Could not shut down stream");
        }
    }
}
