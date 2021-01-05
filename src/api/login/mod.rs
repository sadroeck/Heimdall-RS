use async_std::io;
use async_std::io::{Read, ReadExt};
use tracing::error;

pub use credentials::LoginCredentials;

mod credentials;

const KEEP_ALIVE: u16 = 0x0200;
const UPDATE_CLIENT_HASH: u16 = 0x0204;
const CLIENT_LOGIN_RAW_PASS_V1: u16 = 0x0064;
const CLIENT_LOGIN_RAW_PASS_V2: u16 = 0x0277;
const CLIENT_LOGIN_RAW_PASS_V3: u16 = 0x02b0;
const CLIENT_LOGIN_HASHED_PASS_V1: u16 = 0x01dd;
const CLIENT_LOGIN_HASHED_PASS_V2: u16 = 0x01fa;
const CLIENT_LOGIN_HASHED_PASS_V3: u16 = 0x027c;
const CLIENT_LOGIN_HASHED_PASS_V4: u16 = 0x0825;
const CODE_KEY: u16 = 0x01db;
const OTP_LOGIN: u16 = 0x0acf;
const CHAR_CONNECT: u16 = 0x2710;

#[derive(Debug)]
pub enum Request {
    KeepAlive,
    UpdateClientHash([u8; 16]),
    ClientLogin(LoginCredentials),
    CodeKey,
    OneTimeToken,
    ConnectChar,
}

impl Request {
    pub async fn parse(mut reader: impl Read + Unpin) -> Result<Self, io::Error> {
        let mut command_buf = [0u8; 2];
        reader.read_exact(&mut command_buf).await?;
        match u16::from_le_bytes(command_buf) {
            KEEP_ALIVE => {
                let mut buf = [0u8; 24];
                reader.read_exact(&mut buf).await?;
                Ok(Self::KeepAlive)
            }
            UPDATE_CLIENT_HASH => {
                let mut buf = [0u8; 16];
                reader.read_exact(&mut buf).await?;
                Ok(Self::UpdateClientHash(buf))
            }
            CLIENT_LOGIN_RAW_PASS_V1 => {
                let mut buf = [0u8; 53];
                reader.read_exact(&mut buf).await?;
                Ok(Self::ClientLogin(parse_cleartext_credentials(&buf)))
            }
            CLIENT_LOGIN_RAW_PASS_V2 => {
                let mut buf = [0u8; 82];
                reader.read_exact(&mut buf).await?;
                Ok(Self::ClientLogin(parse_cleartext_credentials(&buf)))
            }
            CLIENT_LOGIN_RAW_PASS_V3 => {
                let mut buf = [0u8; 83];
                reader.read_exact(&mut buf).await?;
                Ok(Self::ClientLogin(parse_cleartext_credentials(&buf)))
            }
            CLIENT_LOGIN_HASHED_PASS_V1 => {
                let mut buf = [0u8; 45];
                reader.read_exact(&mut buf).await?;
                Ok(Self::ClientLogin(parse_hashed_credentials(&buf)))
            }
            CLIENT_LOGIN_HASHED_PASS_V2 => {
                let mut buf = [0u8; 46];
                reader.read_exact(&mut buf).await?;
                Ok(Self::ClientLogin(parse_hashed_credentials(&buf)))
            }
            CLIENT_LOGIN_HASHED_PASS_V3 => {
                let mut buf = [0u8; 58];
                reader.read_exact(&mut buf).await?;
                Ok(Self::ClientLogin(parse_hashed_credentials(&buf)))
            }
            CLIENT_LOGIN_HASHED_PASS_V4 => {
                todo!("Parse OTPs");
            }
            CODE_KEY => Ok(Self::CodeKey),
            OTP_LOGIN => Ok(Self::OneTimeToken),
            CHAR_CONNECT => Ok(Self::ConnectChar),
            unknown => {
                error!("Unknown command {}", unknown);
                todo!("implement me");
            }
        }
    }
}

fn parse_cleartext_credentials(data: &[u8]) -> LoginCredentials {
    let mut username = [0u8; 23 + 1];
    copy_zero_terminated_buffer(&mut username, &data[4..4 + 23 + 1]);
    let mut password = [0u8; 24];
    copy_zero_terminated_buffer(&mut password, &data[28..28 + 24]);
    LoginCredentials::ClearText {
        client_type: data[data.len() - 1],
        username,
        password,
    }
}

fn parse_hashed_credentials(data: &[u8]) -> LoginCredentials {
    let mut username = [0u8; 23 + 1];
    copy_zero_terminated_buffer(&mut username, &data[4..4 + 23 + 1]);
    let mut password = [0u8; 16];
    copy_zero_terminated_buffer(&mut password, &data[28..28 + 16]);
    LoginCredentials::Hashed {
        client_type: data[data.len() - 1],
        username,
        password,
    }
}

fn copy_zero_terminated_buffer(buffer: &mut [u8], string_bytes: &[u8]) {
    for (i, value) in string_bytes.iter().enumerate() {
        if *value == 0 {
            break;
        }
        buffer[i] = *value;
    }
}
