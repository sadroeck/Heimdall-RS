use std::time::SystemTime;

use chrono::{DateTime, Utc};
use stackvec::StackVec;

use crate::{account::mmo_account::Sex, character::ServerInfo as CharacterServerInfo};

const BAN_TIME_FORMAT: &str = "%Y-%M-%D %H:%M";

#[derive(Debug, thiserror::Error)]
pub enum LoginFailed {
    #[error("UnregisteredId")]
    UnregisteredId(String),
    #[error("Incorrect Password")]
    IncorrectPassword,
    #[error("This ID is expired")]
    IdIsExpired,
    #[error("Rejected from Server")]
    RejectedFromServer,
    #[error("You have been blocked by the GM Team")]
    AccountPermanentlySuspended,
    #[error("Your Game's EXE file is not the latest version")]
    GameExeNotUpToDate,
    #[error("You are prohibited to log in until {0:?}")]
    BannedUntil(SystemTime),
    #[error("Server is jammed due to over populated")]
    ServerOverpopulated,
    #[error("No more accounts may be connected from this company")]
    MaxCompanyCapacityReached,
    #[error("MSI_REFUSE_BAN_BY_DBA")]
    BannedByDBA,
    #[error("MSI_REFUSE_EMAIL_NOT_CONFIRMED")]
    EmailNotConfirmed,
    #[error("MSI_REFUSE_BAN_BY_GM")]
    BannedByGM,
    #[error("MSI_REFUSE_TEMP_BAN_FOR_DBWORK")]
    TemporaryBanForDBWork,
    #[error("MSI_REFUSE_SELF_LOCK")]
    SelfLock,
    #[error("MSI_REFUSE_NOT_PERMITTED_GROUP")]
    GroupNotPermittedV1(usize),
    #[error("MSI_REFUSE_NOT_PERMITTED_GROUP")]
    GroupNotPermittedV2(usize),
    #[error("This ID has been totally erased")]
    IdErased,
    #[error("Login information remains at %s")]
    // TODO: adjust name
    LoginInfoRelocated,
    #[error("Account has been locked for a hacking investigation. Please contact the GM Team for more information")]
    LockedForHackingInvestigation,
    #[error("This account has been temporarily prohibited from login due to a bug-related investigation")]
    LockedForBugInvestigation,
    #[error(
        "This character is being deleted. Login is temporarily unavailable for the time being"
    )]
    DeleteInProgressV1,
    #[error(
        "This character is being deleted. Login is temporarily unavailable for the time being"
    )]
    DeleteInProgressV2,
}

