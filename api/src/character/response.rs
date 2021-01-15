use crate::account::db::AccountId;
use crate::codec::{ClientTcpCodec, RagnarokCodec};
use crate::{
    character::{Character, MAX_CHARACTERS_PER_ACCOUNT},
    pincode::PincodeInfo,
};

pub enum Response {
    AccountConnected(AccountId),
    Rejected,
    CharacterSlotCount,
    CharacterInfo(Vec<Character>),
    Characters(Vec<Character>),
    CharacterPagesAvailable(u32),
    BannedCharacters,
    PincodeInfo(PincodeInfo),
}

impl Response {
    pub fn command_code(&self) -> Option<u16> {
        match self {
            Self::AccountConnected(_) => None,
            Self::Rejected => Some(0x6c),
            Self::CharacterSlotCount => Some(0x82d),
            Self::CharacterInfo(_) => Some(0x6b),
            Self::Characters(_) => Some(0x99d),
            Self::CharacterPagesAvailable(_) => Some(0x9a0),
            Self::BannedCharacters => Some(0x20d),
            Self::PincodeInfo { .. } => Some(0x8b9),
        }
    }

    pub fn serialize(&self, buf: &mut [u8]) -> Result<usize, usize> {
        let mut codec = ClientTcpCodec::new(buf);
        match self {
            Self::AccountConnected(account_id) => {
                if codec.capacity() < 4 {
                    return Err(4);
                }
                codec.encode(account_id);
            }
            Self::Rejected => {
                if codec.capacity() < 1 {
                    return Err(1);
                }
                codec.encode(&0u8);
            }
            Self::CharacterSlotCount => {
                if codec.capacity() < 27 {
                    return Err(27);
                }
                codec.encode(&29u16);
                codec.encode(&(MAX_CHARACTERS_PER_ACCOUNT as u8));
                codec.encode(&(MAX_CHARACTERS_PER_ACCOUNT as u8));
                codec.encode(&0u8);
                codec.encode(&(MAX_CHARACTERS_PER_ACCOUNT as u8));
                codec.encode(&(MAX_CHARACTERS_PER_ACCOUNT as u8));
                codec.padding(20);
            }
            Self::CharacterInfo(characters) => {
                let frame_size = 25 + (characters.len() * Character::FRAME_SIZE);
                if codec.capacity() < frame_size {
                    return Err(frame_size);
                }
                codec.encode(&(frame_size as u16 + 2));
                codec.encode(&(MAX_CHARACTERS_PER_ACCOUNT as u8));
                codec.encode(&(MAX_CHARACTERS_PER_ACCOUNT as u8));
                codec.encode(&(MAX_CHARACTERS_PER_ACCOUNT as u8));
                codec.padding(20);
                // Unknown bytes
                characters
                    .iter()
                    .for_each(|character| codec.encode_struct(character));
            }
            Self::Characters(characters) => {
                let frame_size = 2
                    + (characters.len() * Character::FRAME_SIZE)
                    + if characters.len() == 3 { 4 } else { 0 };
                if codec.capacity() < frame_size {
                    return Err(frame_size);
                }
                codec.encode(&(frame_size as u16 + 2));
                characters
                    .iter()
                    .for_each(|character| codec.encode_struct(character));
                if characters.len() == 3 {
                    codec.encode(&self.command_code().unwrap());
                    codec.encode(&4u16);
                }
            }
            Self::CharacterPagesAvailable(count) => {
                if codec.capacity() < 4 {
                    return Err(4);
                }
                codec.encode(count);
            }
            Self::BannedCharacters => {
                if codec.capacity() < 2 {
                    return Err(2);
                }
                // TODO: implement banned list
                codec.encode(&4u16);
            }
            Self::PincodeInfo(PincodeInfo { status, account_id }) => {
                if codec.capacity() < 10 {
                    return Err(10);
                }
                codec.encode(&(fastrand::u16(..) as u32));
                codec.encode(account_id);
                codec.encode(&(*status as u16));
            }
        }
        Ok(codec.len())
    }
}
