use std::sync::Arc;

use async_codec::Framed;
use async_std::{
    io::Error as IOError,
    net::{SocketAddr, TcpListener, TcpStream},
    stream::StreamExt,
    task,
};
use futures_util::SinkExt;
use stackvec::StackVec;
use tracing::{debug, error, info, warn};

use crate::{
    account::AccountDB,
    api::{
        character::{CharacterServer, ServerInfo as CharacterServerInfo},
        login::{CharacterSelectionInfo, LoginCodec, Request, Response},
    },
    login_agent::LoginAgent,
};

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("{0}")]
    IO(#[from] IOError),
}

pub struct LoginServer<A, C>
where
    A: AccountDB + Send + Sync + 'static,
    C: CharacterServer + Send + Sync + 'static,
{
    login_agent: Arc<LoginAgent<A, C>>,
    char_servers: Vec<Arc<C>>,
}

impl<A, C> LoginServer<A, C>
where
    A: AccountDB + Send + Sync + 'static,
    C: CharacterServer + Send + Sync + 'static,
{
    pub fn new(login_agent: LoginAgent<A, C>, char_servers: Vec<Arc<C>>) -> Self {
        Self {
            login_agent: Arc::new(login_agent),
            char_servers,
        }
    }

    fn character_server_info(&self) -> StackVec<[CharacterServerInfo; 5]> {
        self.char_servers
            .iter()
            .map(|server| server.info())
            .collect()
    }

    pub async fn run(self, addr: impl Into<SocketAddr>) -> Result<(), ServerError> {
        let addr = addr.into();
        let listener = TcpListener::bind(addr).await?;
        info!("Listening on {}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream: TcpStream = stream?;
            let login_agent = self.login_agent.clone();
            let char_server_info = self.character_server_info();
            task::spawn(
                async move { process_connection(login_agent, stream, char_server_info).await },
            );
        }
        Ok(())
    }
}

async fn process_connection<A, C>(
    login_agent: Arc<LoginAgent<A, C>>,
    stream: TcpStream,
    char_server_info: StackVec<[CharacterServerInfo; 5]>,
) where
    A: AccountDB + Send + Sync + 'static,
    C: CharacterServer + Send + Sync + 'static,
{
    let ip_addr = stream.peer_addr().expect("Could not retrieve peer addr");
    debug!(ip = %ip_addr, "Received incoming connection");

    let mut framed_stream = Framed::new(stream, LoginCodec {});
    let mut client_hash = Option::<[u8; 16]>::None;

    loop {
        match framed_stream.next().await {
            Some(Ok(request)) => {
                let response = match request {
                    Request::KeepAlive => {
                        warn!(ip = %ip_addr, "unexpected KeepAlive");
                        None
                    }
                    Request::UpdateClientHash(hash) => {
                        client_hash = Some(hash);
                        None
                    }
                    Request::ClientLogin(credentials) => {
                        match login_agent.authenticate(credentials).await {
                            Ok(account) => {
                                let info = CharacterSelectionInfo {
                                    account_id: account.account_id,
                                    authentication_code: fastrand::u32(1..u32::MAX),
                                    user_level: fastrand::u32(1..u32::MAX),
                                    sex: account.sex,
                                    web_auth_token: account.web_auth_token,
                                    char_servers: char_server_info.clone(),
                                };
                                login_agent.create_session(account.account_id);
                                Some(Response::LoginSuccessV3(info))
                            }
                            Err(failure) => Some(Response::LoginFailed(failure)),
                        }
                    }
                    Request::CodeKey | Request::OneTimeToken | Request::ConnectChar => {
                        todo!("Handle request")
                    }
                };

                if let Some(response) = response {
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
