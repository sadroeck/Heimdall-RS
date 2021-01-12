use async_codec::{Decode, DecodeResult, Encode, EncodeResult};
use std::convert::TryFrom;
use tracing::error;

use super::request::{LoginCommand, Request};
use super::{error::Error, response::Response};

pub struct LoginCodec;

impl Decode for LoginCodec {
    type Item = Request;
    type Error = Error;

    fn decode(&mut self, buffer: &mut [u8]) -> (usize, DecodeResult<Self::Item, Self::Error>) {
        if buffer.len() < 2 {
            return (0, DecodeResult::UnexpectedEnd);
        }

        // Parse command type
        let mut command_buf = [0u8; 2];
        command_buf.copy_from_slice(&buffer[..2]);
        let command = match LoginCommand::try_from(u16::from_le_bytes(command_buf)) {
            Ok(command) => command,
            Err(err) => {
                error!(%err);
                return (0, DecodeResult::Err(err));
            }
        };

        let (request_size, request) = match command.parse(&buffer[2..]) {
            Ok((size, request)) => (size, request),
            Err(Error::PacketIncomplete(_count)) => return (0, DecodeResult::UnexpectedEnd),
            Err(err) => return (0, DecodeResult::Err(err)),
        };
        (request_size + 2, DecodeResult::Ok(request))
    }
}

impl Encode for LoginCodec {
    type Item = Response;
    type Error = Error;

    fn encode(&mut self, item: &Self::Item, buf: &mut [u8]) -> EncodeResult<Self::Error> {
        if buf.len() < 2 {
            return EncodeResult::Overflow(2);
        }
        buf[..2].copy_from_slice(&item.command_code().to_le_bytes());
        match item.serialize(&mut buf[2..]) {
            Ok(size) => EncodeResult::Ok(size + 2),
            Err(buffer_size) => EncodeResult::Overflow(buffer_size + 2),
        }
    }
}
