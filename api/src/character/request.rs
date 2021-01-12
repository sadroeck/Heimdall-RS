use std::convert::TryFrom;

use super::error::Error;

pub enum CharacterCommand {
    KeepAlive,
}

impl TryFrom<u16> for CharacterCommand {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0200 => Ok(CharacterCommand::KeepAlive),
            unknown => Err(Error::InvalidCommand(unknown)),
        }
    }
}

impl CharacterCommand {
    pub fn parse(&self, buf: &[u8]) -> Result<(usize, Request), Error> {
        match self {
            Self::KeepAlive => {
                if buf.len() >= 24 {
                    Ok((24, Request::KeepAlive))
                } else {
                    Err(Error::PacketIncomplete(24 - buf.len()))
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Request {
    KeepAlive,
}
