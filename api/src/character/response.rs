use crate::account::db::AccountId;

pub enum Response {
    AccountConnected(AccountId),
    Rejected,
    CharacterSelectWindowInfo(CharacterSelectWindowInfo),
}

#[derive(Debug, Clone)]
pub struct CharacterSelectWindowInfo {
    pub normal_slots: u8,
    pub vip_slots: u8,
    pub billing_slots: u8,
    pub producible_slots: u8,
    pub valid_slots: u8,
}

impl Response {
    pub fn command_code(&self) -> Option<u16> {
        match self {
            Self::AccountConnected(_) => None,
            Self::Rejected => Some(0x6c),
            Self::CharacterSelectWindowInfo(_) => Some(0x82d),
        }
    }

    pub fn serialize(&self, buf: &mut [u8]) -> Result<usize, usize> {
        match self {
            Self::AccountConnected(account_id) => {
                if buf.len() < 4 {
                    return Err(4);
                }
                buf[..4].copy_from_slice(&account_id.to_le_bytes());
                Ok(4)
            }
            Self::Rejected => {
                if buf.len() < 1 {
                    return Err(1);
                }
                buf[0] = 0;
                Ok(1)
            }
            Self::CharacterSelectWindowInfo(info) => {
                if buf.len() < 27 {
                    return Err(27);
                }
                buf[0..2].copy_from_slice(&29u16.to_le_bytes());
                buf[2] = info.normal_slots;
                buf[3] = info.vip_slots;
                buf[4] = info.billing_slots;
                buf[5] = info.producible_slots;
                buf[6] = info.valid_slots;
                buf[7..27].copy_from_slice(&[0u8; 20]);
                Ok(27)
            }
        }
    }
}
