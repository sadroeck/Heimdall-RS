pub enum Response {
    LoginSuccessV1,
}

impl Response {
    pub fn command_code(&self) -> u16 {
        match self {
            Response::LoginSuccessV1 => 0x69,
        }
    }

    pub fn serialize(&self, buf: &mut [u8]) -> Result<usize, usize> {
        match self {
            Self::LoginSuccessV1 => {
                if buf.len() < 20 {
                    return Err(20);
                }
                buf[..4].copy_from_slice(&[0u8; 4]);
            }
        }
        todo!("handle serialization");
    }
}
