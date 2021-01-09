use std::time::SystemTime;

use chrono::{DateTime, Utc};

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
    pub fn error_code(&self) -> u16 {
        match self {
            Self::UnregisteredId(id) => 0,
            Self::IncorrectPassword => 1,
            Self::IdIsExpired => 2,
            Self::RejectedFromServer => 3,
            Self::AccountPermanentlySuspended => 4,
            Self::GameExeNotUpToDate => 5,
            Self::BannedUntil(time) => 6,
            Self::ServerOverpopulated => 7,
            Self::MaxCompanyCapacityReached => 8,
            Self::BannedByDBA => 9,
            Self::EmailNotConfirmed => 10,
            Self::BannedByGM => 11,
            Self::TemporaryBanForDBWork => 12,
            Self::SelfLock => 13,
            Self::GroupNotPermittedV1(usize) => 14,
            Self::GroupNotPermittedV2(usize) => 15,
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
    LoginSuccessV1,
    LoginSuccessV3,
    LoginFailed(LoginFailed),
    LoginAborted(LoginAborted),
}

impl Response {
    pub fn command_code(&self) -> u32 {
        match self {
            Response::LoginSuccessV1 => 0x69,
            Response::LoginSuccessV3 => 0xac4,
            Response::LoginFailed(_) => 0x83e,
            Response::LoginAborted(_) => 0x81,
        }
    }

    pub fn serialize(&self, buf: &mut [u8]) -> Result<usize, usize> {
        match self {
            Self::LoginSuccessV1 => todo!("Serialize login success v1"),
            Self::LoginSuccessV3 => todo!("Serialize login success v3"),
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
