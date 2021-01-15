use std::convert::TryFrom;

use async_codec::{Decode, DecodeResult, Encode, EncodeResult};
use tracing::error;

use crate::{error::PacketError, utils::parse_word};

use super::{
    request::{CharacterCommand, Request},
    response::Response,
};

pub struct CharacterCodec;

impl Decode for CharacterCodec {
    type Item = Request;
    type Error = PacketError;

    fn decode(&mut self, buffer: &mut [u8]) -> (usize, DecodeResult<Self::Item, Self::Error>) {
        if buffer.len() < 2 {
            return (0, DecodeResult::UnexpectedEnd);
        }

        // Parse command type
        let command = match CharacterCommand::try_from(parse_word(&buffer[..2])) {
            Ok(command) => command,
            Err(err) => {
                error!(%err);
                return (0, DecodeResult::Err(err));
            }
        };

        match command.parse(&buffer[2..]) {
            Ok((size, request)) => (size + 2, DecodeResult::Ok(request)),
            Err(PacketError::PacketIncomplete(_count)) => (0, DecodeResult::UnexpectedEnd),
            Err(err) => {
                error!("Could not decode packet {:?}", command);
                (0, DecodeResult::Err(err))
            }
        }
    }
}

impl Encode for CharacterCodec {
    type Item = Response;
    type Error = PacketError;

    fn encode(&mut self, item: &Self::Item, buf: &mut [u8]) -> EncodeResult<Self::Error> {
        if buf.len() < 2 {
            return EncodeResult::Overflow(2);
        }
        let offset = if let Some(command_code) = item.command_code() {
            buf[..2].copy_from_slice(&command_code.to_le_bytes());
            2
        } else {
            0
        };
        match item.serialize(&mut buf[offset..]) {
            Ok(size) => EncodeResult::Ok(size + offset),
            Err(buffer_size) => EncodeResult::Overflow(buffer_size + offset),
        }
    }
}
