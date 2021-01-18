use crate::account::db::AccountId;
use crate::character::db::CharacterId;
use std::time::SystemTime;

#[repr(u16)]
#[derive(Debug, Clone, Copy, int_enum::IntEnum)]
pub enum Class {
    Novice = 0,
    Swordman = 1,
    Mage = 2,
    Archer = 3,
    Acolyte = 4,
    Merchant = 5,
    Thief = 6,
    Knight = 7,
    Priest = 8,
    Wizard = 9,
    Blacksmith = 10,
    Hunter = 11,
    Assassin = 12,
    Knight2 = 13,
    Crusader = 14,
    Monk = 15,
    Sage = 16,
    Rogue = 17,
    Alchemist = 18,
    Bard = 19,
    Dancer = 20,
    Crusader2 = 21,
    Wedding = 22,
    SuperNovice = 23,
    Gunslinger = 24,
    Ninja = 25,
    Xmas = 26,
    Summer = 27,
    Hanbok = 28,
    Oktoberfest = 29,
    Summer2 = 30,
    NoviceHigh = 4001,
    SwordmanHigh = 4002,
    MageHigh = 4003,
    ArcherHigh = 4004,
    AcolyteHigh = 4005,
    MerchantHigh = 4006,
    ThiefHigh = 4007,
    LordKnight = 4008,
    HighPriest = 4009,
    HighWizard = 4010,
    Whitesmith = 4011,
    Sniper = 4012,
    AssassinCross = 4013,
    LordKnight2 = 4014,
    Paladin = 4015,
    Champion = 4016,
    Professor = 4017,
    Stalker = 4018,
    Creator = 4019,
    Clown = 4020,
    Gypsy = 4021,
    Paladin2 = 4022,
    Baby = 4023,
    BabySwordman = 4024,
    BabyMage = 4025,
    BabyArcher = 4026,
    BabyAcolyte = 4027,
    BabyMerchant = 4028,
    BabyThief = 4029,
    BabyKnight = 4030,
    BabyPriest = 4031,
    BabyWizard = 4032,
    BabyBlacksmith = 4033,
    BabyHunter = 4034,
    BabyAssassin = 4035,
    BabyKnight2 = 4036,
    BabyCrusader = 4037,
    BabyMonk = 4038,
    BabySage = 4039,
    BabyRogue = 4040,
    BabyAlchemist = 4041,
    BabyBard = 4042,
    BabyDancer = 4043,
    BabyCrusader2 = 4044,
    SuperBaby = 4045,
    Taekwon = 4046,
    StarGladiator = 4047,
    StarGladiator2 = 4048,
    SoulLinker = 4049,
    Gangsi = 4050,
    DeathKnight = 4051,
    DarkCollector = 4052,
    RuneKnight = 4054,
    Warlock = 4055,
    Ranger = 4056,
    ArchBishop = 4057,
    Mechanic = 4058,
    GuillotineCross = 4059,
    RuneKnightT = 4060,
    WarlockT = 4061,
    RangerT = 4062,
    ArchBishopT = 4063,
    MechanicT = 4064,
    GuillotineCrossT = 4065,
    RoyalGuard = 4066,
    Sorcerer = 4067,
    Minstrel = 4068,
    Wanderer = 4069,
    Sura = 4070,
    Genetic = 4071,
    ShadowChaser = 4072,
    RoyalGuardT = 4073,
    SorcererT = 4074,
    MinstrelT = 4075,
    WandererT = 4076,
    SuraT = 4077,
    GeneticT = 4078,
    ShadowChaserT = 4079,
    RuneKnight2 = 4080,
    RuneKnightT2 = 4081,
    RoyalGuard2 = 4082,
    RoyalGuardT2 = 4083,
    Ranger2 = 4084,
    RangerT2 = 4085,
    Mechanic2 = 4086,
    MechanicT2 = 4087,
    BabyRuneKnight = 4096,
    BabyWarlock = 4097,
    BabyRanger = 4098,
    BabyArchBishop = 4099,
    BabyMechanic = 4100,
    BabyGuillotineCross = 4101,
    BabyRoyalGuard = 4102,
    BabySorcerer = 4103,
    BabyMinstrel = 4104,
    BabyWanderer = 4105,
    BabySura = 4106,
    BabyGenetic = 4107,
    BabyShadowChaser = 4108,
    BabyRuneKnight2 = 4109,
    BabyRoyalGuard2 = 4110,
    BabyRanger2 = 4111,
    BabyMechanic2 = 4112,
    SuperNoviceE = 4190,
    SuperBabyE = 4191,
    Kagerou = 4211,
    Oboro = 4212,
    Rebellion = 4215,
    Summoner = 4218,
    BabySummoner = 4220,
    BabyNinja = 4222,
    BabyKagerou = 4223,
    BabyOboro = 4224,
    BabyTaekwon = 4225,
    BabyStarGladiator = 4226,
    BabySoulLinker = 4227,
    BabyGunslinger = 4228,
    BabyRebellion = 4229,
    BabyStarGladiator2 = 4238,
    StarEmperor = 4239,
    SoulReaper = 4240,
    BabyStarEmperor = 4241,
    BabySoulReaper = 4242,
    StarEmperor2 = 4243,
    BabyStarEmperor2 = 4244,
}

#[derive(Debug, Clone, Copy)]
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

impl Default for Stats {
    fn default() -> Self {
        let hp = (40 * 101) / 100;
        let sp = (11 * 101) / 100;
        Self {
            str: 1,
            agi: 1,
            vit: 1,
            int: 1,
            dex: 1,
            luk: 1,
            hp,
            max_hp: hp,
            sp,
            max_sp: sp,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Experience {
    pub base_level: u16,
    pub job_level: u16,
    pub base_exp: u64,
    pub job_exp: u64,
    pub status_points: u16,
    pub skill_points: u16,
}

impl Default for Experience {
    fn default() -> Self {
        Self {
            base_level: 1,
            job_level: 1,
            base_exp: 0,
            job_exp: 0,
            status_points: 48,
            skill_points: 0,
        }
    }
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
