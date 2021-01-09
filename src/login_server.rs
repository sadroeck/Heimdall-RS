use std::sync::Arc;

use async_codec::Framed;
use async_std::{
    io::Error as IOError,
    net::{SocketAddr, TcpListener, TcpStream},
    stream::StreamExt,
    task,
};
use futures_util::SinkExt;
use tracing::{debug, error, info, warn};

use crate::{
    account::{self, AccountDB},
    api::login::{LoginCodec, LoginCredentials, LoginFailed, Request, Response},
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

impl<A: AccountDB + Send + Sync + 'static> LoginServer<A> {
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
            let session = LoginSession {
                account_db: self.account_db.clone(),
            };
            task::spawn(async move { session.process_connection(stream).await });
        }
        Ok(())
    }
}

struct LoginSession<A: AccountDB + Send + Sync> {
    account_db: Arc<A>,
}

impl<A: AccountDB + Send + Sync> LoginSession<A> {
    async fn process_connection(&self, stream: TcpStream) {
        if let Ok(peer_addr) = stream.peer_addr() {
            // TODO: Check IP blacklist
            debug!("Received incoming connection from {}", peer_addr);
        }

        let mut framed_stream = Framed::new(stream, LoginCodec {});

        loop {
            match framed_stream.next().await {
                Some(Ok(request)) => {
                    if let Some(response) = self.process_request(request).await {
                        if let Err(err) = framed_stream.send(response).await {
                            error!(%err);
                            break;
                        }
                    }
                }
                Some(Err(err)) => {
                    error!(%err, "Could not parse request");
                    break;
                }
                None => {
                    debug!("End-of-stream. Terminating connection");
                    break;
                }
            }
        }
    }

    async fn process_request(&self, request: Request) -> Option<Response> {
        match request {
            Request::KeepAlive => {
                debug!("Keep alive");
                None
            }
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
                    // TODO: Remove printing of password
                    debug!(%username, ?password, "Logging in username={} pass={:?}", username, password);
                    match self.account_db.get_account_by_user(&username).await {
                        Ok(account) => {
                            if let account::Password::MD5Hashed(hashed) = account.password {
                                if password == hashed {
                                    todo!("Handle successful login");
                                } else {
                                    warn!(%username, "Invalid password");
                                    todo!("handle response");
                                }
                            } else {
                                warn!("Invalid password type");
                                todo!("handle response");
                            }
                        }
                        Err(err) => {
                            error!(%err);
                            None
                        }
                    }
                }
                LoginCredentials::ClearText {
                    username, password, ..
                } => {
                    // TODO: Remove printing of password
                    debug!(%username, %password, "Logging in");
                    match self.account_db.get_account_by_user(&username).await {
                        Ok(account) => {
                            if let account::Password::Cleartext(cleartext) = account.password {
                                if password == cleartext {
                                    debug!("Successful login");
                                    Some(Response::LoginSuccessV3)
                                } else {
                                    warn!("Invalid password");
                                    Some(Response::LoginFailed(LoginFailed::IncorrectPassword))
                                }
                            } else {
                                warn!("Invalid password type");
                                Some(Response::LoginFailed(LoginFailed::IncorrectPassword))
                            }
                        }
                        Err(err) => {
                            error!(%err);
                            Some(Response::LoginFailed(LoginFailed::UnregisteredId(
                                username.clone(),
                            )))
                        }
                    }
                }
            },
            Request::CodeKey => todo!("Handle request new session key"),
            Request::OneTimeToken => todo!("Handle OTP"),
            Request::ConnectChar => todo!("Handle connect char"),
        }
    }
}
