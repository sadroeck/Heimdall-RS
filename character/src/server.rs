use std::sync::Arc;

use crate::authentication_db::AuthenticationDB;
use async_codec::Framed;
use async_std::{
    io::Error as IOError,
    net::{SocketAddr, TcpListener, TcpStream},
    stream::StreamExt,
    task,
};
use flume::{bounded, Receiver};
use futures_util::future::{select, Either};
use futures_util::SinkExt;
use tracing::{debug, error, info};

use api::character::{CharacterCodec, Request, Response};

use crate::session::CharacterSession;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("{0}")]
    IO(#[from] IOError),
}

pub struct CharacterServer {}

impl CharacterServer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(self, addr: impl Into<SocketAddr>) -> Result<(), ServerError> {
        let addr = addr.into();
        let listener = TcpListener::bind(addr).await?;
        info!("Listening on {}", listener.local_addr()?);

        let authentication_db = Arc::new(AuthenticationDB::default());
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream: TcpStream = stream?;
            let (response_tx, response_rx) = bounded(10);
            let session = CharacterSession::new(response_tx, authentication_db.clone());
            task::spawn(async move { process_connection(session, response_rx, stream).await });
        }
        Ok(())
    }
}

async fn process_connection(
    mut session: CharacterSession,
    response_rx: Receiver<Response>,
    stream: TcpStream,
) {
    let ip_addr = stream.peer_addr().expect("Could not retrieve peer addr");
    debug!(ip = %ip_addr, "Received incoming connection");

    let mut framed_stream = Framed::new(stream, CharacterCodec {});

    loop {
        let incoming_request = framed_stream.next();
        let outgoing_response = response_rx.recv_async();
        match select(incoming_request, outgoing_response).await {
            Either::Left((None, _)) => {
                debug!("connection closed by client");
                break;
            }
            Either::Left((Some(Err(err)), _)) => {
                error!(%err, "Could not parse request");
                break;
            }
            Either::Left((Some(Ok(request)), _)) => {
                match request {
                    Request::ConnectClient(account_info) => {
                        if session.is_authenticated(account_info).await {
                            session.list_characters().await;
                        } else {
                            if let Err(err) = framed_stream.send(Response::Rejected).await {
                                error!(%err, "Could not send response");
                                break;
                            }
                        }
                    }
                    Request::ListCharacters => todo!("Handle ListCharacters"),
                    Request::SelectCharacter => todo!("Handle SelectCharacter"),
                    Request::CreateCharacter => todo!("Handle CreateCharacter"),
                    Request::DeleteCharacter => todo!("Handle DeleteCharacter"),
                    Request::RequestCharacterDeletion => todo!("Handle RequestCharacterDeletion"),
                    Request::AcceptCharacterDeletion => todo!("Handle AcceptCharacterDeletion"),
                    Request::CancelCharacterDeletion2 => todo!("Handle CancelCharacterDeletion2"),
                    Request::RenameCharacter => todo!("Handle RenameCharacter"),
                    Request::RequestCaptcha => todo!("Handle RequestCaptcha"),
                    Request::CheckCaptcha => todo!("Handle CheckCaptcha"),
                    Request::MoveCharacterSlot => todo!("Handle MoveCharacterSlot"),
                    Request::KeepAlive => todo!("Handle KeepAlive"),
                    Request::CheckPincode => todo!("Handle CheckPincode"),
                    Request::RequestPincode => todo!("Handle RequestPincode"),
                    Request::ChangePincode => todo!("Handle ChangePincode"),
                    Request::NewPincode => todo!("Handle NewPincode"),
                };
            }
            Either::Right((Ok(response), _)) => {
                if let Err(err) = framed_stream.send(response).await {
                    error!(%err, "Could not send response");
                    break;
                }
            }
            Either::Right((Err(_), _)) => {
                debug!("No longer processing. Closing connection");
                break;
            }
        }
    }
}
