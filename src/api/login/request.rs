use std::convert::TryFrom;

use super::{error::Error, LoginCredentials};

pub enum LoginCommand {
    KeepAlive,
    UpdateClientHash,
    ClientLoginRawPassV1,
    ClientLoginRawPassV2,
    ClientLoginRawPassV3,
    ClientLoginHashedPassV1,
    ClientLoginHashedPassV2,
    ClientLoginHashedPassV3,
    ClientLoginHashedPassV4,
    CodeKey,
    OneTimePassLogin,
    CharConnect,
}

impl TryFrom<u16> for LoginCommand {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0200 => Ok(LoginCommand::KeepAlive),
            0x0204 => Ok(LoginCommand::UpdateClientHash),
            0x0064 => Ok(LoginCommand::ClientLoginRawPassV1),
            0x0277 => Ok(LoginCommand::ClientLoginRawPassV2),
            0x02b0 => Ok(LoginCommand::ClientLoginRawPassV3),
            0x01dd => Ok(LoginCommand::ClientLoginHashedPassV1),
            0x01fa => Ok(LoginCommand::ClientLoginHashedPassV2),
            0x027c => Ok(LoginCommand::ClientLoginHashedPassV3),
            0x0825 => Ok(LoginCommand::ClientLoginHashedPassV4),
            0x01db => Ok(LoginCommand::CodeKey),
            0x0acf => Ok(LoginCommand::OneTimePassLogin),
            0x2710 => Ok(LoginCommand::CharConnect),
            unknown => Err(Error::InvalidCommand(unknown)),
        }
    }
}

impl LoginCommand {
    pub fn parse(&self, buf: &[u8]) -> Result<(usize, Request), Error> {
        match self {
            Self::KeepAlive => {
                if buf.len() >= 24 {
                    Ok((24, Request::KeepAlive))
                } else {
                    Err(Error::PacketIncomplete(24 - buf.len()))
                }
            }
            Self::UpdateClientHash => {
                if buf.len() >= 16 {
                    let mut hash = [0u8; 16];
                    hash.copy_from_slice(&buf[..16]);
                    Ok((16, Request::UpdateClientHash(hash)))
                } else {
                    Err(Error::PacketIncomplete(16 - buf.len()))
                }
            }
            Self::ClientLoginRawPassV1 => {
                if buf.len() >= 53 {
                    let credentials = parse_cleartext_credentials(&buf);
                    Ok((53, Request::ClientLogin(credentials)))
                } else {
                    Err(Error::PacketIncomplete(53 - buf.len()))
                }
            }
            Self::ClientLoginRawPassV2 => {
                if buf.len() >= 82 {
                    let credentials = parse_cleartext_credentials(&buf);
                    Ok((82, Request::ClientLogin(credentials)))
                } else {
                    Err(Error::PacketIncomplete(82 - buf.len()))
                }
            }
            Self::ClientLoginRawPassV3 => {
                if buf.len() >= 83 {
                    let credentials = parse_cleartext_credentials(&buf);
                    Ok((83, Request::ClientLogin(credentials)))
                } else {
                    Err(Error::PacketIncomplete(83 - buf.len()))
                }
            }
            Self::ClientLoginHashedPassV1 => {
                if buf.len() >= 45 {
                    let credentials = parse_hashed_credentials(&buf);
                    Ok((45, Request::ClientLogin(credentials)))
                } else {
                    Err(Error::PacketIncomplete(45 - buf.len()))
                }
            }
            Self::ClientLoginHashedPassV2 => {
                if buf.len() >= 46 {
                    let credentials = parse_hashed_credentials(&buf);
                    Ok((46, Request::ClientLogin(credentials)))
                } else {
                    Err(Error::PacketIncomplete(46 - buf.len()))
                }
            }
            Self::ClientLoginHashedPassV3 => {
                if buf.len() >= 58 {
                    let credentials = parse_hashed_credentials(&buf);
                    Ok((58, Request::ClientLogin(credentials)))
                } else {
                    Err(Error::PacketIncomplete(58 - buf.len()))
                }
            }
            Self::ClientLoginHashedPassV4 => {
                todo!("Parse OTPs");
            }
            Self::CodeKey => {
                todo!("Parse CodeKey");
            }
            Self::OneTimePassLogin => {
                todo!("Parse OneTimePassLogin");
            }
            Self::CharConnect => {
                todo!("Parse CharConnect");
            }
        }
    }
}

#[derive(Debug)]
pub enum Request {
    KeepAlive,
    UpdateClientHash([u8; 16]),
    ClientLogin(LoginCredentials),
    CodeKey,
    OneTimeToken,
    ConnectChar,
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
