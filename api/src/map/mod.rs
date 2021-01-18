use crate::codec::EncodeFixed;
use serde::Deserialize;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

pub struct Maps {
    maps: Vec<MapName>,
    name_to_id: HashMap<MapName, u16>,
}

impl Maps {
    pub fn from_file(path: &str) -> Result<Self, anyhow::Error> {
        let file = std::fs::File::open(path)?;
        let maps: Vec<MapName> = serde_yaml::from_reader(file)?;
        let name_to_id = maps
            .iter()
            .enumerate()
            .map(|(i, name)| (name.clone(), i + 1 as u16))
            .collect();
        Ok(Maps { maps, name_to_id })
    }

    pub fn name(&self, id: u16) -> Option<MapName> {
        self.maps.get((id - 1) as usize).cloned()
    }

    pub fn id(&self, name: &MapName) -> Option<u16> {
        self.name_to_id.get(&name).copied()
    }
}
