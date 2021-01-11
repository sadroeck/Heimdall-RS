use std::{net::Ipv4Addr, time::SystemTime};

use chrono::{Date, Utc};

use super::db::{AccountId, UserId};

pub(crate) const MAX_CHARS: u8 = 10;
pub(crate) const PINCODE_LENGTH: usize = 4;
pub(crate) const WEB_AUTH_TOKEN_LENGTH: usize = 16;

#[derive(Debug, Clone)]
pub struct MmoAccount {
    pub(crate) account_id: u32,
    pub(crate) user_id: UserId,
    pub(crate) password: Password,
    /// Gender
    pub(crate) sex: Sex,
    /// Email (by default a@a.com)
    pub(crate) email: String,
    /// Player group ID
    pub(crate) group_id: Option<usize>,
    /// this accounts maximum character slots (maximum is limited to MAX_CHARS define in char server)
    pub(crate) char_slots: u8,
    /// packet 0x006a value + 1 (0: compte OK)
    pub(crate) state: AccountState,
    /// number of successful auth attempts
    pub(crate) login_count: isize,
    /// date+time of last successful login
    pub(crate) lastlogin: SystemTime,
    /// save of last IP of connection
    pub(crate) last_ip: Ipv4Addr,
    /// assigned birth date (format: YYYY-MM-DD)
    pub(crate) birth_date: Date<Utc>,
    /// pincode system
    pub(crate) pincode: [u8; PINCODE_LENGTH],
    /// last time of pincode change
    pub(crate) pincode_change: SystemTime,
    /// web authentication token (randomized on each login)
    pub(crate) web_auth_token: [u8; WEB_AUTH_TOKEN_LENGTH],
}

#[derive(Debug, Clone, Copy)]
pub enum Sex {
    Male,
    Female,
    Server,
}

impl Into<u8> for Sex {
    fn into(self) -> u8 {
        match self {
            Self::Male => 1,
            Self::Female => 0,
            Self::Server => 2,
        }
    }
}

// TODO: Remove derived Debug for passwords
#[derive(Clone, Debug)]
pub enum Password {
    Cleartext(String),
    MD5Hashed([u8; 16]),
    // Only set when initializing account
    None,
}

impl Default for MmoAccount {
    fn default() -> Self {
        Self {
            account_id: 0,
            user_id: UserId::default(),
            password: Password::None,
            sex: Sex::Male,
            email: String::from("a@a.com"),
            group_id: None,
            char_slots: MAX_CHARS,
            state: AccountState::Normal,
            login_count: 0,
            lastlogin: SystemTime::now(),
            last_ip: Ipv4Addr::LOCALHOST,
            birth_date: Utc::today(),
            pincode: [0u8; PINCODE_LENGTH],
            pincode_change: SystemTime::now(),
            web_auth_token: [0u8; WEB_AUTH_TOKEN_LENGTH],
        }
    }
}

impl MmoAccount {
    pub fn new(id: AccountId) -> Self {
        Self {
            account_id: id,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AccountState {
    Normal,
    Banned(SystemTime),
    ExpireOn(SystemTime),
}
