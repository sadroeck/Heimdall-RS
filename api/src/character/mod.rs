use std::cmp::min;
use std::net::Ipv4Addr;

use crate::account::mmo_account::Sex;
use crate::character::attributes::{
    Appearance, Class, Currency, Equipment, Experience, Grouping, Location, MercenaryGuildRank,
    Relationship, Settings, Skill, Stats, Status,
};
use crate::codec::EncodeFixed;
use crate::codec::RagnarokCodec;
use crate::error::PacketError;
use crate::{account::db::AccountId, codec::EncodeStruct};
pub use client::TcpClient;
pub use codec::*;
use db::CharacterId;
pub use request::*;
pub use response::*;
pub use server::TcpServer;
use std::convert::TryFrom;
use std::time::SystemTime;

pub mod attributes;
mod client;
mod codec;
pub mod db;
mod request;
mod response;
mod server;

/// The maximum number of characters per account
pub const MAX_CHARACTERS_PER_ACCOUNT: usize = 12;
pub const DEFAULT_WALK_SPEED: u16 = 150;
const OPTIONS_INCOMPATIBLE_WITH_WEAPON: u32 = 0x20
    | 0x80000
    | 0x100000
    | 0x200000
    | 0x400000
    | 0x800000
    | 0x1000000
    | 0x2000000
    | 0x4000000
    | 0x8000000;
pub const CHARACTER_SLOT_MOVE_ENABLED: bool = true;
pub const CHARACTER_RENAME_ENABLED: bool = true;
/// The maximum number of inventory slots
pub const MAX_INVENTORY_SIZE: usize = 100;

#[async_trait::async_trait]
pub trait CharacterServer {
    fn info(&self) -> ServerInfo;
}

#[derive(Clone)]
pub struct ServerInfo {
    pub(crate) ip_addr: Ipv4Addr,
    pub(crate) port: u16,
    pub(crate) name: String,
    pub(crate) active_users: usize,
    pub(crate) server_type: ServerType,
    pub(crate) server_activity: ServerActivity,
}

#[derive(Clone, Copy, Debug)]
pub enum ServerActivity {
    /// No status color
    Hidden,
    /// Status color = green
    Smooth,
    /// Status color = yellow
    Normal,
    /// Status color = red
    Busy,
    /// Status color = purple
    Crowded,
}

impl Into<u16> for ServerActivity {
    fn into(self) -> u16 {
        match self {
            Self::Hidden => 4,
            Self::Smooth => 0,
            Self::Normal => 1,
            Self::Busy => 2,
            Self::Crowded => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ServerType {
    Normal,
    Maintenance,
    AdultOnly,
    Paying,
    F2P,
}

impl Into<u16> for ServerType {
    fn into(self) -> u16 {
        match self {
            Self::Normal => 0,
            Self::Maintenance => 1,
            Self::AdultOnly => 2,
            Self::Paying => 3,
            Self::F2P => 4,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CharacterName(String);

impl EncodeFixed for CharacterName {
    const SIZE: usize = 24;

    fn encode(&self, buf: &mut [u8]) {
        let name = self.0.as_bytes();
        let len = min(Self::SIZE, name.len());
        buf[..len].copy_from_slice(&name[..len]);
        buf[len] = b'\0';
    }
}

impl TryFrom<&[u8]> for CharacterName {
    type Error = PacketError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < Self::SIZE {
            return Err(PacketError::PacketIncomplete(Self::SIZE));
        }
        Ok(CharacterName(
            String::from_utf8_lossy(
                &value[..Self::SIZE]
                    .split(|char| *char == b'\0')
                    .next()
                    .unwrap_or_default(),
            )
            .to_string(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Character {
    pub id: CharacterId,
    pub account_id: AccountId,
    pub slot: u16,
    pub sex: Sex,
    pub name: CharacterName,
    pub relationship: Relationship,
    pub experience: Experience,
    pub currency: Currency,
    pub class: Class,
    pub stats: Stats,
    pub status: Status,
    pub appearance: Appearance,
    pub grouping: Grouping,
    pub equipment: Equipment,
    pub mercenary_guild_rank: MercenaryGuildRank,
    pub location: Location,
    pub skills: Vec<Skill>,
    pub settings: Settings,
}

impl Character {
    pub const FRAME_SIZE: usize = 155;

    pub fn new(id: CharacterId, account_id: AccountId) -> Self {
        Self {
            id,
            account_id,
            slot: 0,
            sex: Sex::Female,
            name: Default::default(),
            relationship: Default::default(),
            experience: Default::default(),
            currency: Default::default(),
            class: Class::Novice,
            stats: Default::default(),
            status: Default::default(),
            appearance: Default::default(),
            grouping: Default::default(),
            equipment: Default::default(),
            mercenary_guild_rank: Default::default(),
            location: Default::default(),
            skills: vec![],
            settings: Default::default(),
        }
    }
}

impl EncodeStruct for Character {
    fn encode<C: RagnarokCodec>(&self, codec: &mut C) {
        codec.encode(&self.id);
        codec.encode(&self.experience.base_exp);
        codec.encode(&self.currency.zeny);
        codec.encode(&self.experience.job_exp);
        codec.encode(&(self.experience.job_level as u32));
        codec.padding(8);
        codec.encode(&(self.status.option & !0x40));
        codec.encode(&self.status.karma.unwrap_or_default());
        codec.encode(&self.status.manner.unwrap_or_default());
        codec.encode(&self.experience.status_points);
        codec.encode(&self.stats.hp);
        codec.encode(&self.stats.max_hp);
        codec.encode(&self.stats.sp);
        codec.encode(&self.stats.max_sp);
        codec.encode(&DEFAULT_WALK_SPEED);
        codec.encode(&(self.class as u16));
        codec.encode(&self.appearance.hair);
        codec.encode(&self.appearance.body);
        codec.encode(
            &if self.status.option & OPTIONS_INCOMPATIBLE_WITH_WEAPON == 0 {
                self.equipment.weapon
            } else {
                0
            },
        );
        codec.encode(&self.experience.base_level);
        codec.encode(&self.experience.skill_points);
        codec.encode(&self.equipment.head_bottom);
        codec.encode(&self.equipment.shield);
        codec.encode(&self.equipment.head_top);
        codec.encode(&self.equipment.head_mid);
        codec.encode(&self.appearance.hair_color);
        codec.encode(&self.appearance.clothes_color);
        codec.encode(&self.name);
        codec.encode(&self.stats.str);
        codec.encode(&self.stats.agi);
        codec.encode(&self.stats.vit);
        codec.encode(&self.stats.int);
        codec.encode(&self.stats.dex);
        codec.encode(&self.stats.luk);
        codec.encode(&self.slot);
        codec.encode(&if self.settings.rename > 0 { 0u16 } else { 1u16 });
        codec.encode(
            &codec
                .maps()
                .name(self.location.last_location.map_id)
                .expect("invalid map name"),
        );
        codec.encode(&self.status.delete_date.unwrap_or(SystemTime::UNIX_EPOCH));
        codec.encode(&(self.equipment.robe as u32));
        codec.encode(&(CHARACTER_SLOT_MOVE_ENABLED as u32));
        codec.encode(&(CHARACTER_RENAME_ENABLED as u32));
        codec.encode(&(self.sex as u8));
    }
}