impl LoginFailed {
    pub fn error_code(&self) -> u32 {
        match self {
            Self::UnregisteredId(_) => 0,
            Self::IncorrectPassword => 1,
            Self::IdIsExpired => 2,
            Self::RejectedFromServer => 3,
            Self::AccountPermanentlySuspended => 4,
            Self::GameExeNotUpToDate => 5,
            Self::BannedUntil(_) => 6,
            Self::ServerOverpopulated => 7,
            Self::MaxCompanyCapacityReached => 8,
            Self::BannedByDBA => 9,
            Self::EmailNotConfirmed => 10,
            Self::BannedByGM => 11,
            Self::TemporaryBanForDBWork => 12,
            Self::SelfLock => 13,
            Self::GroupNotPermittedV1(_) => 14,
            Self::GroupNotPermittedV2(_) => 15,
            Self::IdErased => 99,
            Self::LoginInfoRelocated => 100,
            Self::LockedForHackingInvestigation => 101,
            Self::LockedForBugInvestigation => 102,
            Self::DeleteInProgressV1 => 103,
            Self::DeleteInProgressV2 => 104,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoginAborted {
    #[error("The server is closed")]
    ServerClosed,
    #[error("Someone has already logged in with this id")]
    AlreadyLoggedIn,
    #[error("The user with this id is already online")]
    AlreadyOnline,
}

impl LoginAborted {
    pub fn error_code(&self) -> u8 {
        match self {
            Self::ServerClosed => 1,
            Self::AlreadyLoggedIn => 2,
            Self::AlreadyOnline => 8,
        }
    }
}

pub enum Response {
    LoginSuccessV1(CharacterSelectionInfo),
    LoginSuccessV3(CharacterSelectionInfo),
    LoginFailed(LoginFailed),
    LoginAborted(LoginAborted),
}

impl Response {
    pub fn command_code(&self) -> u16 {
        match self {
            Response::LoginSuccessV1(_) => 0x69,
            Response::LoginSuccessV3(_) => 0xac4,
            Response::LoginFailed(_) => 0x83e,
            Response::LoginAborted(_) => 0x81,
        }
    }

    pub fn serialize(&self, buf: &mut [u8]) -> Result<usize, usize> {
        match self {
            Self::LoginSuccessV1(_info) => {
                todo!("Handle LoginSuccessV1");
            }
            Self::LoginSuccessV3(info) => {
                let msg_len = 64 + info.char_servers.len() * 160;
                if buf.len() < msg_len {
                    return Err(msg_len);
                }
                buf[..2].copy_from_slice(&(msg_len as u16).to_le_bytes());
                buf[2..6].copy_from_slice(&info.authentication_code.to_le_bytes());
                buf[6..10].copy_from_slice(&info.account_id.to_le_bytes());
                buf[10..14].copy_from_slice(&info.user_level.to_le_bytes());
                // unused (last_login_ip + last_login_time)
                buf[14..44].copy_from_slice(&[0u8; 30]);
                buf[44] = info.sex.into();
                buf[45..61].copy_from_slice(&info.web_auth_token);
                buf[61] = 0;
                let header_offset = 62;
                for (i, server) in info.char_servers.iter().enumerate() {
                    let offset = header_offset + (i * 160);
                    let ip: u32 = server.ip_addr.into();
                    buf[offset..offset + 4].copy_from_slice(&ip.to_be_bytes());
                    buf[offset + 4..offset + 6].copy_from_slice(&server.port.to_le_bytes());
                    buf[offset + 6..offset + 6 + server.name.len()]
                        .copy_from_slice(&server.name.as_bytes());
                    buf[offset + 6 + server.name.len()] = b'\0';
                    let server_activity: u16 = server.server_activity.into();
                    buf[offset + 26..offset + 28].copy_from_slice(&server_activity.to_le_bytes());
                    let server_type: u16 = server.server_type.into();
                    buf[offset + 28..offset + 30].copy_from_slice(&server_type.to_be_bytes());
                    buf[offset + 30..offset + 32].copy_from_slice(&[0u8; 2]);
                    buf[offset + 32..offset + 160].copy_from_slice(&[0u8; 128]);
                }

                Ok(msg_len)
            }
            Self::LoginFailed(failure) => {
                if buf.len() < 20 {
                    return Err(20);
                }
                let failure_code = failure.error_code();
                buf[..4].copy_from_slice(&failure_code.to_le_bytes());
                if let LoginFailed::BannedUntil(time) = failure {
                    let time_str = DateTime::<Utc>::from(*time)
                        .format(BAN_TIME_FORMAT)
                        .to_string();
                    let time_str_bytes = time_str.as_bytes();
                    buf[4..4 + time_str_bytes.len()].copy_from_slice(&time_str_bytes);
                    if time_str_bytes.len() < 20 {
                        buf[4 + time_str_bytes.len()] = b'\0';
                    }
                    Ok(4 + time_str_bytes.len())
                } else {
                    // Fill with zero bytes
                    buf[4..24].copy_from_slice(&[0u8; 20]);
                    Ok(24)
                }
            }
            Self::LoginAborted(aborted) => {
                if !buf.is_empty() {
                    buf[0] = aborted.error_code();
                    Ok(1)
                } else {
                    Err(1)
                }
            }
        }
    }
}

pub struct CharacterSelectionInfo {
    pub account_id: u32,
    pub authentication_code: u32,
    pub user_level: u32,
    pub sex: Sex,
    pub web_auth_token: [u8; 16],
    pub char_servers: StackVec<[CharacterServerInfo; 5]>,
}
