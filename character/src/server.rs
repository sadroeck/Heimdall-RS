use std::sync::Arc;

use crate::authentication_db::AuthenticationDB;
use async_codec::{Framed, WriteFrameError};
use async_std::{
    io::Error as IOError,
    net::{SocketAddr, TcpListener, TcpStream},
    stream::StreamExt,
    task,
};
use databases::character::InMemoryCharacterDB;
use futures_util::SinkExt;
use tracing::{debug, error, info, info_span, trace, Instrument};

use api::{
    character::{db::DBError, CharacterCodec, Request, Response},
    error::PacketError,
};

use crate::config::Config;
use crate::session::CharacterSession;
use api::map::Maps;
use databases::inventory::InMemoryInventoryDB;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("{0}")]
    IO(#[from] IOError),
    #[error("{0}")]
    CharacterDB(#[from] DBError),
    #[error("Could not send response: {0}")]
    SendingResponse(#[from] WriteFrameError<PacketError>),
}

pub struct CharacterServer {}

impl CharacterServer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(
        self,
        config: Config,
        addr: impl Into<SocketAddr>,
    ) -> Result<(), anyhow::Error> {
        let addr = addr.into();
        let listener = TcpListener::bind(addr).await?;
        info!("Listening on {}", listener.local_addr()?);

        let authentication_db = Arc::new(AuthenticationDB::default());
        // TODO: parse from config
        let char_db = Arc::new(InMemoryCharacterDB::new(true).await?);
        let inventory_db = Arc::new(InMemoryInventoryDB::new(true));
        let maps = Arc::new(Maps::from_file(&config.maps.names_file)?);
        let starting_char_config = Arc::new(config.starting_characters.clone());
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream: TcpStream = stream?;
            let session = CharacterSession::new(
                starting_char_config.clone(),
                authentication_db.clone(),
                char_db.clone(),
                inventory_db.clone(),
            );
            let codec = CharacterCodec::new(maps.clone());
            task::spawn(async move { process_connection(session, codec, stream).await });
        }
        Ok(())
    }
}

async fn process_connection(
    mut session: CharacterSession,
    codec: CharacterCodec,
    stream: TcpStream,
) -> Result<(), anyhow::Error> {
    let socket = stream.peer_addr().expect("Could not retrieve peer addr");
    debug!(ip = %socket.ip(), port=socket.port(), "Received incoming connection");

    let mut framed_stream = Framed::new(stream, codec);
    async move {
        while let Some(request) = framed_stream.next().await {
            match request {
                Ok(request) => {
                    if let Err(err) =
                        process_request(&mut session, &mut framed_stream, request).await
                    {
                        error!(%err, "Could not process request");
                        break;
                    }
                }
                Err(err) => {
                    error!(%err, "Could not parse request");
                    break;
                }
            }
        }
        framed_stream.flush().await.unwrap_or_default();
    }
    .instrument(info_span!("session", ip = %socket.ip()))
    .await;

    Ok(())
}

async fn process_request(
    session: &mut CharacterSession,
    stream: &mut Framed<TcpStream, CharacterCodec>,
    request: Request,
) -> Result<(), anyhow::Error> {
    match request {
        Request::ConnectClient(account_info) => {
            debug!(?account_info, "Client connecting");
            if session.is_authenticated(account_info).await {
                debug!(status = "authenticated");
                stream
                    .send(Response::AccountConnected(account_info.account_id))
                    .await?;
                stream.send(Response::CharacterSlotCount).await?;
                let characters = session.get_characters().await?;
                stream.send(Response::CharacterInfo(characters)).await?;
                stream.send(Response::BannedCharacters).await?;
                let pincode_info = session.get_pincode_info().await?;
                stream.send(Response::PincodeInfo(pincode_info)).await?;
            } else {
                stream.send(Response::Rejected).await?;
            }
        }
        Request::ListCharacters => {
            debug!("Client authenticated, sending character info");
            let characters = session.get_characters().await?;
            stream.send(Response::Characters(characters)).await?;
        }
        Request::KeepAlive => trace!("Keep-alive"),
        Request::SelectCharacter { slot } => {
            debug!("Selecting char slot {}", slot);
            match session.select_character(slot).await {
                Ok(character) => {
                    todo!("handle character selection");
                    todo!("Set character online in OnlineDB");
                }
                Err(err) => {
                    error!(%err, "Could not select character");
                    stream.send(Response::Rejected).await?;
                }
            }
        }
        Request::CreateCharacter(new_character) => {
            debug!("Creating new character");
            let char = session.create_character(new_character).await?;
            debug!(character_id = %char.id, "Created character");
            stream.send(Response::NewCharacterInfo(char)).await?;
        }
        Request::DeleteCharacter => todo!("Handle DeleteCharacter"),
        Request::RequestCharacterDeletion => todo!("Handle RequestCharacterDeletion"),
        Request::AcceptCharacterDeletion => todo!("Handle AcceptCharacterDeletion"),
        Request::CancelCharacterDeletion2 => todo!("Handle CancelCharacterDeletion2"),
        Request::RenameCharacter => todo!("Handle RenameCharacter"),
        Request::RequestCaptcha => todo!("Handle RequestCaptcha"),
        Request::CheckCaptcha => todo!("Handle CheckCaptcha"),
        Request::MoveCharacterSlot => todo!("Handle MoveCharacterSlot"),
        Request::CheckPincode => todo!("Handle CheckPincode"),
        Request::RequestPincode => todo!("Handle RequestPincode"),
        Request::ChangePincode => todo!("Handle ChangePincode"),
        Request::NewPincode => todo!("Handle NewPincode"),
    }
    Ok(())
}
