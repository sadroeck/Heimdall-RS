use crate::codec::EncodeFixed;
use std::cmp::min;

// TODO: replace with MapDB from file
pub fn map_name(_id: u16) -> MapName {
    // NEEDS .gat suffix
    MapName(String::from("prt_fild08.gat"))
}

#[derive(Debug, Clone)]
pub struct MapName(String);

impl EncodeFixed for MapName {
    const SIZE: usize = 16;
    fn encode(&self, buf: &mut [u8]) {
        let name = self.0.as_bytes();
        let len = min(Self::SIZE, name.len());
        buf[..len].copy_from_slice(&name[..len]);
        buf[len] = b'\0';
    }
}
