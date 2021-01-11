// pub(crate) mod character;
pub(crate) mod character;
pub(crate) mod login;

struct RequestBuffer(pub(crate) [u8; 0xFFFF]);

impl Default for RequestBuffer {
    fn default() -> Self {
        Self([0u8; 0xFFFF])
    }
}
