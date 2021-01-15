use crate::account::db::AccountId;
use crate::character::db::CharacterId;
use std::time::SystemTime;

#[repr(u16)]
#[derive(Debug, Clone, Copy, int_enum::IntEnum)]
pub enum Class {
    Novice = 0,
    Rogue = 1,
    Mage = 2,
    Archer = 3,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Stats {
    pub str: u8,
    pub agi: u8,
    pub vit: u8,
    pub int: u8,
    pub dex: u8,
    pub luk: u8,
    pub hp: u32,
    pub max_hp: u32,
    pub sp: u16,
    pub max_sp: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Experience {
    pub base_level: u16,
    pub job_level: u16,
    pub base_exp: u64,
    pub job_exp: u64,
    pub status_points: u16,
    pub skill_points: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Currency {
    pub zeny: u32,
    pub fame: i32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Status {
    pub option: u32,
    /// Minutes the character will be muted
    pub manner: Option<u32>,
    pub karma: Option<u32>,
    pub delete_date: Option<SystemTime>,
    pub unban_on: Option<SystemTime>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Appearance {
    pub hair: u16,
    pub hair_color: u16,
    pub clothes: u16,
    pub clothes_color: u16,
    pub body: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Grouping {
    pub party_id: i32,
    pub guild_id: i32,
    pub pet_id: i32,
    pub homunculus_id: i32,
    // Note: this name is likely not correct
    pub mermaid_id: i32,
    pub elemental_id: i32,
    pub clan_id: i32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MercenaryGuildRank {
    pub arch_faith: i32,
    pub arch_calls: i32,
    pub spear_faith: i32,
    pub spear_calls: i32,
    pub sword_faith: i32,
    pub sword_calls: i32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Equipment {
    pub weapon: u16,
    pub shield: u16,
    pub head_top: u16,
    pub head_mid: u16,
    pub head_bottom: u16,
    pub robe: u32,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, int_enum::IntEnum)]
pub enum Weapon {
    Fist = 0,
    Dagger = 1,
    OneHandedSword = 2,
    TwoHandedSword = 3,
    OneHandedSpear = 4,
    TwoHandedSpear = 5,
    OneHandedAxe = 6,
    TwoHandedAxe = 7,
    Mace = 8,
    TwoHandedMace = 9,
    Staff = 10,
    Bow = 11,
    Knuckle = 12,
    Musical = 13,
    Whip = 14,
    Book = 15,
    Katar = 16,
    Revolver = 17,
    Rifle = 18,
    Gatling = 19,
    Shotgun = 20,
    Grenade = 21,
    Huuma = 22,
    TwoHandedStaff = 23,
    // Dual wield
    DoubleDaggers = 25,
    DoubleSwords = 26,
    DoubleAxes = 27,
    DaggerAndSword = 28,
    DaggerAndAxe = 29,
    SwordAndAxe = 30,
}

#[derive(Clone, Debug, Default)]
pub struct Relationship {
    pub partner_id: u32,
    pub father: u32,
    pub mother: u32,
    pub child: u32,
    pub friends: Vec<Friend>,
}

#[derive(Clone, Debug)]
pub struct Friend {
    pub account_id: AccountId,
    pub char_id: CharacterId,
    pub name: String,
}

#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub map_server_id: usize,
    pub last_location: Point,
    pub save: Option<Point>,
    pub memo: Option<Point>,
}

/// Starting location for each character
impl Default for Location {
    fn default() -> Self {
        Self {
            map_server_id: 0,
            last_location: Point {
                map_id: 0,
                x: 0,
                y: 0,
            },
            save: None,
            memo: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub map_id: u16,
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct Skill {
    pub id: u16,
    pub level: u8,
    pub flag: SkillFlag,
}

#[derive(Clone, Copy, Debug)]
pub enum SkillFlag {
    Permanent,
    Temporary,
    Plagiarized,
    Granted,
    TemporaryCombo,
    ReplacedLevel0,
}

#[derive(Clone, Copy, Debug)]
pub struct Hotkey {
    pub id: u32,
    pub level: u16,
    pub type_of: HotkeyType,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, int_enum::IntEnum)]
pub enum HotkeyType {
    Item = 0,
    Skill = 1,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Settings {
    pub show_equip: bool,
    pub allow_party: bool,
    pub rename: u16,
    pub font_id: u8,
    pub cash_shop_sent: bool,
    pub unique_item_counter: u32,
    pub hotkey_row_shift: u8,
    pub hotkey_row_shift2: u8,
    pub title_id: u32,
}
