#[derive(Debug)]
pub enum LoginCredentials {
    OTP {
        client_type: u8,
        account_name: String,
        password: Vec<u8>,
    },
    Hashed {
        client_type: u8,
        username: String,
        password: [u8; 16],
    },
    ClearText {
        client_type: u8,
        username: String,
        password: String,
    },
}
