use std::convert::TryFrom;

use crate::{account::mmo_account::Sex, error::PacketError, utils::parse_long};

#[derive(Debug, Copy, Clone)]
pub enum CharacterCommand {
    ConnectClient,
    ListCharacters,
    SelectCharacter,
    CreateCharacter,
    DeleteCharacter,
    RequestCharacterDeletion,
    AcceptCharacterDeletion,
    CancelCharacterDeletion2,
    RenameCharacter,
    RequestCaptcha,
    CheckCaptcha,
    MoveCharacterSlot,
    KeepAlive,
    CheckPincode,
    RequestPincode,
    ChangePincode,
    NewPincode,
}

impl TryFrom<u16> for CharacterCommand {
    type Error = PacketError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x65 => Ok(CharacterCommand::ConnectClient),
            0xa39 | 0x970 | 0x67 => Ok(CharacterCommand::CreateCharacter),
            0x66 => Ok(CharacterCommand::SelectCharacter),
            0x9a1 => Ok(CharacterCommand::ListCharacters),
            0x68 | 0x1fb => Ok(CharacterCommand::DeleteCharacter),
            0x28d => Ok(CharacterCommand::RenameCharacter),
            0x7e5 => Ok(CharacterCommand::RequestCaptcha),
            0x7e7 => Ok(CharacterCommand::CheckCaptcha),
            0x8d4 => Ok(CharacterCommand::MoveCharacterSlot),
            0x187 => Ok(CharacterCommand::KeepAlive),
            0x827 => Ok(CharacterCommand::RequestCharacterDeletion),
            0x829 => Ok(CharacterCommand::AcceptCharacterDeletion),
            0x82b => Ok(CharacterCommand::CancelCharacterDeletion2),
            0x8b8 => Ok(CharacterCommand::CheckPincode),
            0x8c5 => Ok(CharacterCommand::RequestPincode),
            0x8be => Ok(CharacterCommand::ChangePincode),
            0x8ba => Ok(CharacterCommand::NewPincode),
            unknown => Err(PacketError::InvalidCommand(unknown)),
        }
    }
}

impl CharacterCommand {
    pub fn parse(&self, buf: &[u8]) -> Result<(usize, Request), PacketError> {
        match self {
            Self::ConnectClient => {
                if buf.len() >= 15 {
                    let account_info = AccountInfo {
                        account_id: parse_long(&buf[..4]),
                        authentication_code: parse_long(&buf[4..8]),
                        user_level: parse_long(&buf[8..12]),
                        sex: Sex::try_from(buf[14])?,
                    };
                    Ok((15, Request::ConnectClient(account_info)))
                } else {
                    Err(PacketError::PacketIncomplete(15 - buf.len()))
                }
            }
            Self::ListCharacters => Ok((2, Request::ListCharacters)),
            Self::SelectCharacter => todo!("parse SelectCharacter"),
            Self::CreateCharacter => todo!("parse CreateCharacter"),
            Self::DeleteCharacter => todo!("parse DeleteCharacter"),
            Self::RequestCharacterDeletion => todo!("parse RequestCharacterDeletion"),
            Self::AcceptCharacterDeletion => todo!("parse AcceptCharacterDeletion"),
            Self::CancelCharacterDeletion2 => todo!("parse CancelCharacterDeletion2"),
            Self::RenameCharacter => todo!("parse RenameCharacter"),
            Self::RequestCaptcha => todo!("parse RequestCaptcha"),
            Self::CheckCaptcha => todo!("parse CheckCaptcha"),
            Self::MoveCharacterSlot => todo!("parse MoveCharacterSlot"),
            Self::KeepAlive => {
                if buf.len() >= 4 {
                    Ok((4, Request::KeepAlive))
                } else {
                    Err(PacketError::PacketIncomplete(4 - buf.len()))
                }
            }
            Self::CheckPincode => todo!("parse CheckPincode"),
            Self::RequestPincode => todo!("parse RequestPincode"),
            Self::ChangePincode => todo!("parse ChangePincode"),
            Self::NewPincode => todo!("parse NewPincode"),
        }
    }
}

#[derive(Debug)]
pub enum Request {
    ConnectClient(AccountInfo),
    ListCharacters,
    SelectCharacter,
    CreateCharacter,
    DeleteCharacter,
    RequestCharacterDeletion,
    AcceptCharacterDeletion,
    CancelCharacterDeletion2,
    RenameCharacter,
    RequestCaptcha,
    CheckCaptcha,
    MoveCharacterSlot,
    KeepAlive,
    CheckPincode,
    RequestPincode,
    ChangePincode,
    NewPincode,
}

#[derive(Debug, Clone, Copy)]
pub struct AccountInfo {
    pub account_id: u32,
    pub authentication_code: u32,
    pub user_level: u32,
    pub sex: Sex,
}
