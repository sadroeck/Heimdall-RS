pub fn parse_word(data: &[u8]) -> u16 {
    let mut buf = [0u8; 2];
    buf.copy_from_slice(&data[..2]);
    u16::from_le_bytes(buf)
}

pub fn parse_long(data: &[u8]) -> u32 {
    let mut buf = [0u8; 4];
    buf.copy_from_slice(&data[..4]);
    u32::from_le_bytes(buf)
}
